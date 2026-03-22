# arqix repo readonly

This skill provides read-only repository inspection for arqix.

## Allowed actions

Use only these commands inside the repository root:

- `./tools/openclaw/repo_status.sh`
- `./tools/openclaw/list_handoffs.sh`
- `./tools/openclaw/last_commit.sh`

## Purpose

Use this skill when the user wants to:

- inspect the current repository status
- list approved handoff documents
- review the latest commit summary

## Safety rules

- Do not modify files.
- Do not create branches.
- Do not run package managers.
- Do not execute arbitrary shell commands.
- Stay inside the arqix repository.

## Output style

Return concise structured summaries.
Prefer:
- current branch
- changed files
- available handoffs
- last commit headline and changed files
