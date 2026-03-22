---
name: arqix-repo-readonly
description: Read-only repository inspection for arqix. Use when Codex needs to inspect the current branch and worktree, list approved handoffs, or summarize the latest commit without modifying files, creating branches, or running arbitrary shell commands.
---

# Arqix Repo Readonly

Inspect the repository without changing it.

## Use Only These Commands

Run these commands from the repository root:

- `./tools/openclaw/repo_status.sh`
- `./tools/openclaw/list_handoffs.sh`
- `./tools/openclaw/last_commit.sh`

## Follow This Workflow

1. Confirm the request is inspection-only.
2. Use the smallest allowed command that answers the question.
3. Summarize the result concisely instead of pasting raw command output.
4. Call out blockers or missing repository artifacts explicitly.

## Apply These Safety Rules

- Do not modify files.
- Do not create branches.
- Do not run package managers.
- Do not execute arbitrary shell commands.
- Stay inside the `arqix` repository root.

## Return This Kind of Summary

Prefer:
- current branch and worktree state
- approved handoff availability
- latest commit headline and changed files
