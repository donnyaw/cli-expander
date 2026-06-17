# cli-expander.fish — CLI Expander Fish plugin
# Source this file from config.fish:
#   source /path/to/cli-expander.fish
#
# Requires: cli-expander-cli in PATH

function _cli_expander_expand --description "Expand trigger in current command line"
    set -l input (commandline)
    set -l result (ce expand "$input" 2>/dev/null)
    if test $status -eq 0 -a -n "$result"
        commandline -r "$result"
    end
end

# Bind Ctrl+T to manual expand
bind \ct _cli_expander_expand

# Automatic expansion hook on space
function _cli_expander_space --description "Expand on space"
    _cli_expander_expand
    commandline -i " "
end

bind \x20 _cli_expander_space
