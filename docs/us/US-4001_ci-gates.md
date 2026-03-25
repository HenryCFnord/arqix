---
id: US-4001
kind: user_story
title: CI gates
status: draft
tags:
- user-story
owner: hcf
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
---

# CI gates

## Story
As a maintainer, I want consistent exit codes and CI support, so automation can react to arqix reliably.

## Acceptance Criteria
- Exit codes are consistent: `0` ok, `1` lint fail, `2` usage.
- Optionally, a GitHub Actions template exists for typical gates.

## Notes
This story is done when CI can distinguish usage errors from quality gate failures solely through exit status and stable stderr messaging. Add tests that exercise each documented exit code and verify that lint failures do not collapse into generic command errors. If a GitHub Actions template is shipped, keep it minimal and aligned with the supported commands only.
