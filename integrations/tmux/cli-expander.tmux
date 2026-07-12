# cli-expander tmux integration
# Source from .tmux.conf with:
#   run-shell /path/to/cli-expander/integrations/tmux/cli-expander.tmux

# prefix + e : prompt to type a trigger and expand it inline in the current pane
bind-key e command-prompt -p "cli-expander:" "run-shell 'ce expand \"%%\" --output tmux'"

# prefix + Ctrl+g : open fzf popup picker to browse and select a trigger
# Capture #{pane_id} before creating the popup so injection targets the original pane.
bind-key C-g run-shell 'tmux display-popup -E -T cli-expander -w 90% -h 80% "ce-tmux-picker.sh \"#{pane_id}\""'

# prefix + ] : keep normal tmux paste everywhere, but paste safely inside
# cli-expander forms by sending Ctrl+V so the form reads tmux show-buffer itself.
bind-key ] if-shell -F '#{||:#{==:#{pane_current_command},ce},#{==:#{pane_current_command},cli-expander}}' 'send-keys C-v' 'paste-buffer'

# Direct examples for users to customize:
# bind-key C-h run-shell "ce expand ':hello' --output tmux --target-pane '#{pane_id}'"
