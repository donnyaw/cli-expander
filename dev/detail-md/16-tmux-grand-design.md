# Phase 14: Tmux Grand Design

## Goal

Make `cli-expander` work naturally from tmux, targeting the currently selected pane first and supporting any pane content without app-specific detection. The initial integration should support shell prompts, editors, OpenCode, TUIs, and generic terminal programs by inserting literal expanded text into the selected pane. App-aware behavior can come later after the selected-pane foundation is reliable.

## User Workflow Target

The user is working inside tmux with multiple panes. They focus a pane, press a tmux keybinding, search or choose a `cli-expander` trigger, and the expanded text is inserted into that selected pane. The selected pane may contain Bash, Zsh, Fish, Vim, OpenCode, or another terminal application. The first version should not try to detect or customize behavior for each app. It should insert text safely and predictably into whichever pane the user selected.

## Current Codebase Starting Point

The codebase already has most of the backend pieces:

- `cli-expander-cli/src/main.rs` expands triggers and prints the result to stdout.
- `cli-expander-config` loads YAML match files from `~/.config/cli-expander/matches`.
- `cli-expander-match` finds the matching trigger in the prompt buffer.
- `cli-expander-render` renders templates and variables.
- `cli-expander-ui` renders Cursive forms.
- `cli-expander-inject/src/injector.rs` already has `TmuxInjector` using `tmux send-keys -l`.
- `shell/cli-expander.bash`, `shell/cli-expander.zsh`, and `shell/cli-expander.fish` already use stdout mode for prompt-local expansion.

The missing layer is a user-facing tmux command surface: CLI output modes, target-pane selection, popup picker scripts, tmux docs, and tests around command construction.

## Design Principles

Default behavior must remain backward compatible. Existing shell plugin workflows depend on stdout output and should not change.

Tmux integration should be explicit. A user or tmux binding should request tmux injection with `--output tmux` or `--output auto`.

Insertion must be safe by default. Expanded text should be inserted into the pane but should not press Enter unless the user explicitly enables `--enter`.

The first tmux release should target any selected pane without app detection. App-specific handling for Vim, OpenCode, shells, or full-screen TUIs should be treated as a later layer.

Popup workflows must preserve the original pane. Once a tmux popup opens, the popup becomes the active UI context. The integration must capture the original `#{pane_id}` before opening the popup and inject back into that original pane.

## Architecture

```text
tmux selected pane
  -> tmux keybinding
  -> optional tmux popup / fzf picker
  -> ce expand <trigger> --output tmux --target-pane <original-pane>
  -> cli-expander loads config, matches trigger, renders text/form result
  -> TmuxInjector sends literal keys to target pane
```

Shell plugin mode remains separate:

```text
shell prompt
  -> Space or Ctrl+T binding
  -> ce expand <buffer> --output stdout
  -> shell plugin updates READLINE_LINE / BUFFER / commandline
```

These two paths should share the same expansion function but use different output dispatch.

## CSV Row Explanations

Each Phase 14 task also has its own detailed working plan under `dev/detail-md/phase-14-tmux/`. The sections below remain the grand design overview; implementers should follow the per-task file for exact acceptance criteria and test details.

### P14-01: Refactor Expansion Into Reusable CLI Function

The current expansion logic lives inside the `Commands::Expand` match arm in `cli-expander-cli/src/main.rs`. That is workable for stdout-only behavior, but tmux integration needs the same expansion result to be reused by stdout, tmux injection, clipboard output, popup picker flows, and future app-aware contexts.

This task extracts the expansion pipeline into a function such as:

```rust
fn expand_input(input: &str, config_dir: &str) -> anyhow::Result<String>
```

The function should load configs, create the matcher, find the trigger, render text or form output, normalize command output, and return the final string. It should not print, inject, or mutate the terminal. That makes output routing a separate concern.

The expected result is no behavior change. Existing commands should still work the same, but the code becomes ready for multiple output backends.

### P14-02: Add CLI Output Modes For Stdout Tmux Auto Clipboard

This task adds an output selector to the CLI. The default remains stdout.

Target interface:

```bash
ce expand ":hello" --output stdout
ce expand ":hello" --output tmux
ce expand ":hello" --output auto
ce expand ":hello" --output clipboard
```

`stdout` prints the expanded text. This preserves existing shell plugin behavior.

`tmux` sends the expanded text with the tmux injector.

