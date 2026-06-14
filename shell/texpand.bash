# texpand.bash — Terminal Expander Bash plugin
# Source this file from .bashrc:
#   source /path/to/texpand.bash
#
# Type :hello and press Space or Ctrl+T — auto-expands in-place like Espanso.

_texpand_cmd="te"

_texpand_expand() {
    local input="$READLINE_LINE"
    local point="$READLINE_POINT"
    local result
    result=$($_texpand_cmd "$input" 2>/dev/null)
    if [ $? -eq 0 ] && [ -n "$result" ]; then
        READLINE_LINE="$result"
        READLINE_POINT=${#READLINE_LINE}
    fi
}

# Called after space is inserted — checks if there's a trigger before it
_texpand_on_space() {
    _texpand_expand
    READLINE_LINE+=" "
    READLINE_POINT=${#READLINE_LINE}
}

# Bind space → expand, then insert space
bind -x '" ": _texpand_on_space'

# Bind Ctrl+T → manual expand
bind -x '"\C-t": _texpand_expand'
