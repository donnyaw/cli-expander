# cli-expander.zsh — CLI Expander ZSH plugin
# Source this file from .zshrc:
#   source /path/to/cli-expander.zsh
#
# Requires: cli-expander-cli in PATH

_cli_expander_buffer=""

_cli_expander_expand() {
    local input="$BUFFER"
    local result
    result=$(ce expand "$input" 2>/dev/null)
    if [ $? -eq 0 ] && [ -n "$result" ]; then
        BUFFER="$result"
        CURSOR=$#BUFFER
        _cli_expander_buffer=""
    fi
}

_cli_expander_self_insert() {
    local char=$KEYS
    _cli_expander_buffer+=$char
    zle .self-insert

    # Check if buffer ends with a known trigger pattern
    # Triggers typically start with : or end with !
    if [[ $_cli_expander_buffer == *:* ]] || [[ $_cli_expander_buffer == *!* ]]; then
        _cli_expander_expand
    fi

    # Reset buffer if space or enter was typed without match
    if [[ $char == ' ' ]] || [[ $char == $'\n' ]]; then
        _cli_expander_buffer=""
    fi
}

# Replace self-insert widget
zle -N self-insert _cli_expander_self_insert

# Bind Ctrl+T to manual expand
_cli_expander_manual_expand() {
    _cli_expander_expand
}
zle -N _cli_expander_manual_expand
bindkey '^T' _cli_expander_manual_expand
