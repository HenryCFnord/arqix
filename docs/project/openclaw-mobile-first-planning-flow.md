# OpenClaw mobile-first planning and delivery flow for arqix

## Status

Proposed replacement for the older handoff-first flow.

This document defines the new preferred OpenClaw workflow for `arqix`.

## Problem statement

The older flow assumed that a handoff document had to exist in the repository before OpenClaw or Codex could act on it.

That creates unnecessary friction for a mobile-first workflow:

- a draft idea must first be turned into a repository artifact,
- that artifact must be committed before a branch can be created from it,
- and only then can the actual preparation workflow begin.

For smartphone-driven work, this is too cumbersome.

## Design goal

Make the flow easy to start from a phone while keeping the repository as the source of truth as early as possible.

The new flow should allow:

1. a free-text idea to be sent to OpenClaw,
2. OpenClaw to classify the work and create a branch,
3. OpenClaw to generate draft planning artifacts on that branch,
4. the human to review and refine the plan on the phone,
5. OpenClaw to run Codex only after the reviewed plan has been pushed,
6. OpenClaw to notify the user and create a draft PR at the correct time.

## Core principles

- mobile-first intake
- repository artifacts created early
- clear human review checkpoints
- no direct implementation on `main`
- branch-first, plan-first, implementation-second
- OpenClaw orchestrates
- Codex implements
- GitHub remains the source of truth

## Canonical high-level flow

### Phase 1: idea intake

The user sends a free-text idea to OpenClaw through:
- Web UI
- CLI
- later optionally Signal

### Phase 2: classification

OpenClaw determines the branch category.

Preferred categories remain:

- `feat/`
- `fix/`
- `refactor/`
- `docs/`
- `blog/`
- `report/`
- `chore/`

If classification is unclear, OpenClaw should ask a short follow-up question rather than guessing silently.

### Phase 3: branch creation

OpenClaw creates a dedicated branch from the current default base branch.

The branch name should follow the existing branch naming rules and include a slug derived from the idea.

Examples:
- `chore/handoff-lifecycle-directories`
- `docs/openclaw-mobile-first-flow`
- `report/codex-openclaw-branch-planning`

OpenClaw must never do this work directly on `main`.

### Phase 4: draft planning artifacts

On the new branch, OpenClaw creates a draft planning package from the free-text idea.

Recommended location:

`docs/plans/<slug>/`

Recommended initial files:

- `IDEA.md`
- `PLANS.md`
- `STATUS.md`

### `IDEA.md`

Purpose:
- preserve the original idea
- record the intake source
- record timestamps
- preserve the original wording before refinement

Suggested content:
- title
- raw idea
- intake source
- created timestamp
- chosen branch
- category

### `PLANS.md`

Purpose:
- convert the idea into a structured execution plan
- act as the main planning artifact for later Codex work

Recommended sections:
- Summary
- Goal
- Out of scope
- Context
- Proposed approach
- Constraints
- Acceptance criteria
- Risks and unknowns
- Execution steps

### `STATUS.md`

Purpose:
- act as a lightweight operational ledger

Suggested content:
- current status
- branch name
- latest commit reference
- related files
- next review action
- later PR link
- later implementation summary

## Commit and push behavior

After creating the draft planning artifacts, OpenClaw should:

1. commit them on the branch
2. push the branch to the remote
3. return a concise summary

This enables the user to review and edit the plan from the smartphone using Git tools and repository-backed notes.

## Human review phase

The human then reviews and edits the draft plan on the branch.

Typical actions:
- refine `PLANS.md`
- clarify scope
- remove side quests
- strengthen acceptance criteria
- adjust category if needed

The user then commits and pushes the refined planning artifacts.

## Codex execution gate

Codex must not run on the initial free-text idea alone.

Codex may run only after:
- the planning artifacts exist on the branch,
- the plan has been reviewed or refined by the human,
- the branch has been pushed,
- and OpenClaw can pull the updated branch state.

At that point OpenClaw may:
- pull the branch on the Pi,
- invoke Codex in the repository,
- provide Codex with the relevant planning artifacts,
- and keep the implementation scoped to the reviewed plan.

## Delivery phase