`auto` should use tmux when `$TMUX` exists, otherwise it should fall back to stdout at first. A later version can make auto more advanced.

`clipboard` copies the expanded text to the system clipboard using the existing clipboard injector.

This task is the core bridge between the existing expansion engine and the existing injection crate.

### P14-03: Add Tmux Target Pane Support

The user wants insertion into the selected/current pane. Tmux supports this with pane targets.

Target interface:

```bash
ce expand ":hello" --output tmux --target-pane "%1"
ce expand ":hello" --output tmux --target-pane "$TMUX_PANE"
```

The injector should construct a command equivalent to:

```bash
tmux send-keys -t "$target_pane" -l "$text"
```

If `--target-pane` is omitted, tmux can default to the current pane. For popup workflows, the target pane should always be passed explicitly because the popup changes active context.

This row establishes pane-aware routing without any app detection.

### P14-04: Harden Tmux Injector For Literal Safe Insertion

The existing `TmuxInjector` sends literal text with `tmux send-keys -l`, which is the right foundation. This task hardens it for production use.

Expected details:

- Preserve literal shell characters like `;`, `&`, `|`, quotes, braces, and dollar signs.
- Define how multiline output is handled.
- Keep `$|$` cursor marker behavior consistent with shell plugin behavior or explicitly document that tmux mode strips/does not support it initially.
- Improve errors when `tmux` is missing, `$TMUX` is not set, or target pane is invalid.
- Do not press Enter by default.

The critical design rule is that injection should insert text, not execute it.

### P14-05: Create Tmux Integration Directory And Install Docs

This task creates a dedicated tmux integration area:

```text
integrations/tmux/
├── README.md
├── cli-expander.tmux
└── ce-tmux-picker.sh
```

The first version may only include docs and suggested snippets. The purpose is to give tmux users one obvious place to install and understand the integration.

The README should document:

- Recommended shell-plugin workflow.
- Recommended tmux selected-pane workflow.
- How to bind keys in `.tmux.conf`.
- Why popup commands must preserve the original pane id.
- Safety behavior around not pressing Enter by default.
- Known limitations around Cursive forms.

### P14-06: Build Tmux Fzf Popup Picker MVP

This is the main user-facing tmux feature. The workflow is:

```text
press tmux keybinding
-> popup opens
-> fzf lists triggers from JSON-derived TSV rows
-> user selects trigger
-> ce expands trigger
-> result is injected into original selected pane
```

The picker can start as a shell script, for example `integrations/tmux/ce-tmux-picker.sh`. It should depend only on `tmux`, `ce`, and `fzf`.

The picker should use existing CLI commands, but it should not shell-parse CSV with comma splitting. Use JSON as the source of truth and convert to tab-separated picker rows:

```bash
ce list --json
ce details <trigger>
ce expand <trigger> --output tmux --target-pane <pane>
```

The first version should favor simplicity over complex UX. A reliable picker that inserts into the right pane is more important than advanced preview formatting. Avoid fragile CSV parsing because descriptions and source paths can contain commas.

### P14-07: Preserve Original Pane Across Popup Execution

This task protects against the most common tmux popup mistake. When a popup opens, the command inside the popup is not the same interaction context as the original pane. If the picker simply calls `tmux send-keys` without a target, it may inject into the wrong place.

The tmux binding should pass the original pane id using tmux format expansion:

```tmux
bind-key C-e display-popup -E "ce-tmux-picker '#{pane_id}'"
```

Then the picker uses that pane id:

```bash
ce expand "$trigger" --output tmux --target-pane "$original_pane"
```

This is essential for the user goal: the selected pane should receive the expansion, not the popup.

### P14-08: Add Generic Selected-Pane Workflow Without App Detection

The agreed MVP is option 4: shell prompt plus any selected tmux pane, with no special app detection yet.

That means the integration should work the same whether the target pane is running Bash, Zsh, Fish, Vim, OpenCode, or another TUI. The tool inserts literal text into the pane and does not try to understand the application state.

This avoids premature complexity. App-aware behavior can be added later after the base behavior is proven reliable.

Examples of future app-aware behavior that should not block the MVP:

- Different paste strategy for Vim insert mode.
- OpenCode prompt-specific expansion behavior.
- Shell prompt detection before inserting.
- Per-app escaping or paste-buffer flows.

### P14-09: Add Optional --enter Execution Flag

