---
id: US-4002
kind: user_story
title: Publish
status: draft
tags:
- user-story
owner: hcf
created: 2026-02-22
updated: 2026-02-22
priority: medium
related:
  requirements:
  - REQ-US-4002-01
  - REQ-US-4002-02
  - REQ-US-4002-03
  - REQ-US-4002-04
  - REQ-US-4002-05
  - REQ-US-4002-06
  - REQ-US-4002-07
  - REQ-US-4002-08
  docs: []
  adrs: []
  personas:
  - PER-0004
lang: en
translation_of: US-4002
translation_status: draft
generated: false
source:
---

# Publish

## Story
As a maintainer, I want to generate publishing outputs, so that documentation can be published as PDF and/or a website.

## Acceptance Criteria
- Assembled pages are artifact-ready; optionally `site build` is orchestrated.
- `render pdf` runs Pandoc on assembled pages or selected Markdown files.
- Pandoc `--defaults` is supported; optionally `--template eisvogel`.
- Artifacts are stored according to `artifacts.mode`; Pandoc errors are forwarded cleanly.
- Per-doc-package render config/overrides are supported.
- GitHub Pages deployment is supported (including optional `.nojekyll`).

## Notes
Acceptance should cover both successful artifact generation and clean failure forwarding when Pandoc or site generation returns an error. Add integration-style tests for defaults handling, per-package overrides, and artifact placement under the configured mode. Defer deployment automation details unless they are needed to prove the documented GitHub Pages path end to end.