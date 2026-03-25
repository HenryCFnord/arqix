---
id: US-1001
kind: user_story
title: Doc-Package init
status: draft
tags:
- user-story
owner: hendrik
created: 2026-02-22
updated: 2026-02-22
priority: medium
related:
  requirements:
  - REQ-US-1001-01
  - REQ-US-1001-02
  - REQ-US-1001-03
  docs: []
  adrs: []
  personas:
  - PER-0001
lang: en
translation_of: US-1001
translation_status: draft
generated: false
source:
persona: PER-0001
old_id: US-0001
---
# Doc-Package init

## Story
As a maintainer, I want to initialize a new doc package with a standardized directory structure, so that I can build documentation consistently and reproducibly.

## Acceptance Criteria
- `darcy doc init <path>` creates `index.md`, `units/`, `pages/`, `artifacts/`, `logs/`, and `.darcy/`.
- `index.md` contains frontmatter with `id`, `kind=doc_index`, and `title`.
- `id`/`slug` are derived deterministically from `title` based on configurable slug rules.
- Existing files are not overwritten without explicit approval.

## Notes
TODO