By default, inserted text should not execute. The user should be able to inspect the command first.

Some workflows may want immediate execution. This should be explicit:

```bash
ce expand ":docker-ps" --output tmux --target-pane "%1" --enter
```

Implementation can send the literal text first, then send Enter separately:

```bash
tmux send-keys -t "$target" -l "$text"
tmux send-keys -t "$target" Enter
```

This flag should be documented as powerful and potentially dangerous for command-building triggers.

### P14-10: Add Tmux-Safe Form Execution Strategy

Forms are the most sensitive part of the integration because the current UI uses Cursive. Full-screen or alternate-screen terminal behavior can be fragile inside tmux.

The recommended tmux strategy is to run forms inside the popup, not inside the target pane. After the user completes the form, inject the final command into the original selected pane.

Target flow:

```text
selected pane remains stable
-> popup opens
-> form runs in popup
-> form returns final command
-> popup closes
-> final command is inserted into original pane
```

This keeps visual form interaction separate from the target pane and reduces terminal state corruption risk.

The first version can document limitations and keep form support basic. A future improvement can add a wizard-style non-Cursive form renderer for tmux reliability.

### P14-11: Add Tests For Output Mode And Tmux Command Construction

Tests should validate the integration without requiring a live tmux server in CI.

Recommended test coverage:

- `stdout` remains the default.
- `--output tmux` routes to tmux mode.
- `--output auto` chooses tmux when `$TMUX` is present.
- `--target-pane` is included in the constructed tmux command.
- No Enter is sent unless `--enter` is set.
- Invalid output mode fails clearly.

The code may need a small abstraction around process execution so tests can inspect the intended command without launching tmux.

Live tmux tests can be added as ignored/manual tests later.

### P14-12: Add Manual Tmux Verification Checklist

Tmux behavior should be verified manually because pane focus, popup behavior, and terminal state are hard to fully reproduce in unit tests.

The checklist should include:

```text
1. Open tmux with two panes.
2. Focus pane A and inject :hello.
3. Confirm pane A receives text, pane B does not.
4. Open popup picker from pane A.
5. Select a trigger and confirm pane A receives text after popup closes.
6. Test multiline insertion.
7. Test --enter only executes when explicitly enabled.
8. Test inside Bash, inside editor, and inside OpenCode/TUI as generic selected-pane targets.
9. Test behavior when fzf is missing.
10. Test behavior outside tmux.
```

This checklist should live in `integrations/tmux/README.md` or a dedicated verification document.

## Recommended Delivery Order

1. Complete `P14-01` through `P14-04` first. This creates the technical foundation.
2. Complete `P14-11` early enough to lock down output-routing behavior.
3. Complete `P14-05` through `P14-07` for the first usable tmux popup workflow.
4. Complete `P14-08` to document and verify the generic selected-pane MVP.
5. Add `P14-09` only after insertion-only behavior is stable.
6. Add `P14-10` after popup picker basics work.
7. Finish with `P14-12` to keep manual verification repeatable.

## MVP Definition

The MVP is complete when this works reliably:

```text
Inside tmux, focus any pane, press a tmux keybinding, choose a trigger from fzf, and have the expanded text inserted into that original focused pane without pressing Enter automatically.
```

The MVP does not require app detection, Vim-specific behavior, OpenCode-specific behavior, or automatic execution.

## Known Risks

Tmux popup target confusion is the highest risk. Always pass the original pane id explicitly.

Cursive form rendering may behave poorly in some tmux/terminal combinations. Popup-hosted forms reduce the impact but may not eliminate all issues.

Multiline insertion needs an explicit strategy before implementation. The safest MVP is to reject multiline tmux injection with a clear error until a verified paste-buffer strategy exists. Do not silently translate newlines into Enter keypresses.

Shell plugin mode and tmux injection mode can overlap. Documentation must explain when to use each mode.

## Future App-Aware Layer

After generic selected-pane support works, add optional context rules. These should not be part of the first implementation.

Possible future config:

```yaml
contexts:
  nvim:
    output: tmux
    paste_mode: bracketed
  opencode:
    output: tmux
    enter: false
  shell:
    output: stdout
```

Possible detection commands:

```bash
tmux display-message -p '#{pane_current_command}'
tmux display-message -p '#{pane_current_path}'
tmux display-message -p '#{pane_title}'
```

This future layer should be built only after the selected-pane MVP is stable.
