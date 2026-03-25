---
id: US-1008
kind: user_story
title: Finalize
status: draft
tags:
  - user-story
owner: hcf
created: 2026-02-22
updated: 2026-02-22
priority: medium
related:
  requirements:
    - REQ-US-1008-01
    - REQ-US-1008-02
  docs: []
  adrs: []
  personas:
    - PER-0001
lang: en
translation_of: US-1008
translation_status: draft
generated: false
source:
---

## Finalize

### Story

As a maintainer, I want to mechanically finalize metadata, so that `updated` is set consistently without rewriting content.

### Acceptance Criteria

- `finalize` sets `updated` as an ISO-8601 date (`YYYY-MM-DD`).
- `finalize` performs only mechanical changes (no rewriting of body text).

### Notes

This should be treated as a narrow metadata operation, not a content rewrite step. Add tests showing that `updated` is written in `YYYY-MM-DD` format and that repeated runs only touch metadata when the value changes. If files without frontmatter are unsupported, fail clearly and document that boundary.
