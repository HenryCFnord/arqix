---
name: arqix-repo-prepare
description: Non-executing task preparation for arqix. Use when Codex needs to validate repository readiness for scoped work, derive a deterministic branch proposal from task metadata or a handoff, or render a GitHub issue draft from an approved handoff without creating branches, issues, or other delivery actions.
---

# Arqix Repo Prepare

Prepare task outputs without executing delivery steps.

## Use Only These Commands

Run these commands from the repository root:

- `./tools/openclaw/prepare_validate_env.sh`
- `./tools/openclaw/prepare_branch_name.sh`
- `./tools/openclaw/prepare_issue_from_handoff.sh`

## Follow This Workflow

1. Confirm the request is limited to validation or draft preparation.
2. Run only the command needed for the requested output.
3. Keep outputs deterministic and grounded in repository files.
4. Stop before any branch creation, issue creation, or other execution step.

## Apply These Safety Rules

- Do not modify files.
- Do not create branches.
- Do not call `gh issue create`.
- Do not execute arbitrary shell commands.
- Stay inside the `arqix` repository root.

## Return This Kind of Summary

Prefer:
- readiness status, blockers, and warnings
- proposed branch name only when requested
- issue draft body ready for review
