# Cursive Form Renderer Implementation

## Context
Built the Cursive-based TUI form renderer for terminal-expander. This is the core differentiator — Espanso-style forms rendered in-terminal instead of as GUI popups.

## Finding
Cursive 0.21.1 provides all needed widgets for form rendering:
- `EditView` — single-line text input with `.content()` builder and `.get_content()` → `Arc<String>`
- `TextArea` — multiline input with `.content()` and `.get_content()` → `&str`
- `SelectView::new().popup()` — dropdown selection with `.add_item_str()` and `.set_selection(idx)` returning `Callback`
- `Dialog::around()` — modal wrapper with title
- `Button::new(label, cb)` — callbacks receive `&mut Cursive`

## Resolution
All form field types implemented: text, multiline, choice, list. Tab navigation works natively in Cursive. Submit captures values via `call_on_name()`. Cancel returns None.

## Tags
cursive, tui, form, editview, textarea, selectview, dialog
