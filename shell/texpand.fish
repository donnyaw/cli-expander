# texpand.fish — Terminal Expander Fish plugin
# Source this file from config.fish:
#   source /path/to/texpand.fish
#
# Requires: texpand-cli in PATH

function _texpand_expand --description "Expand trigger in current command line"
    set -l input (commandline)
    set -l result (texpand expand "$input" 2>/dev/null)
    if test $status -eq 0 -a -n "$result"
        commandline -r "$result"
    end
end

# Bind Ctrl+T to manual expand
bind \ct _texpand_expand

# Automatic expansion hook on space
function _texpand_space --description "Expand on space"
    _texpand_expand
    commandline -i " "
end

bind \x20 _texpand_space
