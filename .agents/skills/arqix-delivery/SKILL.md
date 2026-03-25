---
name: arqix-delivery
description: Validate that an arqix planning branch is ready for Codex and later delivery steps. Use when Codex needs to confirm that docs/plans/<slug>/ is complete and in a reviewed state before implementation work proceeds.
---

# Arqix Delivery

Validate the reviewed planning package before implementation or later OpenClaw delivery actions.

## Use Only These Commands

Run these commands from the repository root:

- `./tools/openclaw/delivery_validate_plan.sh`

## Follow This Workflow

1. Confirm the request is limited to plan readiness or delivery gating.
2. Validate the plan package that will guide implementation.
3. Summarize blockers clearly when the plan is still draft or awaiting human review.
4. Stop before PR creation, notifications, or other host-level delivery steps.

## Apply These Safety Rules

- Do not modify files.
- Do not execute arbitrary shell commands.
- Stay inside the `arqix` repository root.

## Return This Kind of Summary

Prefer:
- plan directory and branch
- current status value
- whether required planning files are present
- whether the plan is ready for Codex
