---
id: US-5002
kind: user_story
title: MCP stdio
status: draft
tags:
- user-story
owner: hendrik
created: 2026-02-22
updated: 2026-02-22
priority: medium
related:
  requirements:
  - REQ-US-5002-01
  - REQ-US-5002-02
  docs: []
  adrs: []
  personas:
  - PER-0005
lang: en
translation_of: US-5002
translation_status: draft
generated: false
source:
persona: PER-0005
old_id: US-0014
---
# MCP stdio

## Story
As a maintainer, I want to expose darcy via MCP over stdio, so that agents can use standardized tools to access documentation.

## Acceptance Criteria
- `mcp serve` supports stdio transport.
- MCP provides at least the tools `search`, `read`, `list`.

## Notes
TODO
