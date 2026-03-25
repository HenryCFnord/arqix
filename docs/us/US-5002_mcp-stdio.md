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
---

# MCP stdio

## Story
As a maintainer, I want to expose arqix via MCP over stdio, so that agents can use standardized tools to access documentation.

## Acceptance Criteria
- `mcp serve` supports stdio transport.
- MCP provides at least the tools `search`, `read`, `list`.

## Notes
Acceptance should prove that an MCP client can start the server over stdio, discover the declared tools, and execute `search`, `read`, and `list` successfully. Add an integration test or protocol fixture that validates request and response structure rather than only unit-testing handlers. Keep transport concerns separate from tool logic so the CLI and MCP layers can evolve independently.