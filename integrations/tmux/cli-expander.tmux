# cli-expander tmux integration
# Source from .tmux.conf with:
#   run-shell /path/to/cli-expander/integrations/tmux/cli-expander.tmux

# prefix + e : prompt to type a trigger and expand it inline in the current pane
bind-key e command-prompt -p "cli-expander:" "run-shell 'ce expand \"%%\" --output tmux'"

# prefix + Ctrl+e : open fzf popup picker to browse and select a trigger
# Capture #{pane_id} before creating the popup so injection targets the original pane.
bind-key C-e run-shell 'tmux display-popup -E -T cli-expander -w 90% -h 80% "ce-tmux-picker.sh \"#{pane_id}\""'

# Direct examples for users to customize:
# bind-key C-h run-shell "ce expand ':hello' --output tmux --target-pane '#{pane_id}'"
