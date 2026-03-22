#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "$0")/lib.sh"

ensure_repo_root

if [ "$#" -ne 1 ]; then
  echo "Usage: $0 <handoff-path>" >&2
  exit 1
fi

handoff_path="$1"

[ -f "$handoff_path" ] || {
  echo "Error: handoff file not found: $handoff_path" >&2
  exit 30
}

handoff_id=$(handoff_frontmatter_value "$handoff_path" "id")
handoff_status=$(handoff_frontmatter_value "$handoff_path" "status")
handoff_title=$(handoff_frontmatter_value "$handoff_path" "title")

[ -n "$handoff_id" ] || {
  echo "Error: handoff is missing frontmatter key: id" >&2
  exit 31
}

[ -n "$handoff_status" ] || {
  echo "Error: handoff is missing frontmatter key: status" >&2
  exit 32
}

[ -n "$handoff_title" ] || {
  echo "Error: handoff is missing frontmatter key: title" >&2
  exit 33
}

branch_proposal=$(./tools/openclaw/prepare_branch_name.sh --handoff "$handoff_path")
summary=$(extract_markdown_section "$handoff_path" "Summary" | sed '/^[[:space:]]*$/d')
goal=$(extract_markdown_section "$handoff_path" "Goal")
out_of_scope=$(extract_markdown_section "$handoff_path" "Out of Scope")
acceptance_criteria=$(extract_markdown_section "$handoff_path" "Acceptance Criteria")
test_expectations=$(extract_markdown_section "$handoff_path" "Test Expectations")
risks_and_unknowns=$(extract_markdown_section "$handoff_path" "Risks and Unknowns")

[ -n "$summary" ] || {
  echo "Error: handoff section is empty or missing: Summary" >&2
  exit 34
}

[ -n "$goal" ] || {
  echo "Error: handoff section is empty or missing: Goal" >&2
  exit 35
}

[ -n "$acceptance_criteria" ] || {
  echo "Error: handoff section is empty or missing: Acceptance Criteria" >&2
  exit 36
}

printf '## Summary\n\n%s\n\n' "$summary"
printf '## Goal\n\n%s\n\n' "$goal"
printf '## Source handoff\n\n'
printf -- '- Handoff ID: %s\n' "$handoff_id"
printf -- '- Path: %s\n' "$handoff_path"
printf -- '- Status: %s\n\n' "$handoff_status"
printf '## Scope\n\n'
printf -- '- In scope:\n'
printf '%s\n\n' "$goal"
printf -- '- Out of scope:\n'
if [ -n "$out_of_scope" ]; then
  printf '%s\n\n' "$out_of_scope"
else
  printf -- '- Not specified in handoff\n\n'
fi
printf '## Acceptance criteria\n\n%s\n\n' "$acceptance_criteria"
printf '## Technical context\n\n'
printf -- '- Relevant modules:\n'
printf -- '- Relevant documents: %s\n' "$handoff_path"
printf -- '- Related issue/ADR/PR:\n\n'
printf '## Test expectations\n\n'
if [ -n "$test_expectations" ]; then
  printf '%s\n\n' "$test_expectations"
else
  printf -- '- Manual verification\n\n'
fi
printf '## Risks / unknowns\n\n'
if [ -n "$risks_and_unknowns" ]; then
  printf '%s\n\n' "$risks_and_unknowns"
else
  printf -- '- None captured in handoff\n\n'
fi
printf '## Branch proposal\n\n`%s`\n\n' "$branch_proposal"
printf '## Notes for implementation\n\n'
printf -- '- Derived from handoff `%s`\n' "$handoff_title"
printf -- '- Review and edit before creating the GitHub issue\n'
