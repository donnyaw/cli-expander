#!/usr/bin/env bash
set -euo pipefail

target_pane="${1:-}"

if [ -z "$target_pane" ]; then
  printf 'ce-tmux-picker.sh: missing target pane id\n' >&2
  printf 'Usage: ce-tmux-picker.sh <tmux-pane-id>\n' >&2
  exit 2
fi

for cmd in tmux ce fzf python3; do
  if ! command -v "$cmd" >/dev/null 2>&1; then
    printf 'ce-tmux-picker.sh: required command not found: %s\n' "$cmd" >&2
    exit 2
  fi
done

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
  --preview='ce details {1} 2>/dev/null' \
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

ce expand "$trigger" --output tmux --target-pane "$target_pane"
