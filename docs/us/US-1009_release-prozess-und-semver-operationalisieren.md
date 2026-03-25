---
id: US-1009
kind: user_story
title: Release-Prozess und SemVer operationalisieren
status: draft
tags:
  - user-story
owner: hcf
created: 2026-02-22
updated: 2026-02-22
priority: medium
related:
  requirements: []
  docs: []
  adrs: []
  personas:
    - PER-0001
lang: en
translation_of: US-1009
translation_status: draft
generated: false
source:
---

## Release-Prozess Und SemVer Operationalisieren

### Story

As a maintainer, I want a documented, Codex-friendly release process using SemVer, so releases can be prepared traceably and executed safely.

### Acceptance Criteria

- `CHANGELOG.md` and `RELEASING.md` exist and are consistent.
- SemVer rules for product version and separate `config_version`/`schema_version` are documented.
- Codex is limited to release preparation only (no tagging/publishing without approval).
- Breaking changes require migration notes and changelog entries.

### Notes

Acceptance should include a dry-run style checklist that a reviewer can follow from changelog preparation through release approval without hidden steps. Add documentation examples for patch, minor, and major releases, including when `config_version` or `schema_version` must change independently. Keep the process explicit about the final manual approval points for tagging and publishing.
