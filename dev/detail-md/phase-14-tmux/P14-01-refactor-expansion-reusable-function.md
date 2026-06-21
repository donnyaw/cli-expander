# P14-01: Refactor Expansion Into Reusable CLI Function

## Objective

Extract the current trigger expansion pipeline from `cli-expander-cli/src/main.rs` into a reusable function so every output mode can share the same behavior.

## Why This Exists

Today, expansion and output are coupled inside the `Commands::Expand` match arm. Tmux integration needs the same expanded string to be routed to stdout, tmux, clipboard, popup scripts, and later context-aware behavior. This task separates expansion from delivery.

## Scope

- Move expansion logic into a function that returns `String`.
- Keep stdout behavior unchanged.
- Preserve text triggers, form triggers, variable resolution, default form fallback, and output normalization.
- Do not add tmux behavior in this task.

## Implementation Steps

1. Add a helper near the bottom of `cli-expander-cli/src/main.rs`:

```rust
fn expand_input(input: &str, config_dir: &str) -> anyhow::Result<String>
```

2. Move these responsibilities into the helper:

- Expand `~` in `config_dir`.
- Load config files with `Config::load_dir`.
- Build `Matcher::from_files`.
- Run `find_best` with fallback to `find_in`.
- Render static replacements.
- Render direct `form` matches.
- Render `vars` with `type: form`.
- Call `normalize_command_output` before returning.

3. Change `Commands::Expand` to call the helper and print the returned string.

4. Keep error messages compatible with the existing command behavior.

## Acceptance Criteria

- `ce :hello` behaves exactly as before.
- `ce expand :hello` behaves exactly as before.
- Form trigger behavior remains unchanged.
- No tmux-specific CLI option is added yet.
- Existing tests pass.

## Test Plan

- Run `cargo test`.
- Manually run a text trigger from sample config.
- Manually run a form trigger if local sample files exist.

## Dependencies

- None.

## Follow-Up

This enables `P14-02`, where output dispatch will be added after expansion returns a reusable string.
