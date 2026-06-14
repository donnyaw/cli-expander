# Getting Started

A 5-minute guide to using terminal-expander.

## Step 1: Install

```bash
cd terminal-expander
cargo build --release
cp target/release/texpand ~/.local/bin/
export PATH="$PATH:$HOME/.local/bin"
```

## Step 2: Create Match Files

```bash
mkdir -p ~/.config/texpand/matches

cat > ~/.config/texpand/matches/base.yml << 'EOF'
matches:
  - trigger: ":hello"
    replace: "Hello World!"
  - trigger: ":now"
    replace: "{{time}}"
    vars:
      - name: time
        type: date
        params:
          format: "%H:%M"
EOF
```

## Step 3: Test Expansion

```bash
texpand expand ":hello"
# Output: Hello World!

texpand expand ":now"
# Output: 14:30
```

## Step 4: Add a Form

```bash
cat >> ~/.config/texpand/matches/base.yml << 'EOF'
  - trigger: ":greet"
    form: |
      Hello [[name]]!
    form_fields:
      name:
        placeholder: "Your name"
EOF

texpand expand ":greet"
# Opens interactive form in terminal
```

## Step 5: Try Shell Integration

```bash
# Add to your shell config
echo 'source /path/to/shell/texpand.bash' >> ~/.bashrc

# Restart shell, then:
# Type :hello and press Ctrl+T
```

## What's Next?

- Read the [User Guide](user-guide.md) for detailed usage
- See [Form Syntax](../reference/forms.md) for form field types
- Browse [Example Match Files](../examples/README.md)
- Configure [Shell Integration](shell-integration.md) for in-prompt expansion
