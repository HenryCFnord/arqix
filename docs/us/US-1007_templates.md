---
id: US-1007
kind: user_story
title: Templates
status: draft
tags:
  - user-story
owner: hcf
created: 2026-02-22
updated: 2026-02-22
priority: medium
related:
  requirements:
    - REQ-US-1007-01
    - REQ-US-1007-02
    - REQ-US-1007-03
  docs: []
  adrs: []
  personas:
    - PER-0001
lang: en
translation_of: US-1007
translation_status: draft
generated: false
source:
---

## Templates

### Story

As a maintainer, I want to create documents from templates, so new artifacts (glossary/REQ/US/ADR) are compliant from the start.

### Acceptance Criteria

- `doc new <kind>` creates files from templates; supported `kind` values come exclusively from config.
- Aliases `glossary new`, `req new`, `us new`, `adr new` are available (or documented via `doc new`).
- Templates support placeholders `{title}`, `{slug}`, `{id}`.

### Notes

Acceptance should verify that every configured template kind renders the expected file skeleton with placeholders resolved consistently. Add tests for supported aliases, unknown kinds, and deterministic `{slug}` and `{id}` substitution from the same title. The next step is to keep template configuration as the single source of truth so command behavior and docs stay aligned.
