---
name: arqix-plan-intake
description: Create or validate a mobile-first OpenClaw planning branch for arqix. Use when Codex needs deterministic branch naming, plan package creation under docs/plans/<slug>/, or a local draft planning commit before later delivery actions.
---

# Arqix Plan Intake

Create or validate a planning branch and draft planning package without moving into delivery actions.

## Use Only These Commands

Run these commands from the repository root:

- `./tools/openclaw/plan_intake_validate_env.sh`
- `./tools/openclaw/plan_branch_name.sh`
- `./tools/openclaw/plan_intake.sh`

## Follow This Workflow

1. Confirm the request is limited to intake validation, branch naming, or draft planning package creation.
2. Use the smallest command that answers the request.
3. Keep outputs deterministic and grounded in repository files.
4. Stop before Codex execution, PR creation, notifications, or other delivery actions.

## Apply These Safety Rules

- Do not run on `main`.
- Do not silently overwrite an existing planning package.
- Do not execute arbitrary shell commands.
- Stay inside the `arqix` repository root.

## Return This Kind of Summary

Prefer:
- readiness status, blockers, and warnings
- proposed branch name only when requested
- plan package path and created files
- whether a local draft commit was created
