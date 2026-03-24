#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "$0")/lib.sh"
ensure_repo_root

PLAN_DIR="docs/plans"

if [ ! -d "$PLAN_DIR" ]; then
  echo "No planning package directory found: $PLAN_DIR"
  exit 0
fi

find "$PLAN_DIR" -mindepth 1 -maxdepth 1 -type d | sort
