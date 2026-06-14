# System Architecture

## Crate Map

```
                    ┌─────────────┐
                    │ texpand     │
                    │ (CLI bin)   │
                    └──────┬──────┘
          ┌────────────────┼────────────────┐
          v                v                v
  ┌────────────┐   ┌────────────┐   ┌──────────────┐
  │texpand-    │   │texpand-    │   │texpand-render│
  │config      │──▶│match       │──▶│              │
  └────────────┘   └────────────┘   └──────┬───────┘
          ┌────────────────────────────────┼──────────┐
          v                                v          v
  ┌────────────┐                   ┌────────────┐  ┌──────────┐
  │texpand-    │                   │texpand-    │  │texpand-ui│
  │detect      │                   │inject      │  │(Cursive) │
  └────────────┘                   └────────────┘  └──────────┘
```

## Key Traits

| Trait | Crate | Method | Purpose |
|-------|-------|--------|---------|
| `KeySource` | texpand-detect | `initialize()`, `read_event()` | Platform input detection |
| `Injector` | texpand-inject | `inject(text)` | Platform text injection |
| `FormRenderer` | texpand-ui | `show(title, fields)` | Terminal UI rendering |
| `VariableResolver` | texpand-render | `resolve(type, params)` | Variable value resolution |

## Data Flow: Trigger → Expansion

```
User types → RollingBuffer accumulates keys
    → Matcher.find_best() checks triggers
    → Match found?
      ├── No → continue buffering
      └── Yes
          ├── Has form? → CursiveFormRenderer.show()
          │   └── User submits → FormExtension.render_form()
          └── Has variables? → VariableEngine.resolve_all()
              └── Template.render() → output
```

## CLI Commands

- `texpand expand <input>` — Find trigger and expand
- `texpand list` — List all available triggers
- `texpand form <layout>` — Interactive form prompt
- `texpand config` — Show configuration info