After Codex completes a meaningful implementation step, OpenClaw should:

1. summarize the resulting changes,
2. optionally run targeted tests,
3. push resulting commits if appropriate,
4. notify the user,
5. create a draft PR through `gh`,
6. send the PR link back to the user.

A draft PR should be the default, not a ready-to-merge PR.

## Role separation

### OpenClaw

Responsible for:
- intake
- classification
- branch creation
- draft artifact generation
- orchestration
- notifications
- PR creation

### Codex

Responsible for:
- repository-local planning refinement when explicitly asked
- implementation on a prepared branch
- code and documentation changes
- scoped execution based on `PLANS.md` and repo rules

### Human

Responsible for:
- classification correction if needed
- plan review
- plan refinement
- final review before merge

### GitHub

Responsible for:
- version history
- branch storage
- PR workflow
- review surface
- CI status

## Repository structure proposal

Recommended planning structure:

```text
docs/
  plans/
    <slug>/
      IDEA.md
      PLANS.md
      STATUS.md
```

This structure replaces the assumption that a pre-existing handoff is required before branch creation.

Handoffs may still exist later if they prove useful, but they are no longer the mandatory intake mechanism.

## State model

Recommended lightweight state model for `STATUS.md`:

- `draft-created`
- `awaiting-human-review`
- `plan-refined`
- `ready-for-codex`
- `codex-running`
- `review-ready`
- `pr-created`
- `done`

## Skills impact

The older skill model should be updated.

### Old model

- readonly
- prepare from handoff
- execute from handoff

### New model

#### `arqix-repo-readonly`
Still valid.

#### `arqix-plan-intake`
New or renamed prepare-stage skill.

Used for:
- classify free-text idea
- create branch
- create `docs/plans/<slug>/`
- generate `IDEA.md`, `PLANS.md`, `STATUS.md`
- commit and push the draft plan

#### `arqix-delivery`
Updated delivery skill.

Used for:
- pull branch updates
- validate that the plan is ready
- run Codex against the reviewed plan
- summarize results
- create a draft PR
- notify the user

The old handoff-first prepare skill should be retired or converted.

## AGENTS.md impact

`AGENTS.md` should be updated to reflect the new canonical flow.

It should state that:

- the preferred intake path is now a branch-local planning package, not a pre-committed handoff
- implementation work should follow reviewed `PLANS.md` artifacts
- OpenClaw may create planning branches and draft planning artifacts before Codex starts
- non-trivial AI-assisted work still requires a branch and usually a PR
- `main` remains protected from direct non-trivial AI work

## CONTRIBUTING.md impact

`CONTRIBUTING.md` should also be aligned.

The branch prefixes remain unchanged, but the document should mention that:
- ideas may start as free-text intake
- OpenClaw can turn them into draft planning branches
- `docs/plans/<slug>/` is now the preferred planning artifact path
- handoffs are no longer the mandatory first step

## Migration note

The old handoff-first model is deprecated for the primary mobile workflow.

Existing handoff documents may remain in the repository, but new mobile-first work should default to:

free-text idea -> OpenClaw plan branch -> reviewed `PLANS.md` -> Codex -> draft PR

## Recommended first implementation slice

Implement the new flow in this order:

1. classify idea and create branch
2. create `docs/plans/<slug>/`
3. write `IDEA.md`, `PLANS.md`, and `STATUS.md`
4. commit and push the draft planning package
5. update skills to use this flow
6. update `AGENTS.md`
7. only then connect Codex execution to the reviewed plan

## Success criteria

The design is considered successfully implemented when:

- a free-text idea can be sent to OpenClaw
- OpenClaw can classify or ask for classification
- OpenClaw creates a correctly named branch
- OpenClaw creates and commits the planning package
- the user can review and edit the plan from the smartphone
- OpenClaw can later pull the refined branch
- OpenClaw can start Codex against the reviewed plan
- OpenClaw can send a review-ready notification and create a draft PR

## Final recommendation

Adopt this as the new preferred OpenClaw workflow for `arqix`.

Keep the existing branch naming scheme.

Retain the readonly capability.

Replace the old handoff-first prepare flow with a mobile-first planning-branch flow.
