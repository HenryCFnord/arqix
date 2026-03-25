---
id: US-1003
kind: user_story
title: Units erstellen
status: draft
tags:
  - user-story
owner: hcf
created: 2026-02-22
updated: 2026-02-22
priority: medium
related:
  requirements:
    - REQ-US-1003-01
    - REQ-US-1003-02
  docs: []
  adrs: []
  personas:
    - PER-0001
lang: en
translation_of: US-1003
translation_status: draft
generated: false
source:
---

## Units Erstellen

### Story

As a maintainer, I want to create units quickly, so I can maintain documentation modularly in small, consistent building blocks.

### Acceptance Criteria

- `unit new` creates a unit file (frontmatter optional, configurable).
- Units can carry a global `id` (frontmatter or directive) that is linted for uniqueness.

### Notes

Acceptance should cover both the default unit creation path and the configured variant without frontmatter. Add tests for unique ID validation across multiple units and for the generated file shape when optional metadata is disabled. The command help should make clear where units are created and how IDs are supplied.
