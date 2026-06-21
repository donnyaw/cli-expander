# cli-expander tmux Integration

This integration is for tmux users who want to insert `cli-expander` output into the currently selected pane or a specific target pane.

## Current Status

Implemented:

- `ce expand <trigger> --output tmux`
- `ce expand <trigger> --output tmux --target-pane <pane-id>`
- `ce expand <trigger> --output auto`
- `ce expand <trigger> --output clipboard`

Implemented in this directory:

- `ce-tmux-picker.sh` popup picker with `fzf`
- JSON-derived TSV trigger rows to avoid fragile CSV comma parsing

Planned next:

- Additional original-pane preservation hardening for popup workflows
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

The popup picker binding calls `ce-tmux-picker.sh`. Ensure `integrations/tmux/` is on your `PATH`, or copy the script to a directory already on `PATH`.

## Selected-Pane Rule

Popup workflows must preserve the original pane id. Once a popup opens, the popup is the active UI context, so scripts must receive the original `#{pane_id}` and pass it back to `ce expand --target-pane`.

Correct pattern:

```tmux
bind-key C-e display-popup -E "ce-tmux-picker '#{pane_id}'"
```

`ce-tmux-picker.sh` validates this pane id before opening the picker. If the pane id is missing or no longer exists, it exits without injecting anything.

## Generic Pane Workflow

The MVP intentionally treats every selected pane the same. It sends literal text to the pane you target and does not inspect whether that pane is running Bash, Zsh, Fish, Vim, OpenCode, or another TUI.

This means the target application must already be ready to receive typed text. For example:

- Shell prompts receive the inserted command text.
- Editors receive text only when they are in an input mode that accepts typing.
- OpenCode or other TUIs receive text only when their input area is focused.

App-specific behavior is intentionally deferred until the generic selected-pane workflow is stable.

## Form Triggers In Popup

The picker calls the same command for both text and form triggers:

```bash
ce expand "$trigger" --output tmux --target-pane "$target_pane"
```

If the selected trigger opens a form, the form runs in the popup process. After the form is submitted, only the completed expansion is injected into the original target pane. Canceling the form exits without injection.

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
| Picker says target pane is invalid | Reopen the picker from a live pane and pass `#{pane_id}` |
| Text appears in the wrong app area | Focus the intended input area before opening the picker |
| Form opens in the popup | Expected behavior; submit it to inject the completed result into the target pane |
| Multiline expansion fails | Use a single-line trigger until paste-buffer support lands |
| Picker cannot find `ce-tmux-picker.sh` | Put `integrations/tmux/` on `PATH` or copy the script to `~/.local/bin` |
| Picker cannot find `fzf` | Install `fzf` |
