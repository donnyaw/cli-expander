# cli-expander tmux Integration

This integration is for tmux users who want to insert `cli-expander` output into the currently selected pane or a specific target pane.

## Current Status

The tmux integration is implemented and verified for direct pane injection, target-pane injection, auto mode, explicit Enter, popup trigger selection, and form-trigger expansion into the original pane.

Implemented:

- `ce expand <trigger> --output tmux`
- `ce expand <trigger> --output tmux --target-pane <pane-id>`
- `ce expand <trigger> --output auto`
- `ce expand <trigger> --output clipboard`
- `ce expand <trigger> --output tmux --enter`

Implemented in this directory:

- `ce-tmux-picker.sh` popup picker with `fzf`
- JSON-derived TSV trigger rows to avoid fragile CSV comma parsing
- Original-pane preservation for popup workflows
- Automatic popup close after successful injection
- Guardrails that reject unresolved `{{variable}}` placeholders before injection

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

Then restart tmux or reload config with `prefix + :` then `source-file ~/.tmux.conf`.

### Available Keybindings

| Binding | Action |
|---------|--------|
| `prefix + e` | Prompt to type a trigger name (e.g. `:hello`) and expand it inline in the current pane |
| `prefix + Ctrl+g` | Open fzf popup picker to browse and select a trigger |

The popup picker binding calls `ce-tmux-picker.sh`. Ensure `integrations/tmux/` is on your `PATH`, or copy the script to a directory already on `PATH` such as `~/.local/bin`.

### Workflow: Inline Expansion (prefix + e)

Press `prefix + e` (your prefix is `Ctrl+Space` by default in this config). A prompt appears at the bottom of the tmux window:

```
cli-expander:
```

Type a trigger like `:hello` and press Enter. The expanded text is injected into your current pane immediately.

This works in any pane — shell prompt, editor, TUI — without needing the shell plugin. It's the "prefix + :" equivalent for cli-expander.

## Selected-Pane Rule

Popup workflows must preserve the original pane id. Once a popup opens, the popup is the active UI context, so scripts must receive the original `#{pane_id}` and pass it back to `ce expand --target-pane`.

Correct pattern:

```tmux
bind-key C-g run-shell 'tmux display-popup -E -T cli-expander -w 90% -h 80% "ce-tmux-picker.sh \"#{pane_id}\""'
```

The `run-shell` wrapper lets tmux expand `#{pane_id}` before `display-popup -E` starts the popup command. `ce-tmux-picker.sh` validates this pane id before opening the picker. If the pane id is missing, unexpanded, or no longer exists, it exits without injecting anything.

## Generic Pane Workflow

The integration intentionally treats every selected pane the same. It sends literal text to the pane you target and does not inspect whether that pane is running Bash, Zsh, Fish, Vim, OpenCode, or another TUI.

This means the target application must already be ready to receive typed text. For example:

- Shell prompts receive the inserted command text.
- Editors receive text only when they are in an input mode that accepts typing.
- OpenCode or other TUIs receive text only when their input area is focused.

App-specific behavior is intentionally left to the target application and its focused input area.

## Form Triggers In Popup

The picker calls the same command for both text and form triggers:

```bash
ce expand "$trigger" --output tmux --target-pane "$target_pane"
```

If the selected trigger opens a form, the form runs in the popup process. After the form is submitted, only the completed expansion is injected into the original target pane, and the popup closes automatically. Canceling the form exits without injection.

Form defaults are merged for any missing field values, and cli-expander rejects unresolved `{{variable}}` placeholders before injecting text. This prevents incomplete commands such as `fd {{predicate}} {{path}} -X mv -t {{dest}}` from being inserted silently.

## Safety Rules

- Tmux output inserts text only. It does not press Enter.
- Multiline tmux injection is currently rejected until paste-buffer support is implemented.
- The `$|$` cursor marker is stripped in tmux mode because cursor positioning is currently shell-plugin-only.
- Use `--target-pane` for popup and script workflows.

## Manual Verification Checklist

Run through these steps to validate the tmux integration works correctly after building from source.

### Prerequisites

```bash
cargo build --release
cp target/release/ce ~/.local/bin/
mkdir -p ~/.config/cli-expander/matches

cat > ~/.config/cli-expander/matches/base.yml << 'EOF'
matches:
  - trigger: ":hello"
    replace: "Hello World!"
  - trigger: ":date"
    replace: "{{now}}"
    vars:
      - name: now
        type: date
        params:
          format: "%Y-%m-%d"
EOF

ce generate-csv --force
```

### Step 1: Direct Tmux Injection

1. Open a tmux session.
2. Run: `ce expand ":hello" --output tmux`
3. Expected: "Hello World!" appears in the current tmux prompt.
4. Run: `ce expand ":date" --output tmux`
5. Expected: today's date appears in the prompt.

### Step 2: Target Pane Injection

1. Open tmux with two panes (split horizontally with `C-b "`).
2. Note pane A id: `tmux display-message -p '#{pane_id}'`
3. Focus pane B.
4. Run: `ce expand ":hello" --output tmux --target-pane <pane-A-id>`
5. Expected: "Hello World!" appears in pane A, not pane B.
6. Focus pane A and repeat targeting pane B.

