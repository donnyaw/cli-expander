# cli-expander tmux integration
# Source from .tmux.conf with:
#   run-shell /path/to/cli-expander/integrations/tmux/cli-expander.tmux

bind-key C-e display-popup -E "ce-tmux-picker.sh '#{pane_id}'"

# Direct examples for users to customize:
# bind-key C-h run-shell "ce expand ':hello' --output tmux --target-pane '#{pane_id}'"
