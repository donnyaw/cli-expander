# Variable System Reference

Variables let you inject dynamic content into your expansions. They are defined in the `vars:` section of a match.

## Variable Types

| Type | Description | Resolver |
|------|-------------|----------|
| `date` | Current date/time | `DateVariable` |
| `clipboard` | System clipboard content | `ClipboardVariable` |
| `shell` | Shell command output | `ShellVariable` |
| `form` | Interactive form input | `FormExtension` |
| `match` | Nested match expansion | `MatchVariable` |

## Date Variable

```yaml
vars:
  - name: today
    type: date
    params:
      format: "%Y-%m-%d"     # strftime format
      offset: 86400           # seconds (optional)
```

### Format Specifiers

| Spec | Output |
|------|--------|
| `%Y` | 2026 |
| `%y` | 26 |
| `%m` | 06 |
| `%d` | 14 |
| `%H` | 14 (24-hour) |
| `%I` | 02 (12-hour) |
| `%M` | 30 |
| `%S` | 45 |
| `%p` | AM/PM |
| `%B` | June |
| `%b` | Jun |
| `%A` | Sunday |
| `%a` | Sun |
| `%j` | 165 (day of year) |
| `%W` | 24 (week number) |

### Date with Offset

```yaml
# Yesterday
- name: yesterday
  type: date
  params:
    format: "%Y-%m-%d"
    offset: -86400

# Tomorrow
- name: tomorrow
  type: date
  params:
    format: "%Y-%m-%d"
    offset: 86400

# Next hour
- name: next_hour
  type: date
  params:
    format: "%H:%M"
    offset: 3600
```

## Clipboard Variable

```yaml
vars:
  - name: clip
    type: clipboard
```

No parameters needed. Returns the current text content of the system clipboard.

## Shell Variable

```yaml
vars:
  - name: output
    type: shell
    params:
      cmd: "echo 'Hello from shell'"
```

The `cmd` parameter is a shell command executed via `sh -c`. The stdout is captured and trimmed.

### Shell Examples

```yaml
vars:
  - name: ip
    type: shell
    params:
      cmd: "hostname -I | awk '{print $1}'"

  - name: files
    type: shell
    params:
      cmd: "ls -1 ~/Documents | head -5"

  - name: weather
    type: shell
    params:
      cmd: "curl -s 'wttr.in?format=%C+%t'"
```

## Form Variable

```yaml
vars:
  - name: form
    type: form
    params:
      layout: |
        Name: [[name]]
        Age: [[age]]
      fields:
        name:
          placeholder: "Enter name"
        age:
          type: choice
          values:
            - 18-25
            - 26-35
```

Fields are accessed with dot notation: `{{form.name}}`, `{{form.age}}`.

## Match Variable (Nested)

```yaml
vars:
  - name: sig
    type: match
    params:
      trigger: ":signature"
```

Triggers another match and uses its output.

## Variable Chaining

Variables can reference other variables. This is useful for pre-filling form fields:

```yaml
vars:
  # Step 1: Get clipboard content
  - name: clip
    type: clipboard

  # Step 2: Get current date
  - name: date
    type: date
    params:
      format: "%Y-%m-%d"

  # Step 3: Form with clipboard as default
  - name: form
    type: form
    params:
      layout: |
        Date: [[date]]
        Content:
        [[content]]
      fields:
        date:
          default: "{{date}}"
        content:
          multiline: true
          default: "{{clip}}"

  # Step 4: Shell command using form values
  - name: result
    type: shell
    params:
      cmd: "echo 'Reviewed {{form.content}} on {{date}}'"
```

## Template Syntax

Variables are referenced in replacement text using `{{name}}` or `{{name.field}}`:

```yaml
replace: "{{greeting}}, {{user}}! Today is {{date}}."
```

### Cursor Position

Use `$|$` to mark where the cursor should be placed after expansion:

```yaml
replace: "<div>$|$</div>"
```

After expansion, the cursor will be positioned between `<div>` and `</div>`.

### Escaping

To include literal `{{` or `}}` in your output, they don't need escaping — only `{{name}}` patterns matching known variables will be replaced.
