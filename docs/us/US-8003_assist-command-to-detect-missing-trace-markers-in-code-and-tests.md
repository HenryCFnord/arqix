---
id: US-8003
kind: user_story
title: Assist command to detect missing trace markers in code and tests
status: draft
tags: []
owner: 
created: 
updated: 
priority: 
related:
  requirements: []
  personas:
  - PER-0008
lang: 
translation_of: 
translation_status: 
generated: 
source: 
---

# US-8003 — Assist command to detect missing trace markers in code and tests

As an Automation Agent, I want arqix to detect missing `implements` and `verifies` markers for a given requirement across code and tests, so I can add only the missing annotations and avoid unnecessary edits.

## Acceptance Criteria

- A command exists, e.g. `arqix trace check --req REQ-xxxx`, that reports:
  - whether any `implements` markers exist
  - whether any `verifies` markers exist
  - locations of existing markers (path + line)
- Output can be emitted as JSON for agent consumption.

## Notes

In scope:
- A command (or subcommand) that reports missing marker coverage for a given REQ ID
- Output includes file/line suggestions when markers exist, and missing marker lists when they do not

Out of scope:
- Automatic insertion of markers into files
- Language-specific parsing beyond marker detection
