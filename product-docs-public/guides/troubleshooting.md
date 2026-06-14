# Troubleshooting

## Common Issues

### `texpand: command not found`

The binary is not in your PATH.

```bash
# Find the binary
find ~/.local/bin -name texpand
find /usr/local/bin -name texpand

# Or add to PATH
export PATH="$PATH:$HOME/.local/bin"
```

### `No match found for trigger`

The match file doesn't contain the trigger, or the match directory is not configured.

```bash
# Check your match files
ls ~/.config/texpand/matches/
cat ~/.config/texpand/matches/base.yml

# List available triggers
texpand list
```

### `Form requires an interactive terminal`

The form command needs a real terminal. This happens when:
- Running in a piped context (`echo ":form" | texpand expand`)
- Running over SSH without a terminal (`ssh host texpand expand ":form"`)
- Running in headless/CI environments

**Fix:** Run in a terminal emulator directly.

### `No keyboard devices found`

System-wide detection requires access to `/dev/input/` devices.

```bash
# Check group membership
groups $USER

# Add to input group if missing
sudo usermod -aG input $USER
# Log out and back in
```

### `Permission denied` for uinput

uinput injection needs write access to `/dev/uinput`.

```bash
# Check permissions
ls -l /dev/uinput

# Add to uinput group
sudo usermod -aG uinput $USER

# Or set capability on the binary
sudo setcap cap_dac_override+ep $(which texpand)
```

## Debugging

### Enable logging

```bash
RUST_LOG=debug texpand expand ":hello"
```

### Test match file parsing

```bash
# Parse a specific file
cat ~/.config/texpand/matches/base.yml | texpand list --config-dir ~/.config/texpand/matches/
```

### Check binary version

```bash
texpand --version
```

## Known Limitations

- **Forms require a terminal** — Cursive TUI does not work in non-interactive sessions
- **Linux only** — evdev and uinput are Linux-specific
- **Wayland clipboard** — requires `wl-clipboard` package for clipboard variable
- **Single keyboard layout** — Full Unicode support may have gaps with non-Latin layouts

## Getting Help

- GitHub Issues: https://github.com/donnyaw/terminal-expander/issues
- Repository: https://github.com/donnyaw/terminal-expander
