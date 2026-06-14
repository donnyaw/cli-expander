# API Specification

## CLI Reference

### `texpand expand <input> [--config-dir <path>]`
Reads match files from `--config-dir` (default: `~/.config/texpand/matches/`), finds matching trigger, resolves variables, and prints expansion to stdout.

**Exit codes:**
- 0: Success, expansion printed
- 1: No match found

### `texpand list [--config-dir <path>]`
Lists all triggers with their replacement type (text/form).

### `texpand form <layout> [--title <title>]`
Opens interactive Cursive form with given layout string. Returns filled values with `[[field]]` replaced.

### `texpand config`
Prints configuration paths and available shell plugins.

## Match File Format (Espanso-compatible)

```yaml
matches:
  - trigger: ":greet"
    replace: "Hello {{name}}!"
    vars:
      - name: name
        type: form
        params:
          layout: "Your name: [[name]]"
```

### Supported field types:
- `text` (default) — Single-line text input
- `choice` — Dropdown selection (requires `values:`)
- `list` — Multi-option list (requires `values:`)

### Supported variable types:
- `date` — Current date/time (`format: "%Y-%m-%d"`, `offset: 86400`)
- `clipboard` — Current clipboard content
- `shell` — Command output (`cmd: "echo hello"`)
- `form` — Interactive form input (`layout: "[[field]]"`)
- `match` — Nested match expansion
