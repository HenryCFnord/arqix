---
id: handoff-2026-03-22-initialize-handoff-lifecycle-directories
project: arqix
title: Initialize handoff lifecycle directories
status: approved
type: chore
priority: medium
origin:
  - note:
  - issue:
  - pr:
branch: "chore/handoff-lifecycle-directories"
labels:
  - openclaw
  - handoff
owner: hcf
codex_mode: default
tests_required: false
docs_required: true
review_required: true
---
# Initialize handoff lifecycle directories

## Summary

Create the minimal tracked handoff lifecycle structure in the repository and validate that OpenClaw's branch-creation workflow works against a real approved handoff.

## Goal

Add explicit `draft`, `approved`, and `archived` handoff directories under `docs/handoffs/`, keep empty lifecycle directories tracked with `.gitkeep`, and confirm that `tools/openclaw/create_branch_from_handoff.sh` can create or switch to `chore/handoff-lifecycle-directories` from this approved handoff.

## Out of Scope

- GitHub issue creation
- Pull request creation
- Codex execution from OpenClaw
- Generalized handoff parser work beyond current shell-based parsing

## Context

- Relevant modules: `tools/openclaw/create_branch_from_handoff.sh`, `tools/openclaw/lib.sh`
- Relevant documents: `docs/project/openclaw-stage-2-prepare-plan.md`, `docs/templates/handoff.tmpl.md`
- Related decisions: stage 2 should stay operational but conservative
- Related constraints: keep the change small, local, and reviewable

## Proposed Change

Add the minimal lifecycle directories, commit this approved handoff as a real stage-2 example, and document a real invocation of the branch-creation script against it.

## Constraints

- Prefer simple shell over unnecessary abstraction.
- Do not introduce issue, PR, or execution automation.
- Fail clearly if the handoff path or branch metadata is invalid.

## Acceptance Criteria

- [ ] `docs/handoffs/draft/`, `docs/handoffs/approved/`, and `docs/handoffs/archived/` exist in git.
- [ ] Empty lifecycle directories are tracked with `.gitkeep` where needed.
- [ ] `tools/openclaw/create_branch_from_handoff.sh` works with this handoff and targets `chore/handoff-lifecycle-directories`.
- [ ] The stage-2 prepare documentation includes one real invocation example for this handoff.

## Test Expectations

- Unit tests to add or update: none
- Manual checks: run `./tools/openclaw/create_branch_from_handoff.sh docs/handoffs/approved/2026-03-22-initialize-handoff-lifecycle-directories.md`
- Edge cases to consider: dirty worktree should still block local branch switching

## Files of Interest

- docs/handoffs/approved/2026-03-22-initialize-handoff-lifecycle-directories.md
- docs/project/openclaw-stage-2-prepare-plan.md
- tools/openclaw/create_branch_from_handoff.sh

## Risks and Unknowns

- Shell-based frontmatter parsing is still sensitive to handoff template drift.
- Real success-path validation requires a clean worktree.

## Execution Notes for Agent

- Start with a plan before editing code.
- Work on a dedicated branch.
- Keep commits focused.
- Update docs only where this workflow becomes easier to use.
