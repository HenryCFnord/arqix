---
id: US-2001
kind: user_story
title: Assemble build
status: draft
tags:
- user-story
owner: hcf
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
---

# Assemble build

## Story
As a maintainer, I want to assemble a doc package into pages, so publishable chapters are produced from units.

## Acceptance Criteria
- `assemble build <doc-package>` generates `pages/*`.
- `strip_frontmatter_on_include` can be enabled via config.
- Include cycles are detected and fail with a clear error message.

## Notes
The build flow is complete when a doc package with nested includes produces stable page outputs and cycles fail fast with a readable path trace. Add tests for frontmatter stripping on included content and for deterministic output ordering across repeated runs. The first implementation should optimize for clear diagnostics over aggressive assembly features.
