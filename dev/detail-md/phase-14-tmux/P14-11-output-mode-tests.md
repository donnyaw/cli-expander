# P14-11: Add Tests For Output Mode And Tmux Command Construction

## Objective

Add automated coverage for output routing and tmux command construction without requiring a live tmux server in CI.

## Implementation Status

Completed on branch `feature/tmux-integration` with tag `p14-11`.

## Why This Exists

Tmux behavior has several safety-sensitive paths: target pane routing, explicit Enter, and literal insertion. These should be tested at the construction/routing level even when tmux is not running.

## Scope

- Test stdout default behavior.
- Test output mode parsing.
- Test target pane command construction.
- Test `--enter` command sequence after `P14-09`.
- Avoid requiring real tmux in standard CI.

## Implementation Strategy

Prefer pure command-construction helpers first. Add a process-runner abstraction only if pure helpers are not enough.

Recommended helper shape:

```rust
fn tmux_send_keys_args(text: &str, target: Option<&str>) -> Vec<String>
```

Introduce a small command-runner abstraction only if implementation code otherwise becomes untestable:

```rust
trait CommandRunner {
    fn run(&self, program: &str, args: &[String]) -> anyhow::Result<CommandOutput>;
}
```

Production uses a real runner. Tests use a fake runner and assert the program/args.

Do not spawn real `tmux`, `fzf`, or clipboard providers in ordinary tests.

## Test Cases

- `ce expand :hello` defaults to stdout.
- `--output stdout` prints output.
- `--output tmux` selects tmux injector.
- `--output auto` chooses tmux when `$TMUX` exists.
- `--target-pane %1` produces `send-keys -t %1 -l <text>`.
- Missing target still produces current-pane tmux command.
- `--enter` sends Enter only when explicitly true.
- Invalid output mode fails through Clap.
- Multiline tmux input follows the explicit rule from `P14-04`.
- Picker data conversion does not rely on CSV comma splitting.

## Acceptance Criteria

- Tests can run with `cargo test` outside tmux.
- Tests do not depend on `fzf`.
- Tests do not mutate a real terminal.
- Existing integration tests still pass.
- Tests can be understood without starting tmux manually.

## Dependencies

- `P14-02`.
- `P14-03` for target-pane tests.
- `P14-09` for Enter tests.

## Follow-Up

Add ignored live tmux tests later for end-to-end manual or CI environments with tmux available.
