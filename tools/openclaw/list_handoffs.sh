#!/usr/bin/env bash
set -euo pipefail

# guard: must be run from repo root
source "$(dirname "$0")/lib.sh"
ensure_repo_root

HANDOFF_DIR="docs/handoffs/approved"

if [ ! -d "$HANDOFF_DIR" ]; then
  echo "No approved handoff directory found: $HANDOFF_DIR"
  exit 0
fi

find "$HANDOFF_DIR" -maxdepth 1 -type f \( -name "*.md" -o -name "*.markdown" \) | sort
