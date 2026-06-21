#!/usr/bin/env bash
set -euo pipefail

cat >&2 <<'EOF'
ce-tmux-picker.sh is planned for P14-06 and is not implemented yet.

For now, use direct tmux output mode:
  ce expand ":hello" --output tmux --target-pane "$TMUX_PANE"
EOF

exit 2
