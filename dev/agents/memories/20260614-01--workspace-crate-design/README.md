# Workspace and Crate Design

## Context
Set up the Rust workspace with 7 crate members for the terminal-expander project.

## Finding
- Workspace `Cargo.toml` with `[workspace]` and `members` list works cleanly
- `serde_norway` is required for Espanso-compatible YAML parsing — `serde_yaml` breaks on multiline `form: |` syntax
- `rust-toolchain.toml` with `channel = "stable"` and `components = ["rustfmt", "clippy"]` ensures consistent tooling
- `[[bin]]` override in crate Cargo.toml fixes binary name — without it, the binary is named `texpand-cli` instead of `texpand`
- Workspace dependency sharing via `[workspace.dependencies]` keeps versions consistent

## Resolution
Workspace configured with 7 crates: config, detect, match, render, inject, ui, cli. Binary renamed to `texpand` via [[bin]] section.

## Tags
rust, workspace, cargo, serde_norway, toolchain
