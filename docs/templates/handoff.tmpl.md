---
id: handoff-YYYY-MM-DD-short-slug
project: arqix
title: Short imperative title
status: draft
type: feature
priority: medium
origin:
  - note:
  - issue:
  - pr:
branch: ""
labels:
  - needs-triage
owner: hcf
codex_mode: plan
tests_required: true
docs_required: true
review_required: true
---
# {title}

## Summary

One paragraph that explains the task, why it matters, and what should change.

## Goal

Describe the desired outcome in 1 to 3 concrete sentences.

## Out of Scope

- What should explicitly not be changed?
- What tempting side quest should be ignored?

## Context

List only the context needed to act correctly.

- Relevant modules:
- Relevant documents:
- Related decisions:
- Related constraints:

## Proposed Change

Describe the intended direction at a high level.

## Constraints

- Preserve existing public behavior unless explicitly stated otherwise.
- Prefer small, reviewable changes.
- Do not introduce new runtime dependencies without justification.

## Acceptance Criteria

- [ ] Criterion 1
- [ ] Criterion 2
- [ ] Criterion 3

## Test Expectations

- Unit tests to add or update:
- Manual checks:
- Edge cases to consider:

## Files of Interest

- path/to/file
- path/to/file

## Risks and Unknowns

- Risk 1
- Unknown 1

## Execution Notes for Agent

- Start with a plan before editing code.
- Work on a dedicated branch.
- Keep commits focused.
- Update docs if behavior or interfaces change.
