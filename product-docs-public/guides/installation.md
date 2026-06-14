# Installation Guide

## Prerequisites

- **Rust toolchain** (1.70 or later)
- **Linux** (tested on Ubuntu, Fedora, Arch)
- **Terminal emulator** (Kitty, Alacritty, GNOME Terminal, xterm, etc.)

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Build from Source

```bash
git clone https://github.com/donnyaw/terminal-expander.git
cd terminal-expander
cargo build --release
```

The binary will be at `target/release/texpand`.

### Install to system

```bash
cp target/release/texpand ~/.local/bin/
```

Or for system-wide install:

```bash
sudo cp target/release/texpand /usr/local/bin/
```

## Post-Installation Setup

### 1. Create config directory

```bash
mkdir -p ~/.config/texpand/matches
```

### 2. Add match files

Create `~/.config/texpand/matches/base.yml`:

```yaml
matches:
  - trigger: ":hello"
    replace: "Hello World!"
```

### 3. (Optional) Shell integration

Add to your shell config:

```bash
# ~/.bashrc or ~/.zshrc
source /path/to/shell/texpand.bash
```

### 4. (Optional) System-wide mode

For system-wide keyboard detection and injection, add your user to the required groups:

```bash
sudo usermod -aG input $USER
sudo usermod -aG uinput $USER
```

Then log out and back in.

## Verify Installation

```bash
texpand --version
texpand expand ":hello"
# Should print: Hello World!
```

## Uninstall

```bash
rm ~/.local/bin/texpand
rm -rf ~/.config/texpand
```

And remove the shell integration lines from your shell config file.
