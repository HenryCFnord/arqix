---
id: handoff-2026-03-22-initialize-handoff-lifecycle-directories
project: arqix
title: Initialize handoff lifecycle directories
status: approved
type: chore
priority: low
origin:
  - note: docs/project/openclaw-skill-linking.md
  - issue:
  - pr:
branch: "chore/handoff-lifecycle-directories"
labels:
  - workflow
  - handoff
owner: hcf
codex_mode: plan
tests_required: false
docs_required: true
review_required: true
---

## Initialize Handoff Lifecycle Directories

### Summary

Create the initial handoff lifecycle directory structure under `docs/handoffs/` and keep the directories tracked with `.gitkeep` files. This establishes stable repository paths for the staged OpenClaw workflow without introducing implementation or automation complexity.

### Goal

Create a minimal, explicit lifecycle structure for handoff documents. Ensure the repository contains tracked directories for the most important handoff states so that later automation can rely on stable paths. Keep the change intentionally small and operational.

### Out of Scope

- Do not implement OpenClaw automation changes.
- Do not add issue creation or PR creation logic.
- Do not implement handoff parsing or validation logic.
- Do not introduce additional lifecycle states unless clearly necessary.

### Context

Only the minimum context needed for correct execution:

- Relevant modules:
  - `docs/handoffs/`
  - `tools/openclaw/`
- Relevant documents:
  - `AGENTS.md`
  - `CONTRIBUTING.md`
  - `docs/project/openclaw-skill-linking.md`
- Related decisions:
  - OpenClaw capabilities are introduced in stages: `readonly`, `prepare`, `execute / delivery`.
  - The next safe step is to make the handoff workflow more concrete before adding more automation.
- Related constraints:
  - Keep the workflow lightweight and suitable for a solo project.
  - Prefer deterministic file-system level changes before adding orchestration logic.

### Proposed Change

Create the basic lifecycle directories for handoffs and add `.gitkeep` files so that empty directories are tracked in Git. If needed, add a brief documentation note that explains the intended purpose of the directories.

### Constraints

- Preserve existing public behavior unless explicitly stated otherwise.
- Prefer small, reviewable changes.
- Do not introduce new runtime dependencies without justification.

### Acceptance Criteria

- [ ] `docs/handoffs/draft/` exists.
- [ ] `docs/handoffs/approved/` exists.
- [ ] `docs/handoffs/done/` exists.
- [ ] Each lifecycle directory contains a `.gitkeep` file.
- [ ] The structure is documented briefly if needed.

### Test Expectations

- Unit tests to add or update:
  - None expected.
- Manual checks:
  - Verify the directories exist.
  - Verify `.gitkeep` files are tracked by Git.
  - Verify the branch name from this handoff is used or accepted correctly by the prepare-stage workflow.
- Edge cases to consider:
  - Directory already exists.
  - `.gitkeep` already exists.
  - Existing files inside the target directories must remain untouched.

### Files of Interest

- `docs/handoffs/`
- `docs/project/`
- `AGENTS.md`
- `CONTRIBUTING.md`

### Risks and Unknowns

- Future lifecycle states may be introduced later and could slightly change the structure.
- The final handoff movement workflow has not been fully defined yet.

### Execution Notes for Agent

- Start with a plan before editing code.
- Work on a dedicated branch.
- Keep commits focused.
- Update docs if behavior or interfaces change.
