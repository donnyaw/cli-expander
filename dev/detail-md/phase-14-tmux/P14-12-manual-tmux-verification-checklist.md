# P14-12: Add Manual Tmux Verification Checklist

## Objective

Create a repeatable manual checklist for validating tmux behavior across panes, popup picker, selected-pane targeting, multiline insertion, and explicit execution.

## Implementation Status

Completed on branch `feature/tmux-integration` with tag `p14-12`. The full checklist is available at `integrations/tmux/README.md`.

## Why This Exists

Some tmux behaviors are hard to fully test in automated CI. Pane focus, popup lifecycle, and terminal UI behavior require manual verification.

## Scope

- Add checklist to `integrations/tmux/README.md` or a dedicated verification markdown.
- Cover direct tmux output mode.
- Cover popup picker mode.
- Cover generic selected-pane workflow.
- Cover failure paths.

## Checklist Content

```text
1. Build ce and ensure it is in PATH.
2. Open tmux with two panes.
3. Focus pane A.
4. Run direct tmux injection targeting pane A.
5. Confirm pane A receives text and pane B does not.
6. Focus pane B and repeat.
7. Open popup picker from pane A.
8. Select a trigger and confirm pane A receives text after popup exits.
9. Cancel popup picker and confirm no text is inserted.
10. Test a multiline trigger.
11. Test text containing quotes, pipes, dollar signs, and semicolons.
12. Test --enter and confirm it only executes when set.
13. Test missing fzf error.
14. Test missing ce error.
15. Test outside tmux error or fallback behavior.
16. Test generic target panes: shell, editor insert mode, OpenCode/TUI input area.
```

## Acceptance Criteria

- Checklist is committed with tmux docs.
- Each step includes expected result.
- Failure cases include troubleshooting hints.
- Checklist references relevant commands and tmux bindings.

## Dependencies

- `P14-05`.
- `P14-06`.
- `P14-07`.

## Follow-Up

When app-aware behavior is added later, extend the checklist with per-app sections.
