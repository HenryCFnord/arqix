---
id: US-3001
kind: user_story
title: Trace scan
status: draft
tags:
- user-story
owner: hendrik
created: 2026-02-22
updated: 2026-02-22
priority: medium
related:
  requirements:
  - REQ-US-3001-01
  - REQ-US-3001-02
  - REQ-US-3001-03
  - REQ-US-3001-04
  docs: []
  adrs: []
  personas:
  - PER-0003
lang: en
translation_of: US-3001
translation_status: draft
generated: false
source:
persona: PER-0003
old_id: US-0010
---
# Trace scan

## Story
As a maintainer, I want to scan traceability information, so that a graph of documentation, code, and test references can be built.

## Acceptance Criteria
- `trace scan` detects markers in Rust comments (markers configurable).
- `trace scan` detects markers in Markdown HTML comments.
- `trace scan` reads unit frontmatter links (`requirements/stories/adrs/refs`).
- Trace outputs a graph (nodes/edges) as JSON.

## Notes
TODO
