#!/usr/bin/env bash
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

echo "Repository: $(basename "$(pwd)")"
echo "Branch: $(git branch --show-current)"
echo
echo "Status:"
git status --short