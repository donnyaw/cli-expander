# Example Match Files

## Base Examples

File: `base.yml`

```yaml
matches:
  # Simple text replacement
  - trigger: ":hello"
    replace: "Hello World!"

  - trigger: ":thanks"
    replace: "Thank you for your help!"

  # Multiple triggers
  - triggers: [":hi", ":hey", ":howdy"]
    replace: "Greetings!"

  # Multi-line
  - trigger: ":sig"
    replace: |
      Best regards,
      John Doe
      Engineering Team
      john@example.com

  # Date
  - trigger: ":today"
    replace: "Today's date: {{date}}"
    vars:
      - name: date
        type: date
        params:
          format: "%B %d, %Y"

  - trigger: ":time"
    replace: "{{time}}"
    vars:
      - name: time
        type: date
        params:
          format: "%H:%M"

  # Clipboard
  - trigger: ":clip"
    replace: "{{clip}}"
    vars:
      - name: clip
        type: clipboard

  # Shell command
  - trigger: ":ip"
    replace: "IP: {{ip}}"
    vars:
      - name: ip
        type: shell
        params:
          cmd: "hostname -I | awk '{print $1}'"
```

## Form Examples

File: `forms.yml`

```yaml
matches:
  # Simple form
  - trigger: ":greet"
    form: |
      Hello [[name]]!
      Welcome to the team.
    form_fields:
      name:
        placeholder: "Your full name"

  # Form with choice
  - trigger: ":priority"
    form: "Priority: [[level]]"
    form_fields:
      level:
        type: choice
        values:
          - Low
          - Medium
          - High
          - Critical

  # Form with multiline
  - trigger: ":report"
    replace: |
      Title: {{form.title}}
      Details:
      {{form.details}}
    vars:
      - name: form
        type: form
        params:
          layout: |
            Title: [[title]]
            Details:
            [[details]]
          fields:
            details:
              multiline: true

  # Form with defaults and clipboard
  - trigger: ":review"
    replace: |
      Reviewed by: Donny
      Date: {{date}}
      Notes: {{form.notes}}
      Content:
      {{form.content}}
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
          layout: |
            Notes: [[notes]]
            Content:
            [[content]]
          fields:
            notes:
              placeholder: "Add your notes"
            content:
              multiline: true
              default: "{{clip}}"
```

## Developer Examples

File: `dev.yml`

```yaml
matches:
  # Git release workflow
  - trigger: "git!rel"
    replace: |
      git add . &&
      git commit -m "{{form.message}}" &&
      git push
    vars:
      - name: form
        type: form
        params:
          layout: "Commit message: [[message]]"
          fields:
            message:
              multiline: true

  # Docker compose
  - trigger: ":dcup"
    replace: "docker compose up -d"

  - trigger: ":dcdown"
    replace: "docker compose down"

  - trigger: ":dclogs"
    replace: "docker compose logs -f"

  # System commands
  - trigger: ":disk"
    replace: "{{disk}}"
    vars:
      - name: disk
        type: shell
        params:
          cmd: "df -h / | tail -1 | awk '{print $3 \"/\" $2 \" used (\" $5 \")\"}'"
```
