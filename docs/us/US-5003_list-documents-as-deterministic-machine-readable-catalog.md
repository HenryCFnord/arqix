---
id: US-5003
kind: user_story
title: List documents as deterministic machine-readable catalog
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
# List documents as deterministic machine-readable catalog

## Story
As an automation engineer, I want a deterministic document catalog export, so that downstream indexing and retrieval systems can consume arqix content without scraping Markdown.

## Acceptance Criteria
- `doc list` can emit JSON with stable ordering and core metadata for each document.
- Catalog entries include at minimum `id`, `kind`, `title`, `lang`, and source path.
- Filtering by kind and language is supported.

## Notes
Draft gap-fill for Alex AIOps workflows that treat documentation as a dataset.
