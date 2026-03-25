---
id: US-6002
kind: user_story
title: Glossary term scaffolding with cross-linkable IDs
status: draft
persona: PER-0006
tags:
- user-story
- supplemental-draft
owner: codex
created: '2026-03-25'
updated: '2026-03-25'
priority: medium
related:
  personas:
  - PER-0006
  workflows: []
  stories: []
  requirements: []
  docs: []
  adrs: []
lang: en
translation_of: ''
translation_status: ''
generated: false
source: ''
---
# Glossary term scaffolding with cross-linkable IDs

## Story
As an architect, I want to create glossary terms from a template with stable IDs, so that architecture vocabulary stays consistent across ADRs, handbooks, and requirements.

## Acceptance Criteria
- `doc new glossary` creates a glossary term with required metadata and deterministic routing.
- Glossary terms can be referenced by stable ID from ADRs and other docs.
- Lint detects duplicate or malformed glossary IDs.

## Notes
Draft gap-fill for Aria Architect terminology governance.
