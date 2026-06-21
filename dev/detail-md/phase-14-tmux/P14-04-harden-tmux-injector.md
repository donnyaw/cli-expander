# P14-04: Harden Tmux Injector For Literal Safe Insertion

## Objective

Make tmux injection robust, predictable, and safe for literal command insertion.

## Why This Exists

Tmux injection is powerful because it can send text to any pane. It must avoid accidental execution, shell interpretation surprises, and unclear failures.

## Scope

- Preserve literal text through `tmux send-keys -l`.
- Improve tmux error messages.
- Define behavior for multiline output.
- Keep Enter disabled by default.
- Decide how `$|$` cursor marker is handled in tmux mode.
- Make command construction testable without a live tmux server.

## Implementation Steps

1. Keep using `send-keys -l` for literal insertion.

2. Capture stderr from tmux and include it in errors:

```text
tmux send-keys failed for target %1: can't find pane
```

3. If `tmux` binary is missing, return a direct actionable error.

4. If `--output tmux` is used outside tmux without target, return a clear error or let tmux report failure. Prefer clear local validation.

5. Decide cursor marker behavior:

- MVP: strip `$|$` before tmux injection and document that cursor placement is shell-plugin-only.
- Later: support cursor movement with tmux key events if needed.

6. Define the MVP multiline rule before implementation:

- Preferred MVP: reject multiline tmux injection with a clear error until a verified paste-buffer strategy exists.
- Acceptable alternative: implement multiline with `tmux load-buffer` plus `tmux paste-buffer`, with manual verification that it does not execute commands unexpectedly.
- Do not silently convert newlines to Enter keypresses.

7. Add pure helpers for tmux argument construction so tests can assert exact commands.

## Acceptance Criteria

- Literal shell characters are inserted and not interpreted by the invoking shell.
- Errors include enough detail to diagnose missing tmux, invalid target pane, and tmux command failure.
- No command executes unless later `--enter` is explicitly used.
- `$|$` behavior is defined and consistent.
- Multiline behavior is explicitly implemented or explicitly rejected with a useful error.

## Test Plan

- Test command construction with text containing quotes, pipes, dollar signs, semicolons, and braces.
- Manual-test invalid target pane.
- Manual-test text with spaces and shell metacharacters.
- Manual-test multiline behavior according to the chosen MVP rule.

## Dependencies

- `P14-03`.

## Follow-Up

`P14-09` adds explicit Enter execution after safe insertion is stable.
