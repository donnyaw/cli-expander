# User Guide

## How It Works

terminal-expander watches for trigger patterns in your input text. When a trigger is detected, it replaces it with the configured expansion — either static text or an interactive form.

### Basic Flow

```
You type: "Say :hello to the team"
                        ↓
Expander finds trigger ":hello"
                        ↓
Replaces with configured text
                        ↓
Output: "Say Hello World! to the team"
```

## Commands

### `texpand expand <input>`

The primary command. Pass your typed text and it returns the expansion.

```bash
# Simple trigger
texpand expand ":hello"
# Output: Hello World!

# Trigger in context
texpand expand "say :hello now"
# Output: say Hello World! now
```

**Exit codes:**
- `0`: Success, expansion printed
- `1`: No trigger found

### `texpand list`

List all available triggers from your match files:

```bash
$ texpand list
Trigger              Replace/Form                             Type
-------------------- ---------------------------------------- ----------
:hello               Hello World!                             text
:now                 {{time}}                                 text
:greet                                                        form
:choose                                                       form
```

### `texpand form <layout>`

Open an interactive form directly (useful for testing):

```bash
texpand form "Name: [[name]], Age: [[age]]" --title "User Info"
```

This opens a Cursive TUI with two input fields. Submit with Ctrl+Enter, cancel with Esc.

### `texpand config`

Show configuration paths and shell plugin locations:

```bash
$ texpand config
texpand configuration:
  Config directory: ~/.config/texpand/matches/
  Config file: ~/.config/texpand/config.yml

Shell plugins available at:
  shell/texpand.zsh — Zsh plugin
  shell/texpand.bash — Bash plugin
  shell/texpand.fish — Fish plugin
```

## Match Files

Match files are YAML files placed in `~/.config/texpand/matches/`. Each file contains a list of match rules.

### Simple Text Replacement

```yaml
matches:
  - trigger: ":thanks"
    replace: "Thank you for your help!"
```

### Multiple Triggers

```yaml
matches:
  - triggers: [":hello", ":hi", ":hey"]
    replace: "Greetings!"
```

### Multi-line Replacement

```yaml
matches:
  - trigger: ":sig"
    replace: |
      Best regards,
      John Doe
      Engineering Team
```

## Form Usage

Forms let you collect user input before expansion. They work exactly like Espanso forms but render in your terminal instead of a GUI popup.

### Simple Form

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

### Form with Choice Dropdown

```yaml
- trigger: ":priority"
  form: "Set priority to [[level]]"
  form_fields:
    level:
      type: choice
      values:
        - Low
        - Medium
        - High
```

### Form with Default Values

```yaml
- trigger: ":meeting"
  replace: "Meeting with {{form.person}} at {{form.location}}"
  vars:
    - name: form
      type: form
      params:
        layout: |
          Who: [[person]]
          Where: [[location]]
        fields:
          person:
            default: "The Team"
          location:
            default: "Main Conference Room"
```

## Date Variables

```yaml
- trigger: ":today"
  replace: "{{today}}"
  vars:
    - name: today
      type: date
      params:
        format: "%Y-%m-%d"
```

Available format specifiers follow `strftime` conventions:

| Specifier | Output |
|-----------|--------|
| `%Y` | 2026 |
| `%m` | 06 |
| `%d` | 14 |
| `%H:%M` | 14:30 |
| `%B %d, %Y` | June 14, 2026 |

### Date with Offset

```yaml
- trigger: ":tomorrow"
  replace: "{{date}}"
  vars:
    - name: date
      type: date
      params:
        format: "%Y-%m-%d"
        offset: 86400   # +1 day in seconds
```

## Clipboard Variables

```yaml
- trigger: ":clip"
  replace: "{{clip}}"
  vars:
    - name: clip
      type: clipboard
```

## Shell Command Variables

```yaml
- trigger: ":ip"
  replace: "{{ip}}"
  vars:
    - name: ip
      type: shell
      params:
        cmd: "hostname -I | awk '{print $1}'"
```

## Variable Chaining

Variables can be chained. For example, pre-fill a form field with clipboard content:

```yaml
- trigger: ":review"
  replace: |
    Review by: {{user}}
    Date: {{date}}
    Notes: {{form.notes}}
  vars:
    - name: clip
      type: clipboard
    - name: date
      type: date
      params:
        format: "%Y-%m-%d"
    - name: form
      type: form
      params:
        layout: "Notes: [[notes]]"
        fields:
          notes:
            multiline: true
            default: "{{clip}}"
```
