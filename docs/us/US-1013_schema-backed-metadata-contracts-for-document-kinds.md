---
id: US-1013
kind: user_story
title: Schema-backed metadata contracts for document kinds
status: draft
tags:
  - user-story
  - supplemental-draft
owner: codex
created: '2026-03-25'
updated: '2026-03-25'
priority: medium
related:
  personas:
  - PER-0001
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

## Schema-backed Metadata Contracts for Document Kinds

### Story

## Story
As a maintainer, I want schema-backed metadata contracts for each document kind, so frontmatter drift is caught early and templates remain enforceable over time.

### Acceptance Criteria

- Document kinds can declare required and optional metadata fields in a schema contract.
- Lint surfaces missing, extra, and type-invalid fields deterministically.
- Templates and validation use the same contract source.

## Notes
Draft gap fill for long-term maintainability and scalable documentation governance.
