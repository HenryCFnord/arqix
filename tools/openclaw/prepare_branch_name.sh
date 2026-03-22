#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "$0")/lib.sh"

ensure_repo_root

usage() {
  echo "Usage:" >&2
  echo "  $0 --type <type> --slug <slug>" >&2
  echo "  $0 --handoff <path>" >&2
  exit 1
}

branch_prefix_for_type() {
  case "$1" in
    feature) echo "feat" ;;
    bugfix) echo "fix" ;;
    refactor) echo "refactor" ;;
    docs) echo "docs" ;;
    chore) echo "chore" ;;
    report) echo "report" ;;
    blog) echo "blog" ;;
    *) return 20 ;;
  esac
}

handoff_path=""
task_type=""
task_slug=""

while [ "$#" -gt 0 ]; do
  case "$1" in
    --handoff)
      [ "$#" -ge 2 ] || usage
      handoff_path="$2"
      shift 2
      ;;
    --type)
      [ "$#" -ge 2 ] || usage
      task_type="$2"
      shift 2
      ;;
    --slug)
      [ "$#" -ge 2 ] || usage
      task_slug="$2"
      shift 2
      ;;
    *)
      usage
      ;;
  esac
done

if [ -n "$handoff_path" ]; then
  [ -f "$handoff_path" ] || {
    echo "Error: handoff file not found: $handoff_path" >&2
    exit 21
  }

  if [ -n "$task_type" ] || [ -n "$task_slug" ]; then
    echo "Error: use either --handoff or --type/--slug, not both." >&2
    exit 22
  fi

  task_type=$(handoff_frontmatter_value "$handoff_path" "type")
  task_slug=$(handoff_frontmatter_value "$handoff_path" "branch")

  if [ -n "$task_slug" ]; then
    printf '%s\n' "$task_slug"
    exit 0
  fi

  task_slug=$(handoff_frontmatter_value "$handoff_path" "title")

  if [ -z "$task_type" ] || [ -z "$task_slug" ]; then
    echo "Error: handoff must provide type and title or branch metadata." >&2
    exit 23
  fi
fi

if [ -z "$task_type" ] || [ -z "$task_slug" ]; then
  usage
fi

prefix=$(branch_prefix_for_type "$task_type") || {
  echo "Error: unsupported task type: $task_type" >&2
  exit 24
}

normalized_slug=$(slugify "$task_slug")

if [ -z "$normalized_slug" ]; then
  echo "Error: branch slug is empty after normalization." >&2
  exit 25
fi

printf '%s/%s\n' "$prefix" "$normalized_slug"
