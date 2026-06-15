# texpand.bash — Terminal Expander Bash plugin
# Source this file from .bashrc:
#   source /path/to/texpand.bash
#
# Usage:
#   :hello[Space]   → expand inline into the current prompt
#   :find[Space]    → open form TUI, then insert generated command into prompt
#   Ctrl+T          → manually expand the current prompt buffer
#
# Space is the primary workflow. Press Enter only after the expansion is
# visible in the prompt and you are ready to execute it.

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

# Avoid dumping generated command text above the prompt. Use Space/Ctrl+T so the
# expansion lands in the editable command line first.
command_not_found_handle() {
    if [[ "$1" == :* ]]; then
        printf 'texpand: use %s[Space] to expand into the prompt, then press Enter to run it.\n' "$1" >&2
        return 127
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
