# Planning Packages

OpenClaw planning packages live under:

`docs/plans/<slug>/`

Each package is branch-local and is intended to be easy to review from a phone before Codex starts implementation work.

The standard package contains:

- `IDEA.md` for the original free-text intake
- `PLANS.md` for the reviewed execution plan
- `STATUS.md` for the lightweight operational state

The preferred mobile-first flow is:

1. free-text idea goes to OpenClaw
2. OpenClaw classifies the work and creates a branch
3. OpenClaw creates the draft planning package
4. the human reviews and edits the plan on that branch
5. Codex implements only after the reviewed plan is ready

Handoffs remain supported where useful, but they are no longer the default intake artifact for mobile-first work.
