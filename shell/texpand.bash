# texpand.bash — Terminal Expander Bash plugin
# Source this file from .bashrc:
#   source /path/to/texpand.bash
#
# :hello[space]   → auto-expands inline (text only)
# te:hello[Enter] → runs te :hello (no space needed)
# te :ticket[Enter] → opens form TUI

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

_texpand_on_space() {
    _texpand_expand
    READLINE_LINE+=" "
    READLINE_POINT=${#READLINE_LINE}
}

# Allow te:hello[Enter] — intercepts command_not_found
# Falls back to system /usr/lib/command-not-found for other unknown commands
command_not_found_handle() {
    if [[ "$1" == te:* ]]; then
        te ":${1#te:}"
        return $?
    fi
    if [ -x /usr/lib/command-not-found ]; then
        /usr/lib/command-not-found -- "$1"
        return $?
    fi
    if [ -x /usr/share/command-not-found/command-not-found ]; then
        /usr/share/command-not-found/command-not-found -- "$1"
        return $?
    fi
    echo "bash: $1: command not found" >&2
    return 127
}

bind -x '" ": _texpand_on_space'
bind -x '"\C-t": _texpand_expand'
