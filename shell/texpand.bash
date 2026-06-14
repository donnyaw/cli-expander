# texpand.bash — Terminal Expander Bash plugin
# Source this file from .bashrc:
#   source /path/to/texpand.bash
#
# Requires: texpand-cli in PATH

_texpand_buffer=""

_texpand_expand() {
    local input="$READLINE_LINE"
    local result
    result=$(texpand expand "$input" 2>/dev/null)
    if [ $? -eq 0 ] && [ -n "$result" ]; then
        READLINE_LINE="$result"
        READLINE_POINT=${#READLINE_LINE}
        _texpand_buffer=""
    fi
}

# Check for expansion after space or enter
_texpand_preexec_check() {
    local typed="${READLINE_LINE:0:READLINE_POINT}"
    if [[ "$typed" == *:* ]] || [[ "$typed" == *!* ]]; then
        _texpand_expand
    fi
}

# Bind Ctrl+T to manual expand
bind -x '"\C-t": _texpand_expand'

# Check on accept-line (Enter key)
_texpand_accept_line() {
    _texpand_expand
    builtin eval "$READLINE_LINE"
}
