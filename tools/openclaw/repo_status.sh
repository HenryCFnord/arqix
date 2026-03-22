#!/usr/bin/env bash
set -euo pipefail

# guard: must be run from repo root
source "$(dirname "$0")/lib.sh"
ensure_repo_root

# now safe to proceed

echo "Repository: $(basename "$(pwd)")"
echo "Branch: $(git branch --show-current)"
echo
echo "Status:"
git status --short
