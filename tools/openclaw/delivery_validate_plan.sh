#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "$0")/lib.sh"

ensure_repo_root

usage() {
  echo "Usage: $0 <plan-dir>" >&2
  exit 1
}

if [ "$#" -ne 1 ]; then
  usage
fi

plan_dir="$1"
idea_file="${plan_dir}/IDEA.md"
plans_file="${plan_dir}/PLANS.md"
status_file="${plan_dir}/STATUS.md"

[ -d "$plan_dir" ] || {
  echo "Error: plan directory not found: $plan_dir" >&2
  exit 30
}

[ -f "$idea_file" ] || {
  echo "Error: missing plan file: $idea_file" >&2
  exit 31
}

[ -f "$plans_file" ] || {
  echo "Error: missing plan file: $plans_file" >&2
  exit 32
}

[ -f "$status_file" ] || {
  echo "Error: missing plan file: $status_file" >&2
  exit 33
}

status_value=$(frontmatter_value "$status_file" "status")
branch_value=$(frontmatter_value "$status_file" "branch")
current_branch_name=$(current_branch)

[ -n "$status_value" ] || {
  echo "Error: STATUS.md is missing frontmatter key: status" >&2
  exit 34
}

[ -n "$branch_value" ] || {
  echo "Error: STATUS.md is missing frontmatter key: branch" >&2
  exit 35
}

plan_ready=false

case "$status_value" in
  plan-refined|ready-for-codex|codex-running|review-ready|pr-created|done)
    plan_ready=true
    ;;
esac

branch_matches=false
if [ "$branch_value" = "$current_branch_name" ]; then
  branch_matches=true
fi

printf 'plan_dir=%s\n' "$plan_dir"
printf 'status=%s\n' "$status_value"
printf 'branch=%s\n' "$branch_value"
printf 'current_branch=%s\n' "$current_branch_name"
printf 'branch_matches=%s\n' "$branch_matches"
printf 'plan_ready=%s\n' "$plan_ready"
