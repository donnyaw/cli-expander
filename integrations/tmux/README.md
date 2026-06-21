# cli-expander tmux Integration

This integration is for tmux users who want to insert `cli-expander` output into the currently selected pane or a specific target pane.

## Current Status

Implemented:

- `ce expand <trigger> --output tmux`
- `ce expand <trigger> --output tmux --target-pane <pane-id>`
- `ce expand <trigger> --output auto`
- `ce expand <trigger> --output clipboard`

Planned next:

- `ce-tmux-picker.sh` popup picker with `fzf`
- Original-pane preservation for popup workflows
- Tmux-safe form execution from popup

## Direct Usage

Insert an expansion into the current tmux pane:

```bash
ce expand ":hello" --output tmux
```

Insert into a specific tmux pane:

```bash
ce expand ":hello" --output tmux --target-pane "%1"
```

Use the current pane id from inside tmux:

```bash
ce expand ":hello" --output tmux --target-pane "$TMUX_PANE"
```

## Recommended `.tmux.conf` Setup

Source the integration file from your tmux config:

```tmux
run-shell /path/to/cli-expander/integrations/tmux/cli-expander.tmux
```

The popup picker binding is included but remains disabled until `P14-06` implements `ce-tmux-picker.sh`.

## Selected-Pane Rule

Popup workflows must preserve the original pane id. Once a popup opens, the popup is the active UI context, so scripts must receive the original `#{pane_id}` and pass it back to `ce expand --target-pane`.

Correct pattern:

```tmux
bind-key C-e display-popup -E "ce-tmux-picker '#{pane_id}'"
```

## Safety Rules

- Tmux output inserts text only. It does not press Enter.
- Multiline tmux injection is currently rejected until paste-buffer support is implemented.
- The `$|$` cursor marker is stripped in tmux mode because cursor positioning is currently shell-plugin-only.
- Use `--target-pane` for popup and script workflows.

## Troubleshooting

| Problem | Fix |
|---|---|
| `tmux send-keys` fails | Confirm you are inside tmux or pass a valid target pane |
| Text goes to the wrong pane | Pass the original pane id with `--target-pane` |
| Multiline expansion fails | Use a single-line trigger until paste-buffer support lands |
| Picker script says not implemented | Complete `P14-06` |
