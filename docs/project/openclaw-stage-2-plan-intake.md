# OpenClaw Stage 2 Plan Intake

## Summary

Stage 2 is now a mobile-first plan intake step rather than a handoff-first prepare step.

It should:

- validate that the local repository is ready for planning intake
- accept a chosen branch category and slug
- create or switch to the planning branch
- create `docs/plans/<slug>/`
- generate `IDEA.md`, `PLANS.md`, and `STATUS.md`
- optionally create a local draft commit
- stop before Codex execution, PR creation, or notifications

This keeps OpenClaw aligned with the preferred workflow in `AGENTS.md`, `CONTRIBUTING.md`, and `docs/project/openclaw-mobile-first-planning-flow.md`.

## Stage split

The repository-managed OpenClaw split is now:

1. `arqix-repo-readonly`
2. `arqix-plan-intake`
3. `arqix-delivery`

Readonly inspects the repository, plan intake creates the planning branch artifacts, and delivery validates that a reviewed plan is ready for Codex and later draft PR steps.

## Repository support

The first repository-side plan intake slice uses these scripts:

- `tools/openclaw/plan_from_idea.sh`
- `tools/openclaw/plan_intake_validate_env.sh`
- `tools/openclaw/plan_branch_name.sh`
- `tools/openclaw/plan_intake.sh`
- `tools/openclaw/delivery_validate_plan.sh`

The wrapper contract lives here:

- `docs/project/openclaw-plan-intake-wrapper.md`

The plan package lives here:

- `docs/plans/<slug>/IDEA.md`
- `docs/plans/<slug>/PLANS.md`
- `docs/plans/<slug>/STATUS.md`

## Operational defaults

- branch prefixes remain `feat/`, `fix/`, `refactor/`, `docs/`, `blog/`, `report/`, and `chore/`
- `plan_intake.sh` fails on a dirty worktree
- `plan_intake.sh` refuses to overwrite an existing planning package
- `delivery_validate_plan.sh` treats `plan-refined`, `ready-for-codex`, `codex-running`, `review-ready`, `pr-created`, and `done` as delivery-ready states
- human review is still required before Codex starts implementation

## Deprecated flow

The old handoff-first prepare flow is no longer the preferred path for mobile-first work.

Existing handoffs may remain in the repository, but new OpenClaw intake should default to:

free-text idea -> plan branch -> reviewed `PLANS.md` -> Codex -> draft PR
