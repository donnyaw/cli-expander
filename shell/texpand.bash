# texpand.bash — Terminal Expander Bash plugin
# Source this file from .bashrc:
#   source /path/to/texpand.bash
#
# Type :hello and press Space or Ctrl+T — expands inline.
# For forms, use te :ticket directly (opens TUI in terminal).

_texpand_cmd="te"

_texpand_expand() {
    local input="$READLINE_LINE"
    local result
    result=$($_texpand_cmd "$input" 2>/dev/null)
    if [ $? -eq 0 ] && [ -n "$result" ]; then
        READLINE_LINE="$result"
        READLINE_POINT=${#READLINE_LINE}
    fi
}

# Called on Space — expand first, then insert space
_texpand_on_space() {
    _texpand_expand
    READLINE_LINE+=" "
    READLINE_POINT=${#READLINE_LINE}
}

# Bind space → expand, then insert space
bind -x '" ": _texpand_on_space'

# Bind Ctrl+T → manual expand
bind -x '"\C-t": _texpand_expand'
