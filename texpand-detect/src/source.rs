use crate::InputEvent;

#[derive(Debug, Clone, Default)]
pub struct SourceConfig {
    pub device_glob: Option<String>,
}

pub trait KeySource: Send {
    fn initialize(&mut self) -> anyhow::Result<()>;
    fn read_event(&mut self) -> anyhow::Result<Option<InputEvent>>;
}

pub struct EvdevKeySource {
    devices: Vec<evdev::Device>,
    grab: bool,
}

impl EvdevKeySource {
    pub fn new(grab: bool) -> Self {
        Self {
            devices: Vec::new(),
            grab,
        }
    }

    fn find_keyboards() -> Vec<String> {
        let mut paths = Vec::new();
        if let Ok(entries) = std::fs::read_dir("/dev/input/by-path") {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("");
                if name.contains("-kbd") || name.contains("-event-kbd") {
                    if let Ok(target) = std::fs::read_link(&path) {
                        let full = if target.is_absolute() {
                            target
                        } else {
                            std::path::PathBuf::from("/dev/input")
                                .join(target)
                        };
                        paths.push(full.to_string_lossy().to_string());
                    }
                }
            }
        }
        if paths.is_empty() {
            for i in 0..32 {
                let path = format!("/dev/input/event{}", i);
                if std::path::Path::new(&path).exists() {
                    paths.push(path);
                }
            }
        }
        paths
    }
}

impl KeySource for EvdevKeySource {
    fn initialize(&mut self) -> anyhow::Result<()> {
        let paths = Self::find_keyboards();
        if paths.is_empty() {
            anyhow::bail!("No keyboard devices found in /dev/input");
        }

        for path in &paths {
            match evdev::Device::open(path) {
                Ok(mut device) => {
                    let name = device.name().unwrap_or("unknown").to_string();
                    log::info!("Opened input device: {} ({})", name, path);
                    if self.grab {
                        let _ = device.grab();
                    }
                    self.devices.push(device);
                }
                Err(e) => {
                    log::warn!("Failed to open {}: {}", path, e);
                }
            }
        }

        if self.devices.is_empty() {
            anyhow::bail!("Could not open any keyboard devices. Try running with 'input' group membership.");
        }

        Ok(())
    }

    fn read_event(&mut self) -> anyhow::Result<Option<InputEvent>> {
        for device in &mut self.devices {
            if let Ok(events) = device.fetch_events() {
                for ev in events {
                    if ev.event_type() != evdev::EventType::KEY {
                        continue;
                    }
                    let code = ev.code();
                    let value = ev.value();

                    match value {
                        1 => {
                            return Ok(Some(InputEvent::KeyPress {
                                code,
                                key: format!("KEY_{}", code),
                            }));
                        }
                        0 => {
                            return Ok(Some(InputEvent::KeyRelease {
                                code,
                                key: format!("KEY_{}", code),
                            }));
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_config_default() {
        let config = SourceConfig::default();
        assert!(config.device_glob.is_none());
    }

    #[test]
    fn test_input_event_debug() {
        let event = InputEvent::KeyPress {
            code: 42,
            key: "KEY_A".to_string(),
        };
        assert_eq!(format!("{:?}", event), "KeyPress { code: 42, key: \"KEY_A\" }");
    }
}
