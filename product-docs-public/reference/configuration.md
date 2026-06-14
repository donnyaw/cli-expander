# Configuration Reference

## Directory Structure

```
~/.config/texpand/
├── matches/          # YAML match files (loaded alphabetically)
│   ├── base.yml
│   ├── git.yml
│   └── emoji.yml
└── config.yml        # Global config (optional)
```

## Match File Format

Match files use Espanso-compatible YAML syntax. Each file contains a list of matches:

```yaml
matches:
  - trigger: ":hello"
    replace: "Hello World!"
    # ... match-level settings
```

### Top-Level Keys

| Key | Type | Required | Description |
|-----|------|----------|-------------|
| `matches` | array | Yes | List of match rules |

### Match-Level Keys

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `trigger` | string | — | Single trigger keyword |
| `triggers` | array | — | Multiple triggers for same expansion |
| `replace` | string | — | Text to replace the trigger with |
| `form` | string | — | Form layout with `[[field]]` placeholders |
| `form_fields` | object | — | Field configuration for forms |
| `vars` | array | — | Variable definitions |
| `force_mode` | string | — | Injection mode override |
| `propagate_case` | bool | false | Match case of typed trigger |
| `word` | bool | false | Word boundary required |

### Match Example

```yaml
matches:
  - trigger: ":hello"
    replace: "Hello World!"
    propagate_case: true
    word: true

  - triggers: [":hi", ":hey"]
    replace: "Hi there!"

  - trigger: ":greet"
    form: |
      Hello [[name]]!
    form_fields:
      name:
        placeholder: "Your name"
        default: "Friend"
```

## Config File Format

The optional `config.yml` can contain global settings:

```yaml
backend: clipboard     # Injection mode: keys, clipboard, evdev
search_trigger: ":"
max_form_width: 80
max_form_height: 40
```
