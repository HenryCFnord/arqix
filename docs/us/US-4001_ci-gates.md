---
id: US-4001
kind: user_story
title: CI gates
status: draft
tags:
- user-story
owner: hendrik
created: 2026-02-22
updated: 2026-02-22
priority: medium
related:
  requirements:
  - REQ-US-4001-01
  - REQ-US-4001-02
  docs: []
  adrs: []
  personas:
  - PER-0004
lang: en
translation_of: US-4001
translation_status: draft
generated: false
source:
persona: PER-0004
old_id: US-0015
---
# CI gates

## Story
As a maintainer, I want consistent exit codes and CI support, so that automation can react to arqix reliably.

## Acceptance Criteria
- Exit codes are consistent: `0` ok, `1` lint fail, `2` usage.
- Optionally, a GitHub Actions template exists for typical gates.

## Notes
TODO
