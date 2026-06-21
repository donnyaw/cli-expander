#!/usr/bin/env bash
set -euo pipefail

# Enable debug logging with: CE_TMUX_PICKER_DEBUG=1 ce-tmux-picker.sh <pane-id>
if [ "${CE_TMUX_PICKER_DEBUG:-}" = "1" ]; then
  exec 2>/tmp/ce-picker-debug.log
  set -x
fi

# Ensure ce and other tools are found in tmux popup context
export PATH="$HOME/.local/bin:$PATH"

target_pane="${1:-}"

if [ -z "$target_pane" ]; then
  printf 'ce-tmux-picker.sh: missing target pane id\n' >&2
  printf 'Usage: ce-tmux-picker.sh <tmux-pane-id>\n' >&2
  exit 2
fi

if [[ "$target_pane" == *'#{'* ]]; then
  printf 'ce-tmux-picker.sh: tmux pane id was not expanded: %s\n' "$target_pane" >&2
  printf 'Reload integrations/tmux/cli-expander.tmux and try prefix + Ctrl+e again.\n' >&2
  exit 2
fi

printf "Target pane: %s\n" "$target_pane" >&2

for cmd in tmux ce fzf python3; do
  if ! command -v "$cmd" >/dev/null 2>&1; then
    printf 'ce-tmux-picker.sh: required command not found: %s\n' "$cmd" >&2
    exit 2
  fi
done

if ! tmux display-message -t "$target_pane" -p '#{pane_id}' >/dev/null 2>&1; then
  printf 'ce-tmux-picker.sh: invalid tmux target pane: %s\n' "$target_pane" >&2
  exit 2
fi

printf "All checks passed\n" >&2

rows=$(ce list --json | python3 -c '
import json
import sys

for row in json.load(sys.stdin):
    trigger = str(row.get("trigger", ""))
    description = str(row.get("description", ""))
    category = str(row.get("category", ""))
    if trigger:
        print("\t".join(part.replace("\t", " ") for part in (trigger, description, category)))
')

if [ -z "$rows" ]; then
  printf 'ce-tmux-picker.sh: no triggers available\n' >&2
  exit 1
fi

selected=$(printf '%s\n' "$rows" | fzf \
  --delimiter=$'\t' \
  --with-nth=1,2 \
  --nth=1,2 \
  --preview='$HOME/.local/bin/ce details {1} 2>/dev/null' \
  --preview-window=right:60%:wrap \
  --header='Enter: inject into selected pane | Esc: cancel' \
  --height=50% \
  --min-height=10) || exit 0

if [ -z "$selected" ]; then
  exit 0
fi

trigger="${selected%%$'\t'*}"

if [ -z "$trigger" ]; then
  printf 'ce-tmux-picker.sh: selected row did not contain a trigger\n' >&2
  exit 1
fi

set +e
"$HOME/.local/bin/ce" expand "$trigger" --output tmux --target-pane "$target_pane" 2>/tmp/ce-picker-error.log
exit_code=$?
set -e

if [ $exit_code -ne 0 ]; then
  printf "\nExpand FAILED (exit code: %d)\n" $exit_code >&2
  printf "See /tmp/ce-picker-error.log for details\n" >&2
  sleep 4
  exit 1
fi

printf "\nExpanded '%s' into pane %s\n" "$trigger" "$target_pane" >&2
