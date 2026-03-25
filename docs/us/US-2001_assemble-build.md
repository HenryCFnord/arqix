---
id: US-2001
kind: user_story
title: Assemble build
status: draft
tags:
- user-story
owner: hendrik
created: 2026-02-22
updated: 2026-02-22
priority: medium
related:
  requirements:
  - REQ-US-2001-01
  - REQ-US-2001-02
  - REQ-US-2001-03
  docs: []
  adrs: []
  personas:
  - PER-0002
lang: en
translation_of: US-2001
translation_status: draft
generated: false
source:
persona: PER-0002
old_id: US-0004
---
# Assemble build

## Story
As a maintainer, I want to assemble a doc package into pages, so that publishable chapters are produced from units.

## Acceptance Criteria
- `assemble build <doc-package>` generates `pages/*`.
- `strip_frontmatter_on_include` can be enabled via config.
- Include cycles are detected and fail with a clear error message.

## Notes
TODO
