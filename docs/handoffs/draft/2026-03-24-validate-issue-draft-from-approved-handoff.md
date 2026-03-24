---
id: handoff-2026-03-24-validate-issue-draft-from-approved-handoff
project: arqix
title: Validate issue draft preparation from approved handoff
status: draft
type: chore
priority: medium
origin:
  - note:
  - issue:
  - pr:
branch: "chore/validate-issue-draft-from-handoff"
labels:
  - openclaw
  - handoff
  - issue-prep
owner: hcf
codex_mode: plan
tests_required: false
docs_required: true
review_required: true
---
# Validate issue draft preparation from approved handoff

## Summary

Validate that `tools/openclaw/prepare_issue_from_handoff.sh` produces a useful issue draft from a real approved handoff, and tighten the output only where needed so it aligns more clearly with the repository's issue template.

## Goal

Use the approved handoff at `docs/handoffs/approved/2026-03-22-initialize-handoff-lifecycle-directories.md` as real input for `prepare_issue_from_handoff.sh`, confirm the generated draft is structurally sound, and make the smallest useful refinements so the result is easier to review before any future `gh issue create` automation.

## Out of Scope

- Creating a GitHub issue
- Opening a pull request
- Codex execution from OpenClaw
- Broad handoff parser redesign
- Stage-3 delivery behavior
- Branch creation changes unless issue-draft work exposes a narrow blocking defect

## Context

- Relevant modules: `tools/openclaw/prepare_issue_from_handoff.sh`, `tools/openclaw/prepare_branch_name.sh`, `tools/openclaw/lib.sh`
- Relevant documents: `.github/ISSUE_TEMPLATE/task-from-handoff.md`, `docs/handoffs/approved/2026-03-22-initialize-handoff-lifecycle-directories.md`, `docs/project/openclaw-stage-2-prepare-plan.md`
- Related decisions: stage 2 should stay deterministic and local; approved handoffs are the execution-ready source of truth
- Related constraints: keep the change small, shell-based, and non-executing

## Proposed Change

Run the existing issue-draft preparation flow against the real approved handoff, compare the output with the GitHub issue template, and make only the narrow formatting or extraction fixes needed for a reliable draft artifact.

## Constraints

- Preserve existing public behavior unless explicitly stated otherwise.
- Prefer small, reviewable changes.
- Do not introduce new runtime dependencies without justification.
- Do not call `gh issue create`.
- Do not broaden scope into execution or delivery automation.

## Acceptance Criteria

- [ ] `tools/openclaw/prepare_issue_from_handoff.sh` works against `docs/handoffs/approved/2026-03-22-initialize-handoff-lifecycle-directories.md`.
- [ ] The generated draft remains concise and maps cleanly onto `.github/ISSUE_TEMPLATE/task-from-handoff.md`.
- [ ] Any changes stay limited to issue-draft preparation and closely related shared helpers.
- [ ] Documentation is updated only if a real invocation or caveat needs to be made explicit.

## Test Expectations

- Unit tests to add or update: none
- Manual checks: run `./tools/openclaw/prepare_issue_from_handoff.sh docs/handoffs/approved/2026-03-22-initialize-handoff-lifecycle-directories.md`
- Edge cases to consider: missing required handoff sections, blank branch metadata, and output shape drift from the issue template

## Files of Interest

- tools/openclaw/prepare_issue_from_handoff.sh
- .github/ISSUE_TEMPLATE/task-from-handoff.md
- docs/handoffs/approved/2026-03-22-initialize-handoff-lifecycle-directories.md

## Risks and Unknowns

- The current issue template may need normalization before later automation consumes it directly.
- Shell-based section extraction may become brittle if handoff structure drifts.
- The approved handoff may reveal formatting mismatches that should be fixed narrowly, not with a broader redesign.

## Execution Notes for Agent

- Start with a plan before editing code.
- Work on a dedicated branch.
- Keep commits focused.
- Do not create the GitHub issue; stop at the generated draft output.
