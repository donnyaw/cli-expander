# Shell Integration Guide

Shell plugins let you use terminal-expander directly from your shell prompt. Type a trigger and press the expand keybinding.

## Available Plugins

| Shell | File | Features |
|-------|------|----------|
| Zsh | `shell/texpand.zsh` | self-insert widget hook, buffer tracking, manual expand |
| Bash | `shell/texpand.bash` | bind -x, READLINE_LINE manipulation, accept-line hook |
| Fish | `shell/texpand.fish` | commandline -r, space-triggered expand, Ctrl+T binding |

## Installation

### Zsh

Add to `~/.zshrc`:

```zsh
source /path/to/shell/texpand.zsh
```

### Bash

Add to `~/.bashrc`:

```bash
source /path/to/shell/texpand.bash
```

### Fish

Add to `~/.config/fish/config.fish`:

```fish
source /path/to/shell/texpand.fish
```

## Usage

### Manual Expansion (Ctrl+T)

Type a trigger in your shell prompt and press `Ctrl+T` to expand it.

```
$ Say :hello to everyone
                  ^ press Ctrl+T
$ Say Hello World! to everyone
```

### Automatic Expansion (Zsh)

The Zsh plugin hooks into `self-insert`. It watches for trigger patterns as you type and expands automatically when it detects a trigger followed by a word boundary.

### Auto-Expand on Space (Fish)

The Fish plugin expands when you press Space after a trigger.

## How It Works

The shell plugin:
1. Maintains a rolling buffer of typed characters
2. When you press the expand keybinding, it passes the current buffer to `texpand expand`
3. If expansion succeeds, it replaces the buffer with the result
4. For form-based triggers, it spawns the Cursive TUI within the terminal

## Troubleshooting

**Q: Expansion doesn't work in my shell**
A: Make sure `texpand` is in your PATH. Test with `texpand expand ":hello"`.

**Q: Ctrl+T is already bound**
A: Edit the plugin file and change `^T` to another keybinding like `^E`.

**Q: Form doesn't open when triggered**
A: Forms need a real terminal. Make sure you're not piping output and your `TERM` variable is set.

**Q: Zsh plugin conflicts with other plugins**
A: The plugin uses `zle -N self-insert` which chains properly with zsh-autosuggestions and zsh-syntax-highlighting. If conflicts occur, load texpand last.
