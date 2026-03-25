---
id: US-4004
kind: user_story
title: Language-aware site publishing (Zensical-first)
status: draft
tags:
- user_story
- i18n
- publish
owner: Hendrik
created: 2026-02-27
updated: 2026-02-27
related:
  personas:
  - PER-0004
  workflows:
  - WF-0004
  - WF-0001
  - WF-0008
  stories:
  - US-8201
  requirements: []
  docs:
  - ADR-0012
lang: en
translation_of:
translation_status:
generated: false
source:
persona: PER-0004
old_id: US-8205
---
# Language-aware site publishing (Zensical-first)

## Story

As a DevOps Engineer, I want to publish documentation sites per language using arqix, with Zensical as the first supported site toolchain, so that bilingual documentation can be built and deployed deterministically in CI.

## Scope

### In scope
- `arqix publish site --lang <lang>` selects the correct language root based on i18n configuration
- Zensical is invoked as the first site builder integration
- Outputs are written to deterministic artifact locations (e.g. `doc/artifacts/site/<lang>/...`)
- Machine-readable diagnostics for failures (`--format json`)

### Out of scope
- HTML generation via Pandoc
- Additional site builders beyond Zensical

## Acceptance Criteria
- `arqix publish site --lang en` builds a site from the EN root and writes outputs to the EN artifact target.
- `arqix publish site --lang de` builds a site from the DE root and writes outputs to the DE artifact target.
- The resolved roots come from `arqix.toml` i18n configuration and are visible in effective config.
- If Zensical fails, arqix returns exit code 2 and diagnostics that identify the failing tool invocation context.

## Notes
This story validates that the chosen i18n layout is practical for CI and automation.
