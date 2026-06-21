# P14-08: Add Generic Selected-Pane Workflow Without App Detection

## Objective

Support insertion into any focused tmux pane without detecting which application is running inside that pane.

## Why This Exists

The agreed MVP is selected-pane support first. App-specific behavior for Vim, OpenCode, shells, or TUIs can add complexity later. The first reliable version should simply insert literal expanded text into the selected pane.

## Scope

- Treat all target panes the same.
- Insert literal text only.
- Do not auto-detect shell/editor/TUI.
- Do not add context config yet.

## Supported MVP Targets

- Bash prompt.
- Zsh prompt.
- Fish prompt.
- Vim or Neovim in insert mode.
- OpenCode input area.
- Generic terminal program that accepts typed text.

## Non-Goals

- Detecting insert mode in Vim.
- Detecting OpenCode prompt focus.
- Detecting shell prompt readiness.
- Bracketed paste support.
- App-specific escaping.

## Implementation Steps

1. Document generic behavior in tmux README.

2. Ensure picker and direct injection use the same tmux output path.

3. Ensure no code path assumes the target is a shell prompt.

4. Keep execution disabled unless `--enter` is explicitly provided in `P14-09`.

5. Add manual verification across common pane types.

## Acceptance Criteria

- User can inject into any selected pane.
- No application-specific logic is required for MVP.
- Documentation clearly states that the target app must be ready to receive typed text.

## Test Plan

- Test injection into a shell prompt.
- Test injection into a text editor in insert mode.
- Test injection into OpenCode input if available.
- Test injection into a full-screen TUI input area if available.

## Dependencies

- `P14-07`.

## Follow-Up

Future phases may add optional app-aware rules using `tmux display-message -p '#{pane_current_command}'`.
