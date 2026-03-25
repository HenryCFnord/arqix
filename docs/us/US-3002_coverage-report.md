---
id: US-3002
kind: user_story
title: Coverage report
status: draft
tags:
- user-story
owner: hendrik
created: 2026-02-22
updated: 2026-02-22
priority: medium
related:
  requirements:
  - REQ-US-3002-01
  - REQ-US-3002-02
  - REQ-US-3002-03
  docs: []
  adrs: []
  personas:
  - PER-0003
lang: en
translation_of: US-3002
translation_status: draft
generated: false
source:
---

# Coverage report

## Story
As a maintainer, I want to generate coverage reports, so that I can detect gaps between requirements, code, and tests.

## Acceptance Criteria
- `report coverage` identifies REQs without `verifies` tests.
- `report coverage` identifies REQs without `implements` code.
- Output supports at least Markdown and JSON.

## Notes
This report is useful only if missing links are easy to spot and identical inputs produce identical output ordering. Add tests that exercise uncovered requirements for both code and tests, plus format checks for Markdown and JSON rendering. A good next step is to define whether partially covered requirements should be flagged separately from fully uncovered ones.