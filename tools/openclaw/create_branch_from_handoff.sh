#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "$0")/lib.sh"

ensure_repo_root

usage() {
  echo "Usage: $0 <handoff-path>" >&2
  exit 1
}

if [ "$#" -ne 1 ]; then
  usage
fi

handoff_path="$1"

[ -f "$handoff_path" ] || {
  echo "Error: handoff file not found: $handoff_path" >&2
  exit 40
}

if ! worktree_is_clean; then
  echo "Error: worktree has uncommitted changes; refusing to create or switch branches." >&2
  exit 41
fi

selected_branch=$(handoff_frontmatter_value "$handoff_path" "branch")

if [ -z "$selected_branch" ]; then
  selected_branch=$(./tools/openclaw/prepare_branch_name.sh --handoff "$handoff_path")
fi

[ -n "$selected_branch" ] || {
  echo "Error: unable to determine branch name from handoff: $handoff_path" >&2
  exit 42
}

if [ "$selected_branch" = "main" ]; then
  echo "Error: refusing to create or switch to protected branch: main" >&2
  exit 43
fi

if ! git check-ref-format --branch "$selected_branch" >/dev/null 2>&1; then
  echo "Error: invalid branch name: $selected_branch" >&2
  exit 44
fi

branch_existed=false

if local_branch_exists "$selected_branch"; then
  branch_existed=true
  git switch --quiet "$selected_branch"
else
  git switch --quiet -c "$selected_branch"
fi

printf 'handoff=%s\n' "$handoff_path"
printf 'branch=%s\n' "$selected_branch"
printf 'branch_existed=%s\n' "$branch_existed"
printf 'checkout_succeeded=true\n'
