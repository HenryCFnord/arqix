---
id: US-1005
kind: user_story
title: Format
status: draft
tags:
- user-story
owner: hendrik
created: 2026-02-22
updated: 2026-02-22
priority: medium
related:
  requirements:
  - REQ-US-1005-01
  - REQ-US-1005-02
  docs: []
  adrs: []
  personas:
  - PER-0001
lang: en
translation_of: US-1005
translation_status: draft
generated: false
source:
persona: PER-0001
old_id: US-0006
---
# Format

## Story
As a maintainer, I want to format documents canonically, so that diffs stay small and conventions are applied automatically.

## Acceptance Criteria
- `fmt` sorts frontmatter keys by configurable `key_order`.
- `fmt` normalizes directives (attribute order, whitespace) without semantic changes.

## Notes
Acceptance should prove that formatting is idempotent and does not change document meaning beyond canonical ordering and whitespace normalization. Add snapshot-style tests for frontmatter key ordering and directive normalization on realistic inputs. Any fields or directives intentionally left untouched should be called out explicitly to avoid accidental scope growth.
