---
name: arqix-repo-prepare
description: Prepare-task support for arqix. Use when Codex needs to validate repository readiness, derive a deterministic branch proposal from a handoff, create or switch to a safe local task branch, or render a GitHub issue draft without issue/PR creation or later delivery actions.
---

# Arqix Repo Prepare

Prepare task outputs and perform the first safe local branch setup step without executing delivery actions.

## Use Only These Commands

Run these commands from the repository root:

- `./tools/openclaw/prepare_validate_env.sh`
- `./tools/openclaw/prepare_branch_name.sh`
- `./tools/openclaw/prepare_issue_from_handoff.sh`
- `./tools/openclaw/create_branch_from_handoff.sh`

## Follow This Workflow

1. Confirm the request is limited to validation, draft preparation, or local branch setup.
2. Run only the command needed for the requested output.
3. Keep outputs deterministic and grounded in repository files.
4. Stop before any issue creation, PR creation, Codex execution, or other delivery step.

## Apply These Safety Rules

- Do not modify files.
- Do not call `gh issue create`.
- Do not execute arbitrary shell commands.
- Stay inside the `arqix` repository root.

## Return This Kind of Summary

Prefer:
- readiness status, blockers, and warnings
- proposed branch name only when requested
- branch creation or checkout result when requested
- issue draft body ready for review
