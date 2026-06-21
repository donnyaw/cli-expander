# P14-07: Preserve Original Pane Across Popup Execution

## Objective

Guarantee that popup picker actions inject into the pane that was selected before the popup opened.

## Why This Exists

Tmux popups change interaction context. If a script uses the implicit current pane, injection can target the popup or the wrong pane. The user goal requires targeting the originally selected pane.

## Scope

- Use tmux format `#{pane_id}` in bindings.
- Pass the pane id into picker scripts.
- Require picker scripts to use `--target-pane`.
- Add documentation and verification steps.

## Correct Binding Pattern

```tmux
bind-key C-e display-popup -E "ce-tmux-picker '#{pane_id}'"
```

## Incorrect Pattern

```tmux
bind-key C-e display-popup -E "ce-tmux-picker"
```

The incorrect pattern forces the script to guess the target pane.

## Implementation Steps

1. Update `integrations/tmux/cli-expander.tmux` to pass `#{pane_id}`.

2. Update `ce-tmux-picker.sh` to require or strongly prefer a pane argument.

3. Add validation:

```bash
if [ -z "$target_pane" ]; then
  printf 'ce-tmux-picker: missing target pane id\n' >&2
  exit 2
fi
```

4. Use the target pane for all injection commands.

5. Document why this is required.

## Acceptance Criteria

- Popup selection always injects into the pane that launched the popup.
- Missing pane id fails with a clear error.
- Docs explain original-pane preservation.

## Test Plan

- Open two panes.
- Launch picker from pane A.
- While popup is open, confirm selected trigger inserts into pane A.
- Repeat from pane B.
- Test missing argument behavior by running the script directly.

## Dependencies

- `P14-06`.

## Follow-Up

`P14-08` verifies this behavior across generic selected-pane contexts.
