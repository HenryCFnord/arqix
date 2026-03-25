---
id: US-1004
kind: user_story
title: Assemble log
status: draft
tags:
  - user-story
owner: hcf
created: 2026-02-22
updated: 2026-02-22
priority: medium
related:
  requirements:
    - REQ-US-1004-01
  docs: []
  adrs: []
  personas:
    - PER-0001
lang: en
translation_of: US-1004
translation_status: draft
generated: false
source:
persona: PER-0001
old_id: US-0005
---

## Assemble Log

### Story

As a maintainer, I want a machine-readable log during assembly, so that I can trace include structure and outputs.

### Acceptance Criteria

- Assemble writes a JSONL log (path configurable).
- Log contains at least: `doc`, `chapter_id`, `out`, `include`, `sha256`, `bytes`, `at_line`.

### Notes

This is done when each assembly step emits one stable JSONL record that downstream tooling can parse without guessing field names. Add a test that checks the required keys and verifies the logged hash and byte count against a known include. If logging can be disabled or redirected, capture that behavior in CLI help and examples.
