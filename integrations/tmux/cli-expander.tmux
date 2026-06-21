# cli-expander tmux integration
# Source from .tmux.conf with:
#   run-shell /path/to/cli-expander/integrations/tmux/cli-expander.tmux

# prefix + e : prompt to type a trigger and expand it inline in the current pane
# Uses full path to ce because tmux command-prompt runs as a tmux command, not a shell
bind-key e command-prompt -p "cli-expander:" "run-shell '/home/rezriz/.local/bin/ce expand \"%%\" --output tmux'"

# prefix + Ctrl+e : open fzf popup picker to browse and select a trigger
bind-key C-e display-popup "ce-tmux-picker.sh '#{pane_id}'"

# Direct examples for users to customize:
# bind-key C-h run-shell "ce expand ':hello' --output tmux --target-pane '#{pane_id}'"