### Step 3: Auto Mode

1. Inside tmux: `ce expand ":hello" --output auto`
2. Expected: Text is injected into current pane (behaves like `--output tmux`).
3. Outside tmux: `ce expand ":hello" --output auto`
4. Expected: Text is printed to stdout.

### Step 4: Clipboard Mode

1. Run: `ce expand ":hello" --output clipboard`
2. Expected: Text is copied to clipboard (may fail if no clipboard provider is available).

### Step 5: Popup Picker

1. Ensure `integrations/tmux/ce-tmux-picker.sh` is on `PATH` or copy to `~/.local/bin/`.
2. Open a tmux session with two panes.
3. Run picker directly with a pane id:
   `integrations/tmux/ce-tmux-picker.sh "$(tmux display-message -p '#{pane_id}')"`
4. Expected: fzf picker opens with a trigger list.
5. Select `:hello` and press Enter.
6. Expected: "Hello World!" is injected into the original pane and the popup closes.
7. Reopen the picker and press Esc.
8. Expected: No text is injected (picker cancelled cleanly).

### Step 6: Form Trigger In Popup

1. Create a form trigger:
   ```bash
   cat > ~/.config/cli-expander/matches/form-test.yml << 'EOF'
   matches:
     - trigger: ":greet"
       form: "Say hello to [[name]]!"
       form_fields:
         name:
           placeholder: "Enter a name"
   EOF
   ce generate-csv --force
   ```
2. Run picker and select `:greet`.
3. Expected: Form renders in the popup. Fill in a name and submit.
4. Expected: The completed expansion ("Say hello to <name>!") is injected into the original pane.
5. Reopen picker, select `:greet`, and cancel the form.
6. Expected: Nothing is injected into the pane.

### Step 6b: Form Template Resolution

1. Run picker and select a form trigger with multiple template variables, such as `:fd-move` if that trigger pack is installed.
2. Submit the form with defaults or custom values.
3. Expected: The injected output contains concrete values only.
4. Expected: No unresolved placeholders such as `{{path}}`, `{{predicate}}`, or `{{dest}}` appear in the target pane.

### Step 7: --enter Flag

1. Run: `ce expand ":hello" --output tmux --enter`
2. Expected: "Hello World!" appears followed by an Enter keypress (command executes).

### Step 8: Error Handling

1. Outside tmux, run: `ce expand ":hello" --output tmux`
2. Expected: Clear error message about tmux not available.
3. Run: `ce expand ":hello" --output tmux --target-pane "nonexistent"`
4. Expected: Error about invalid target pane (or tmux error).
5. Run: `ce expand ":missing-trigger" --output tmux`
6. Expected: Error about no match found.
7. Run: `ce expand ":hello" --output stdout --enter`
8. Expected: Error that `--enter` is only supported with tmux output.

### Step 9: Generic Pane Targets

1. Open a text editor in one tmux pane (Vim with insert mode).
2. Run picker from the other pane, targeting the editor pane.
3. Expected: The expanded text appears as typed input in the editor.
4. Open a shell prompt in another pane and repeat.
5. Expected: Text appears in the shell prompt.

### Step 10: Multiline Rejection

1. Create a multiline trigger:
   ```bash
   cat > ~/.config/cli-expander/matches/multiline-test.yml << 'EOF'
   matches:
     - trigger: ":multi"
       replace: |
         line one
         line two
   EOF
   ce generate-csv --force
   ```
2. Run: `ce expand ":multi" --output tmux`
3. Expected: Error message about multiline injection not being supported yet.

## Troubleshooting

| Problem | Fix |
|---|---|
| `tmux send-keys` fails | Confirm you are inside tmux or pass a valid target pane |
| Text goes to the wrong pane | Pass the original pane id with `--target-pane` |
| Picker says target pane is invalid | Reopen the picker from a live pane and pass `#{pane_id}` |
| Text appears in the wrong app area | Focus the intended input area before opening the picker |
| Form opens in the popup | Expected behavior; submit it to inject the completed result into the target pane |
| Popup stays open after injection | Reload the integration file; the current binding uses `display-popup -E` so successful picker runs close automatically |
| Output contains `{{variable}}` placeholders | Update to the current binary; unresolved template variables are rejected before injection |
| Multiline expansion fails | Use a single-line trigger until paste-buffer support lands |
| Picker cannot find `ce-tmux-picker.sh` | Put `integrations/tmux/` on `PATH` or copy the script to `~/.local/bin` |
| Picker cannot find `fzf` | Install `fzf` |

## Known Issues

1. **Injection into full-screen TUI apps**: Sending keystrokes via `tmux send-keys` into a pane running a TUI application may not produce visible results depending on the application's input state. The keystrokes are delivered but the app may buffer or ignore them.

2. **No multiline support**: Multiline tmux injection is explicitly rejected. A paste-buffer strategy (`tmux load-buffer` + `paste-buffer`) is planned.

3. **Cursive forms depend on terminal state**: Form triggers use a terminal UI renderer. If a popup or terminal cannot host that UI, cli-expander falls back to defaults where possible and rejects unresolved template output.
