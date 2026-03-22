# arqix repo prepare

This skill provides non-executing preparation actions for arqix.

## Allowed actions

Use only these commands inside the repository root:

- `./tools/openclaw/prepare_validate_env.sh`
- `./tools/openclaw/prepare_branch_name.sh`
- `./tools/openclaw/prepare_issue_from_handoff.sh`

## Purpose

Use this skill when the user wants to:

- validate whether the repository is ready for the next scoped task
- derive a safe branch proposal for a handoff-driven task
- render a GitHub issue draft from a repository handoff

## Safety rules

- Do not modify files.
- Do not create branches.
- Do not call `gh issue create`.
- Do not execute arbitrary shell commands.
- Stay inside the arqix repository.

## Output style

Return concise structured summaries.
Prefer:
- readiness status and blockers
- proposed branch name
- issue draft body ready for review
