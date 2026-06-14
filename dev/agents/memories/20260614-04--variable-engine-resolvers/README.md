# Variable Engine and Template System

## Context
Built the variable resolution system supporting date, clipboard, shell, and form variables.

## Finding
- `chrono::DateTime::from_timestamp(secs, 0)` converts Unix timestamps to DateTime
- `arboard::Clipboard::new()` may fail in headless environments (no display server) — must handle errors gracefully
- Shell variables use `sh -c <cmd>` for cross-platform compatibility
- Template `extract_variable_names()` uses simple `{{name}}` pattern matching (no regex needed for basic cases)
- Clippy enforces `FromStr` trait implementation instead of standalone `from_str()` method
- `serde_norway::Value` uses `as_sequence()` not `as_array()` — different from serde_json

## Resolution
VariableEngine with 3 default resolvers: DateVariable, ClipboardVariable, ShellVariable. Template supports `{{var}}` substitution and `$|$` cursor marker.

## Tags
template, variables, date, clipboard, shell, chrono, arboard
