---
id: US-5004
kind: user_story
title: Read structured document sections with stable selectors
status: draft
persona: PER-0005
tags:
- user-story
- supplemental-draft
owner: codex
created: '2026-03-25'
updated: '2026-03-25'
priority: medium
related:
  personas:
  - PER-0005
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
# Read structured document sections with stable selectors

## Story
As an automation engineer, I want to read specific sections of a document using stable selectors, so that agents can cite the right context without reparsing whole files.

## Acceptance Criteria
- `doc read` supports section-level retrieval by heading slug or explicit anchor.
- Structured output includes resolved document metadata and selector details.
- Failures identify whether the document or selector was not found.

## Notes
Draft gap-fill for precise downstream retrieval workflows.
