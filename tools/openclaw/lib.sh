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

require_command() {
  local command_name="$1"

  if ! command -v "$command_name" >/dev/null 2>&1; then
    echo "Error: required command not found: $command_name" >&2
    return 10
  fi
}

current_branch() {
  git branch --show-current
}

ensure_not_main_branch() {
  local branch
  branch=$(current_branch)

  if [ "$branch" = "main" ]; then
    echo "Error: this action must not run from branch 'main'." >&2
    return 11
  fi
}

worktree_is_clean() {
  [ -z "$(git status --short)" ]
}

local_branch_exists() {
  local branch_name="$1"
  git rev-parse --verify --quiet "refs/heads/${branch_name}" >/dev/null
}

slugify() {
  printf '%s\n' "$1" \
    | tr '[:upper:]' '[:lower:]' \
    | sed -E 's/[^a-z0-9]+/-/g; s/^-+//; s/-+$//; s/-+/-/g'
}

handoff_frontmatter_value() {
  local handoff_path="$1"
  local key="$2"

  awk -v key="$key" '
    BEGIN { in_frontmatter = 0 }
    /^---$/ {
      if (in_frontmatter == 0) {
        in_frontmatter = 1
        next
      }
      exit
    }
    in_frontmatter == 1 && $0 ~ ("^" key ":") {
      sub("^" key ":[[:space:]]*", "", $0)
      gsub(/^"/, "", $0)
      gsub(/"$/, "", $0)
      print
      exit
    }
  ' "$handoff_path"
}

extract_markdown_section() {
  local handoff_path="$1"
  local section_name="$2"

  awk -v target="## ${section_name}" '
    $0 == target {
      in_section = 1
      next
    }
    /^## / && in_section == 1 {
      exit
    }
    in_section == 1 {
      print
    }
  ' "$handoff_path"
}
