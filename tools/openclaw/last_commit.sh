#!/usr/bin/env bash
set -euo pipefail

# guard: must be run from repo root
source "$(dirname "$0")/lib.sh"
ensure_repo_root

git log -1 --stat --decorate
