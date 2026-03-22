#!/usr/bin/env bash
set -euo pipefail

# lib.sh - guard helpers for OpenClaw tools
# Verifies the script is run from the arqix repository root (PWD == git top level)
# Exits with non-zero codes and clear messages on failure.

ensure_repo_root() {
  # resolve git top-level
  repo_top=$(git rev-parse --show-toplevel 2>/dev/null || true)
  if [ -z "$repo_top" ]; then
    echo "Error: not inside a git repository." >&2
    return 2
  fi

  repo_name=$(basename "$repo_top")
  if [ "$repo_name" != "arqix" ]; then
    echo "Error: expected repository 'arqix' but found '$repo_name' (git top-level: $repo_top)." >&2
    return 3
  fi

  if [ "$(pwd)" != "$repo_top" ]; then
    echo "Error: this script must be run from the arqix repository root: $repo_top" >&2
    echo "Current directory: $(pwd)" >&2
    return 4
  fi

  return 0
}
