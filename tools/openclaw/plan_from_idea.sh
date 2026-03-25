#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "$0")/lib.sh"

ensure_repo_root

usage() {
  cat >&2 <<'EOF'
Usage:
  ./tools/openclaw/plan_from_idea.sh \
    --idea <text> \
    [--category <category>] \
    [--title <title>] \
    [--slug <slug>] \
    [--source <source>] \
    [--base <branch>] \
    [--commit]
EOF
  exit 1
}

idea_text=""
task_category=""
task_title=""
task_slug=""
idea_source="openclaw"
base_branch="main"
create_commit=false

while [ "$#" -gt 0 ]; do
  case "$1" in
    --idea)
      [ "$#" -ge 2 ] || usage
      idea_text="$2"
      shift 2
      ;;
    --category|--type)
      [ "$#" -ge 2 ] || usage
      task_category="$2"
      shift 2
      ;;
    --title)
      [ "$#" -ge 2 ] || usage
      task_title="$2"
      shift 2
      ;;
    --slug)
      [ "$#" -ge 2 ] || usage
      task_slug="$2"
      shift 2
      ;;
    --source)
      [ "$#" -ge 2 ] || usage
      idea_source="$2"
      shift 2
      ;;
    --base)
      [ "$#" -ge 2 ] || usage
      base_branch="$2"
      shift 2
      ;;
    --commit)
      create_commit=true
      shift
      ;;
    *)
      usage
      ;;
  esac
done

if [ -z "$idea_text" ]; then
  usage
fi

normalized_idea=$(trim_whitespace "$idea_text")

if [ -z "$normalized_idea" ]; then
  echo "Error: idea text is empty after normalization." >&2
  exit 30
fi

if [ -z "$task_title" ]; then
  task_title=$(derive_title_from_idea "$normalized_idea") || {
    echo "Error: unable to derive a title from the idea text." >&2
    exit 31
  }
fi

if [ -z "$task_slug" ]; then
  task_slug=$(slugify "$task_title")
fi

if [ -z "$task_slug" ]; then
  echo "Error: unable to derive a usable slug from the idea text." >&2
  exit 32
fi

if [ -z "$task_category" ]; then
  printf 'needs_category=true\n'
  printf 'suggested_title=%s\n' "$task_title"
  printf 'suggested_slug=%s\n' "$task_slug"
  printf 'allowed_categories=feat,fix,refactor,docs,blog,report,chore\n'
  printf 'follow_up_prompt=%s\n' "Choose one category: feat, fix, refactor, docs, blog, report, or chore."
  exit 0
fi

branch_name=$(./tools/openclaw/plan_branch_name.sh --category "$task_category" --slug "$task_slug")

printf 'needs_category=false\n'
printf 'suggested_title=%s\n' "$task_title"
printf 'suggested_slug=%s\n' "$task_slug"

plan_intake_args=(
  --category "$task_category"
  --slug "$task_slug"
  --title "$task_title"
  --idea "$normalized_idea"
  --source "$idea_source"
  --base "$base_branch"
)

if [ "$create_commit" = true ]; then
  plan_intake_args+=(--commit)
fi

./tools/openclaw/plan_intake.sh "${plan_intake_args[@]}"
