# Form Syntax Reference

Forms are interactive dialogs that collect user input before expansion. They render in the terminal using Cursive TUI.

## Form Field Types

| Type | Description | Widget |
|------|-------------|--------|
| `text` | Single-line text input (default) | `EditView` |
| `choice` | Dropdown selection list | `SelectView::popup()` |
| `list` | Multi-option scrollable list | `SelectView` |
| `multiline` | Multi-line text area | `TextArea` |

## Shorthand Syntax

The simplest form syntax uses `form:` with inline `[[field]]` placeholders and `form_fields:` for configuration:

```yaml
- trigger: ":greet"
  form: |
    Hello [[name]]!
    You are [[age]] years old.
  form_fields:
    name:
      placeholder: "Your name"
    age:
      placeholder: "Your age"
```

## Verbose Syntax

For advanced usage (variable chaining, shell integration), use the verbose `vars` syntax:

```yaml
- trigger: ":greet"
  replace: "Hello {{form.name}}! You are {{form.age}}."
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
              - 36+
```

## Field Configuration

| Parameter | Type | Applies to | Description |
|-----------|------|-----------|-------------|
| `type` | string | all | `text`, `choice`, or `list` |
| `multiline` | bool | text | Enable multi-line input |
| `default` | string | all | Pre-filled value |
| `placeholder` | string | text | Hint text when empty |
| `values` | array/string | choice, list | Options as array or multiline YAML |
| `trim_string_values` | bool | choice, list | Trim whitespace from values (default: true) |

### Text Field

```yaml
field_name:
  placeholder: "Enter text"
  default: "Default value"
  multiline: false
```

### Multiline Text Area

```yaml
field_name:
  multiline: true
  default: |
    Default
    multi-line
    content
```

### Choice Dropdown

```yaml
field_name:
  type: choice
  values:
    - Option A
    - Option B
    - Option C
  default: "Option B"
```

### Dynamic Values from Shell

```yaml
field_name:
  type: choice
  values: "{{files}}"
```

Combined with a shell variable:

```yaml
- trigger: ":file"
  replace: "{{form.file}}"
  vars:
    - name: files
      type: shell
      params:
        cmd: "ls ~/Documents"
    - name: form
      type: form
      params:
        layout: "Select file: [[file]]"
        fields:
          file:
            type: list
            values: "{{files}}"
```

## Form Behavior

- **Tab** — Move focus to next field
- **Shift+Tab** — Move focus to previous field
- **Ctrl+Enter** — Submit form
- **Esc** — Cancel form
- **Enter (text fields)** — New line in multiline, submit otherwise
- **Arrow keys** — Navigate choice/list options
