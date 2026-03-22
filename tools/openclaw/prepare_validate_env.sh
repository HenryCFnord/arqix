#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "$0")/lib.sh"

ensure_repo_root

blockers=0
warnings=0

check_required_command() {
  local command_name="$1"

  if require_command "$command_name" >/dev/null 2>&1; then
    echo "OK: command available: $command_name"
  else
    echo "BLOCKER: required command not found: $command_name" >&2
    blockers=$((blockers + 1))
  fi
}

check_required_path() {
  local path="$1"
  local description="$2"

  if [ -e "$path" ]; then
    echo "OK: ${description}: ${path}"
  else
    echo "BLOCKER: missing ${description}: ${path}" >&2
    blockers=$((blockers + 1))
  fi
}

echo "Repository: arqix"
echo "Repository root: $(pwd)"

check_required_command git
check_required_command gh
check_required_command codex

branch=$(current_branch)
echo "Current branch: ${branch}"

ensure_not_main_branch || exit 11
echo "OK: branch is not main"

if worktree_is_clean; then
  echo "OK: worktree is clean"
else
  echo "WARNING: worktree has uncommitted changes" >&2
  warnings=$((warnings + 1))
fi

ISSUE_TEMPLATE=".github/ISSUE_TEMPLATE/task-from-handoff.md"
check_required_path "$ISSUE_TEMPLATE" "issue template"

HANDOFF_DIR="docs/handoffs/approved"
if [ -d "$HANDOFF_DIR" ]; then
  echo "OK: approved handoff directory exists: $HANDOFF_DIR"
  if [ -n "$(find "$HANDOFF_DIR" -mindepth 1 -maxdepth 1 -print -quit)" ]; then
    echo "OK: approved handoff directory is non-empty"
  else
    echo "WARNING: approved handoff directory is empty: $HANDOFF_DIR" >&2
    warnings=$((warnings + 1))
  fi
else
  echo "BLOCKER: missing approved handoff directory: $HANDOFF_DIR" >&2
  blockers=$((blockers + 1))
fi

echo "Validation complete (non-executing)."
echo "Blockers: $blockers"
echo "Warnings: $warnings"

if [ "$blockers" -gt 0 ]; then
  exit 12
fi
