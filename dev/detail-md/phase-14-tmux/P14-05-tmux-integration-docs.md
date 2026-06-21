# P14-05: Create Tmux Integration Directory And Install Docs

## Objective

Create a dedicated tmux integration location with install instructions, recommended bindings, and selected-pane workflow documentation.

## Implementation Status

Completed on branch `feature/tmux-integration` with tag `p14-05`.

## Why This Exists

Tmux integration has different usage patterns from shell plugins. Users need one clear place for tmux-specific files and guidance.

## Scope

- Create `integrations/tmux/`.
- Add a tmux README.
- Add a starter `.tmux` config snippet.
- Do not require a full installer yet.

## Proposed Files

```text
integrations/tmux/
├── README.md
├── cli-expander.tmux
└── ce-tmux-picker.sh
```

`ce-tmux-picker.sh` may be a placeholder until `P14-06`.

## README Content

Document these workflows:

- Existing shell plugin mode for prompt-local expansion.
- Tmux selected-pane injection mode.
- Popup picker mode.
- How to get a pane id with `#{pane_id}`.
- Why popup commands must pass the original pane id.
- Why Enter is not sent by default.
- Known form limitations.

## Example Tmux Binding

```tmux
bind-key C-e display-popup -E "ce-tmux-picker '#{pane_id}'"
```

## Acceptance Criteria

- `integrations/tmux/README.md` exists.
- README explains selected-pane behavior.
- README links back to the Phase 14 design or summarizes it.
- Example binding is copyable.

## Test Plan

- Read docs from a new-user perspective.
- Verify snippets use valid tmux syntax.
- Verify all paths and command names are consistent with the repo.

## Dependencies

- `P14-03`.

## Follow-Up

`P14-06` will fill in the actual popup picker script.
