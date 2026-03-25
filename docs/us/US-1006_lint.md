---
id: US-1006
kind: user_story
title: Lint
status: draft
tags:
- user-story
owner: hendrik
created: 2026-02-22
updated: 2026-02-22
priority: medium
related:
  requirements:
  - REQ-US-1006-01
  - REQ-US-1006-02
  - REQ-US-1006-03
  docs: []
  adrs: []
  personas:
  - PER-0001
lang: en
translation_of: US-1006
translation_status: draft
generated: false
source:
persona: PER-0001
old_id: US-0007
---
# Lint

## Story
As a maintainer, I want to lint documents, so that errors (includes, metadata, IDs) are found early and deterministically.

## Acceptance Criteria
- `lint` checks include targets for existence.
- `lint` reports forbidden frontmatter keys in units (allowlist).
- `lint` reports duplicate IDs globally (units/REQ/US/ADR/glossary).

## Notes
TODO
