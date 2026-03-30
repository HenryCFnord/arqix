---
id: US-1001
kind: user_story
title: Doc-Package init
status: draft
tags:
  - user-story
owner: hcf
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
translation_of:
translation_status: draft
generated: false
source:
---

## Doc-Package Init

### Story

As a maintainer, I want to initialize a new doc package with a standardized directory structure, so I can build documentation consistently and reproducibly.

### Acceptance Criteria

- `arqix doc init <path>` creates `index.md`, `units/`, `pages/`, `artifacts/`, `logs/`, and `.arqix/`.
- `index.md` contains frontmatter with `id`, `kind=doc_index`, and `title`.
- `id`/`slug` are derived deterministically from `title` based on configurable slug rules.
- Existing files are not overwritten without explicit approval.

### Notes

Acceptance is met when initialization works in an empty target path and produces the expected scaffold without manual cleanup. Add tests for deterministic `id` and `slug` generation from the same title input and for the refusal path when files already exist. Document any prompt or force behavior explicitly if overwrite protection is configurable.
