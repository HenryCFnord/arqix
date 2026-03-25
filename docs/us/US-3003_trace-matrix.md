---
id: US-3003
kind: user_story
title: Trace matrix
status: draft
tags:
- user-story
owner: hendrik
created: 2026-02-22
updated: 2026-02-22
priority: medium
related:
  requirements:
  - REQ-US-3003-01
  docs: []
  adrs: []
  personas:
  - PER-0003
lang: en
translation_of: US-3003
translation_status: draft
generated: false
source:
persona: PER-0003
old_id: US-0012
---
# Trace matrix

## Story
As a maintainer, I want to export a trace matrix, so that I can analyze relationships (REQ×Test, US×REQ) in tabular form.

## Acceptance Criteria
- `report trace-matrix` can export CSV.
- At least `REQ×Test` and `US×REQ` are supported.

## Notes
Acceptance should confirm that the exported CSV has stable headers and one row model per supported matrix type. Add tests for both `REQ×Test` and `US×REQ`, including empty-link cases that should still appear in a reviewer-friendly form. Keep the command explicit about which matrix is being generated so downstream analysis stays predictable.
