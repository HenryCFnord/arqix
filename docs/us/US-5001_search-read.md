---
id: US-5001
kind: user_story
title: Search/read
status: draft
tags:
- user-story
owner: hcf
created: 2026-02-22
updated: 2026-02-22
priority: medium
related:
  requirements:
  - REQ-US-5001-01
  - REQ-US-5001-02
  docs: []
  adrs: []
  personas:
  - PER-0005
lang: en
translation_of: US-5001
translation_status: draft
generated: false
source:
---

# Search/read

## Story
As a maintainer, I want to search and read documentation, so that I can quickly retrieve content via CLI (and later via MCP).

## Acceptance Criteria
- Search is available (v0: full-text is sufficient).
- `read` supports `doc-id` and optionally `section/anchor`.

## Notes
The first version is sufficient if maintainers can reliably find a document by text query and read a full document or anchored section without extra tooling. Add tests for exact and partial matches, ambiguous search results, and missing anchors. Keep CLI output concise because the same behavior will likely be reused by MCP later.