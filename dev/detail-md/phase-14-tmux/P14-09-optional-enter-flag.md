# P14-09: Add Optional --enter Execution Flag

## Objective

Add an explicit flag that sends Enter after tmux insertion for trusted workflows.

## Implementation Status

Completed on branch `feature/tmux-integration` with tag `p14-09`.

## Why This Exists

The safe default is insert-only. Some users may want a selected trigger to run immediately. That behavior must be explicit because many expanded commands should be inspected before execution.

## Scope

- Add `--enter` to `ce expand`.
- Only apply it to tmux output mode initially.
- Keep default behavior insert-only.
- Document safety implications.

## Proposed CLI

```bash
ce expand ":docker-ps" --output tmux --target-pane "%1" --enter
```

## Implementation Steps

1. Add `enter: bool` to `Commands::Expand`.

2. Output dispatch passes the flag to tmux injection.

3. Tmux injection sends literal text first.

4. If `enter` is true, send Enter as a separate tmux command:

```bash
tmux send-keys -t "$target" Enter
```

5. If `--enter` is used with stdout or clipboard, either ignore it with no effect or return a clear validation error. Prefer validation error to avoid confusion.

## Acceptance Criteria

- `--enter` is opt-in.
- Without `--enter`, text is inserted only.
- With `--enter`, tmux receives text then Enter.
- Docs warn that `--enter` can execute commands.

## Test Plan

- Unit-test command sequence construction.
- Manual-test insert-only behavior.
- Manual-test explicit execution behavior.
- Manual-test `--enter` with unsupported output mode.

## Dependencies

- `P14-04`.

## Follow-Up

Tmux config snippets may offer separate keybindings for insert-only and insert-and-run.
