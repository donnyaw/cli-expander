#[derive(Debug)]
pub enum InjectionMethod {
    Uinput,
    Clipboard,
    TmuxSendKeys,
    Stdout,
}

pub trait Injector: Send {
    fn inject(&self, text: &str) -> anyhow::Result<()>;
    fn method(&self) -> InjectionMethod;
}

pub struct UinputInjector;

impl Injector for UinputInjector {
    fn inject(&self, text: &str) -> anyhow::Result<()> {
        match std::env::var("TMUX") {
            Ok(_) => {
                let output = std::process::Command::new("tmux")
                    .args(["send-keys", "-l", text])
                    .output()
                    .map_err(|e| anyhow::anyhow!("tmux send-keys failed: {}", e))?;
                if !output.status.success() {
                    anyhow::bail!("tmux send-keys exited with error");
                }
                Ok(())
            }
            Err(_) => {
                // Fallback: use ydotool if available
                if which("ydotool") {
                    let output = std::process::Command::new("ydotool")
                        .args(["type", text])
                        .output()
                        .map_err(|e| anyhow::anyhow!("ydotool failed: {}", e))?;
                    if !output.status.success() {
                        anyhow::bail!("ydotool exited with error");
                    }
                    return Ok(());
                }
                anyhow::bail!("No injection method available. Install ydotool or run inside tmux.");
            }
        }
    }

    fn method(&self) -> InjectionMethod {
        InjectionMethod::Uinput
    }
}

pub struct TmuxInjector;

impl Injector for TmuxInjector {
    fn inject(&self, text: &str) -> anyhow::Result<()> {
        let output = std::process::Command::new("tmux")
            .args(["send-keys", "-l", text])
            .output()
            .map_err(|e| anyhow::anyhow!("tmux send-keys failed: {}", e))?;
        if !output.status.success() {
            anyhow::bail!("tmux send-keys exited with error");
        }
        Ok(())
    }

    fn method(&self) -> InjectionMethod {
        InjectionMethod::TmuxSendKeys
    }
}

pub struct ClipboardInjector;

impl Injector for ClipboardInjector {
    fn inject(&self, text: &str) -> anyhow::Result<()> {
        let mut clipboard = arboard::Clipboard::new()
            .map_err(|e| anyhow::anyhow!("Failed to open clipboard: {}", e))?;
        clipboard.set_text(text)
            .map_err(|e| anyhow::anyhow!("Failed to set clipboard text: {}", e))?;
        Ok(())
    }

    fn method(&self) -> InjectionMethod {
        InjectionMethod::Clipboard
    }
}

fn which(name: &str) -> bool {
    std::process::Command::new("which")
        .arg(name)
        .output()
        .is_ok_and(|o| o.status.success())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_inject() {
        let injector = ClipboardInjector;
        let result = injector.inject("test text");
        // May fail if no display server, but shouldn't crash
        if let Err(e) = result {
            assert!(e.to_string().contains("clipboard"));
        }
    }

    #[test]
    fn test_injection_method_display() {
        assert_eq!(format!("{:?}", InjectionMethod::Uinput), "Uinput");
        assert_eq!(format!("{:?}", InjectionMethod::Clipboard), "Clipboard");
    }
}
