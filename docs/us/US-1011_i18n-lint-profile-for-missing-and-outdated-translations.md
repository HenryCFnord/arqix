---
id: US-1011
kind: user_story
title: i18n lint profile for missing and outdated translations
status: draft
tags:
  - user_story
  - i18n
  - lint
owner: hcf
created: 2026-02-27
updated: 2026-02-27
priority: 
related:
  personas:
    - PER-0001
  workflows:
    - WF-0001
    - WF-0003
    - WF-0004
    - WF-0008
  stories:
    - US-8201
    - US-8202
  requirements: []
  docs:
    - ADR-0012
lang: en
translation_of:
translation_status:
generated: false
source:
---

## i18n Lint Profile for Missing and Outdated Translations

### Story

As a maintainer, I want an i18n lint profile that detects missing translations, outdated translations (drift), and translation metadata mismatches, so bilingual documentation quality can be enforced in CI and used reliably by agents.

### Acceptance Criteria

- Running the i18n lint profile produces violations for:
  - an EN source doc without required DE translation
  - a DE translation doc whose `translation_of` cannot be resolved
  - a DE translation doc whose `source_updated` is older than source `updated`
- The set of required kinds/domains is configurable (policy in `arqix.toml`).
- Diagnostics include stable codes and source locations.
- i18n lint can be used as a CI gate (exit code 1 on violations).

### Notes


#### In Scope

- `arqix lint run --profile i18n` checks:
  - Missing translations for configured target languages and required kinds/domains
  - Outdated translation when `translation.source_updated != source.updated`
  - Mismatched or missing `translation_of`
  - Wrong `lang` fields
  - Optional: markup safety checks (arqix markers unchanged across languages)
- Diagnostics are machine-readable (`--format json`) and deterministic.

#### Out of Scope

- Enforcing linguistic quality of translations
- Cross-language semantic equivalence checks


This story depends on the i18n mapping (US-8201) and metadata contract (US-8202).
