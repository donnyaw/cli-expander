# terminal-expander

A terminal-native text expander with Espanso-compatible forms. Written in Rust.

Type a short trigger like `:hello` and it expands to a full sentence. Need user input? Forms render directly in your terminal — no GUI popups.

## Features

- **Trigger expansion** — `:hello` → `Hello World!`
- **Interactive forms** — Text fields, multiline, choice dropdowns, lists — all in-terminal via Cursive TUI
- **Espanso-compatible** — Use your existing Espanso `match/*.yml` files directly
- **Variable system** — Date, clipboard, shell command output, and form field injection
- **Shell plugins** — Expansion hooks for Zsh, Bash, and Fish
- **Multiple injection modes** — uinput, ydotool, tmux send-keys, clipboard
- **Privacy-first** — 100% local, no data leaves your machine

## Quick Start

```bash
# Install
cargo install --path .

# Create your first match file
mkdir -p ~/.config/texpand/matches
cat > ~/.config/texpand/matches/base.yml << 'EOF'
matches:
  - trigger: ":hello"
    replace: "Hello World!"
  - trigger: ":now"
    replace: "{{time}}"
    vars:
      - name: time
        type: date
        params:
          format: "%H:%M"
EOF

# Try it
texpand expand ":hello"
# Output: Hello World!

texpand expand ":now"
# Output: 14:30
```

## Commands

| Command | Description |
|---------|-------------|
| `texpand expand <input>` | Find trigger and print expansion |
| `texpand list` | List all available triggers |
| `texpand form <layout>` | Open interactive form in terminal |
| `texpand config` | Show configuration info |

## Example Match Files

```yaml
# Simple replacement
- trigger: ":thanks"
  replace: "Thank you for your help!"

# Multi-line replacement
- trigger: ":sig"
  replace: |
    Best regards,
    John Doe
    john@example.com

# Date variable
- trigger: ":today"
  replace: "{{date}}"
  vars:
    - name: date
      type: date
      params:
        format: "%B %d, %Y"

# Form with text field
- trigger: ":greet"
  form: |
    Hello [[name]],
    Welcome to the team!
  form_fields:
    name:
      placeholder: "Enter name"

# Form with choice dropdown
- trigger: ":ticket"
  form: "Priority: [[level]]"
  form_fields:
    level:
      type: choice
      values:
        - Low
        - Medium
        - High
        - Critical
```

## Installation

### From source

```bash
git clone https://github.com/donnyaw/terminal-expander.git
cd terminal-expander
cargo build --release
cp target/release/texpand ~/.local/bin/
```

### Requirements

- Rust 1.70+
- Linux (Wayland, X11, or TTY)
- For system-wide mode: `input` and `uinput` group membership

## Shell Integration

Source one of the shell plugins for in-prompt expansion:

```bash
# Zsh
echo 'source /path/to/shell/texpand.zsh' >> ~/.zshrc

# Bash
echo 'source /path/to/shell/texpand.bash' >> ~/.bashrc

# Fish
echo 'source /path/to/shell/texpand.fish' >> ~/.config/fish/config.fish
```

Then type `:hello` in your shell prompt and press `Ctrl+T` to expand.

## Documentation

- [Installation Guide](guides/installation.md)
- [User Guide](guides/user-guide.md)
- [Configuration Reference](reference/configuration.md)
- [Form Syntax Reference](reference/forms.md)
- [Shell Integration](guides/shell-integration.md)
- [Variable System](reference/variables.md)
- [Troubleshooting](guides/troubleshooting.md)

## License

MIT
