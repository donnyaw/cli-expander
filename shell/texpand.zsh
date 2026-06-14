# texpand.zsh — Terminal Expander ZSH plugin
# Source this file from .zshrc:
#   source /path/to/texpand.zsh
#
# Requires: texpand-cli in PATH

_texpand_buffer=""

_texpand_expand() {
    local input="$BUFFER"
    local result
    result=$(texpand expand "$input" 2>/dev/null)
    if [ $? -eq 0 ] && [ -n "$result" ]; then
        BUFFER="$result"
        CURSOR=$#BUFFER
        _texpand_buffer=""
    fi
}

_texpand_self_insert() {
    local char=$KEYS
    _texpand_buffer+=$char
    zle .self-insert

    # Check if buffer ends with a known trigger pattern
    # Triggers typically start with : or end with !
    if [[ $_texpand_buffer == *:* ]] || [[ $_texpand_buffer == *!* ]]; then
        _texpand_expand
    fi

    # Reset buffer if space or enter was typed without match
    if [[ $char == ' ' ]] || [[ $char == $'\n' ]]; then
        _texpand_buffer=""
    fi
}

# Replace self-insert widget
zle -N self-insert _texpand_self_insert

# Bind Ctrl+T to manual expand
_texpand_manual_expand() {
    _texpand_expand
}
zle -N _texpand_manual_expand
bindkey '^T' _texpand_manual_expand
