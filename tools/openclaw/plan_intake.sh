#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "$0")/lib.sh"

ensure_repo_root

usage() {
  cat >&2 <<'EOF'
Usage:
  ./tools/openclaw/plan_intake.sh \
    --category <category> \
    --slug <slug> \
    --title <title> \
    --idea <text> \
    [--source <source>] \
    [--base <branch>] \
    [--commit]
EOF
  exit 1
}

task_category=""
task_slug=""
task_title=""
idea_text=""
idea_source="openclaw"
base_branch="main"
create_commit=false

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
    --title)
      [ "$#" -ge 2 ] || usage
      task_title="$2"
      shift 2
      ;;
    --idea)
      [ "$#" -ge 2 ] || usage
      idea_text="$2"
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

if [ -z "$task_category" ] || [ -z "$task_slug" ] || [ -z "$task_title" ] || [ -z "$idea_text" ]; then
  usage
fi

require_clean_worktree || exit 12

branch_name=$(./tools/openclaw/plan_branch_name.sh --category "$task_category" --slug "$task_slug")
plan_slug=$(slugify "$task_slug")
plan_dir="docs/plans/${plan_slug}"

if [ -d "$plan_dir" ]; then
  echo "Error: planning package already exists: $plan_dir" >&2
  exit 26
fi

if [ "$branch_name" = "main" ]; then
  echo "Error: refusing to create or switch to protected branch: main" >&2
  exit 27
fi

if ! git check-ref-format --branch "$branch_name" >/dev/null 2>&1; then
  echo "Error: invalid branch name: $branch_name" >&2
  exit 28
fi

if ! ref_exists "$base_branch"; then
  echo "Error: base branch or ref not found: $base_branch" >&2
  exit 29
fi

branch_existed=false

if local_branch_exists "$branch_name"; then
  branch_existed=true
  git switch --quiet "$branch_name"
else
  git switch --quiet -c "$branch_name" "$base_branch"
fi

mkdir -p "$plan_dir"

created_at=$(iso_timestamp_utc)
latest_commit=$(git rev-parse --short HEAD)

cat > "${plan_dir}/IDEA.md" <<EOF
---
title: "${task_title}"
date: ${created_at}
status: draft-created
category: ${task_category}
branch: ${branch_name}
source: ${idea_source}
---

# ${task_title}

## Raw idea

${idea_text}

## Intake metadata

- Intake source: ${idea_source}
- Created: ${created_at}
- Chosen branch: \`${branch_name}\`
- Category: \`${task_category}\`
EOF

cat > "${plan_dir}/PLANS.md" <<EOF
---
title: "${task_title} plan"
date: ${created_at}
status: draft-created
category: ${task_category}
branch: ${branch_name}
---

# ${task_title} plan

## Summary

Draft plan created from the original idea. Review and refine this file before Codex starts implementation.

## Goal

-

## Out of scope

-

## Context

- Source idea: [IDEA.md](IDEA.md)

## Proposed approach

-

## Constraints

- Stay within the reviewed plan scope.

## Acceptance criteria

- [ ] Goal is explicit and reviewable.
- [ ] Out of scope is clear enough to avoid side quests.
- [ ] Implementation steps are ready for Codex.

## Risks and unknowns

-

## Execution steps

1. Review the idea and refine the scope.
2. Update this plan until it is ready for Codex.
3. Set \`STATUS.md\` to \`plan-refined\` or \`ready-for-codex\` when review is complete.
EOF

cat > "${plan_dir}/STATUS.md" <<EOF
---
title: "${task_title} status"
date: ${created_at}
status: awaiting-human-review
branch: ${branch_name}
plan_dir: ${plan_dir}
---

# ${task_title} status

## Current status

awaiting-human-review

## Branch

\`${branch_name}\`

## Latest commit reference

\`${latest_commit}\`

## Related files

- [IDEA.md](IDEA.md)
- [PLANS.md](PLANS.md)

## Next review action

Review and refine \`PLANS.md\`, then update the status to \`plan-refined\` or \`ready-for-codex\`.

## Later PR link

- Pending

## Later implementation summary

- Pending
EOF

git add "${plan_dir}/IDEA.md" "${plan_dir}/PLANS.md" "${plan_dir}/STATUS.md"

commit_created=false
commit_sha=""

if [ "$create_commit" = true ]; then
  git commit --quiet -m "chore: add draft plan package for ${plan_slug}"
  commit_created=true
  commit_sha=$(git rev-parse --short HEAD)
fi

printf 'branch=%s\n' "$branch_name"
printf 'branch_existed=%s\n' "$branch_existed"
printf 'plan_dir=%s\n' "$plan_dir"
printf 'idea_file=%s\n' "${plan_dir}/IDEA.md"
printf 'plans_file=%s\n' "${plan_dir}/PLANS.md"
printf 'status_file=%s\n' "${plan_dir}/STATUS.md"
printf 'commit_created=%s\n' "$commit_created"
if [ -n "$commit_sha" ]; then
  printf 'commit_sha=%s\n' "$commit_sha"
fi
