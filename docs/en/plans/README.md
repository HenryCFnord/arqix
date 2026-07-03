# Planning Packages

Branch-local planning packages live under:

`docs/en/plans/<slug>/`

Each package is intended to be easy to review before implementation work starts.

The standard package contains:

- `IDEA.md` for the original free-text intake
- `PLANS.md` for the reviewed execution plan
- `STATUS.md` for the lightweight operational state

The preferred flow is:

1. a free-text idea is captured as a draft planning package on a dedicated branch
2. the human reviews and edits the plan on that branch
3. a coding agent implements only after the reviewed plan is ready

Handoffs remain supported where useful, but they are not the default intake artefact.
