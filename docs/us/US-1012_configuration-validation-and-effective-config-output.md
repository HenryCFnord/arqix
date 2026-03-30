---
id: US-1012
kind: user_story
title: Configuration validation and effective config output
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

## Configuration Validation and effective Config Output

### Story

## Story
As a maintainer, I want to validate repository configuration and inspect the effective config, so documentation rules stay consistent and automation has a deterministic baseline.

### Acceptance Criteria

- `config validate` reports schema and contract violations deterministically.
- `config show` renders the effective config after defaults and overrides are applied.
- Diagnostics identify the failing key and source file when possible.

## Notes
Draft gap-fill for Mara Maintainer's open needs around diagnostics contracts and baseline configuration management.
