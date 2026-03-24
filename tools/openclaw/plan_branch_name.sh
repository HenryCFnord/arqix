#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "$0")/lib.sh"

ensure_repo_root

usage() {
  echo "Usage: $0 --category <category> --slug <slug>" >&2
  exit 1
}

task_category=""
task_slug=""

while [ "$#" -gt 0 ]; do
  case "$1" in
    --category|--type)
      [ "$#" -ge 2 ] || usage
      task_category="$2"
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

if [ -z "$task_category" ] || [ -z "$task_slug" ]; then
  usage
fi

prefix=$(branch_prefix_for_category "$task_category") || {
  echo "Error: unsupported task category: $task_category" >&2
  exit 24
}

normalized_slug=$(slugify "$task_slug")

if [ -z "$normalized_slug" ]; then
  echo "Error: branch slug is empty after normalization." >&2
  exit 25
fi

printf '%s/%s\n' "$prefix" "$normalized_slug"
