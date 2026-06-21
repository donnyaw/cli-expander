# P14-06: Build Tmux Fzf Popup Picker MVP

## Objective

Build a tmux popup picker that lets the user select a trigger with `fzf` and inject the expanded result into the original selected pane.

## Implementation Status

Verified for merge with task tag `p14-06`.

## Why This Exists

The main tmux user experience should not require memorizing every trigger. A popup picker gives tmux users fast discovery and selected-pane insertion.

## Scope

- Implement `integrations/tmux/ce-tmux-picker.sh`.
- Use `ce list --json` as the source of truth and convert to a tab-separated picker view.
- Use `fzf` for selection.
- Use `ce expand --output tmux --target-pane` for insertion.
- Keep the MVP simple.

## Proposed Flow

```text
tmux binding passes original pane id
-> popup runs ce-tmux-picker.sh <pane-id>
-> script runs ce list --json and builds fzf rows
-> user selects trigger
-> script calls ce expand <trigger> --output tmux --target-pane <pane-id>
-> popup exits
```

## Implementation Steps

1. Script validates required commands:

- `tmux`
- `ce`
- `fzf`

2. Script accepts original pane id as first argument.

3. Script converts JSON to tab-separated rows so trigger extraction is reliable even when descriptions contain commas.

4. Script shows fzf using tab-separated rows.

5. Script extracts the trigger column.

6. Script expands into original pane.

7. Script exits cleanly on cancel.

## Data Format Rule

Do not parse CSV with `cut -d,` for the picker. Trigger descriptions and source paths can contain commas, which makes CSV parsing fragile in shell. Use JSON and convert to TSV with a reliable parser.

Preferred minimal conversion:

```bash
ce list --json | python3 -c 'import json,sys;\nfor r in json.load(sys.stdin): print("{}\t{}\t{}".format(r["trigger"], r.get("description", ""), r.get("category", "")))'
```

If `python3` is not acceptable as a dependency, add a future `ce list --tsv` command instead of shell-parsing CSV.

## Preview Option

Use `ce details` as fzf preview with the first tab-separated field:

```bash
fzf --delimiter='\t' --with-nth=1,2 --preview='ce details {1}'
```

## Acceptance Criteria

- Picker opens in tmux popup.
- Canceling the picker does not inject anything.
- Selecting a trigger injects the expansion into the original pane.
- Errors for missing `fzf` or `ce` are clear.
- Descriptions with commas do not break trigger extraction.

## Test Plan

- Manual-test with two panes.
- Manual-test cancel path.
- Manual-test selected trigger path.
- Manual-test missing `fzf` by temporarily altering `PATH`.
- Manual-test a trigger whose description contains a comma.

## Dependencies

- `P14-03`.
- `P14-05`.

## Follow-Up

`P14-07` hardens original pane preservation around popup execution.
