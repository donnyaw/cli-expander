# Phase 14 Tmux Working Plans

This directory contains one implementation-ready working plan for each Phase 14 task in `dev/cli-expander-dev-plan.csv`.

## Task Files

| Task ID | File |
|---|---|
| P14-01 | `P14-01-refactor-expansion-reusable-function.md` |
| P14-02 | `P14-02-cli-output-modes.md` |
| P14-03 | `P14-03-tmux-target-pane-support.md` |
| P14-04 | `P14-04-harden-tmux-injector.md` |
| P14-05 | `P14-05-tmux-integration-docs.md` |
| P14-06 | `P14-06-tmux-fzf-popup-picker.md` |
| P14-07 | `P14-07-preserve-original-pane.md` |
| P14-08 | `P14-08-generic-selected-pane-workflow.md` |
| P14-09 | `P14-09-optional-enter-flag.md` |
| P14-10 | `P14-10-tmux-safe-form-execution.md` |
| P14-11 | `P14-11-output-mode-tests.md` |
| P14-12 | `P14-12-manual-tmux-verification-checklist.md` |

## Implementation Order

1. Build the core output path: `P14-01` through `P14-04`.
2. Add automated coverage early: `P14-11` can begin after `P14-02` and expand as later tasks land.
3. Add integration files and popup picker: `P14-05` through `P14-07`.
4. Validate generic selected-pane behavior: `P14-08`.
5. Add explicit execution only after insertion is safe: `P14-09`.
6. Add tmux-safe form behavior: `P14-10`.
7. Finish with manual verification docs: `P14-12`.

## Non-Negotiable Design Rules

- Existing stdout shell plugin behavior must not regress.
- Tmux popup workflows must preserve and target the original pane id.
- Tmux insertion must not press Enter unless `--enter` is explicitly used.
- Picker scripts must not parse CSV with `cut -d,`; use JSON-derived TSV or add a future TSV output mode.
- Multiline tmux behavior must be explicit: reject it or implement a verified paste-buffer strategy.
