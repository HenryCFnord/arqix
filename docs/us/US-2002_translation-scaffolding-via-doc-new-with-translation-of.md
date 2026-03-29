---
id: US-2002
kind: user_story
title: Translation scaffolding via doc new with --translation-of
status: draft
tags:
  - user_story
  - i18n
  - templates
owner: hcf
created: 2026-02-27
updated: 2026-02-27
priority: 
related:
  personas:
  - PER-0002
  workflows:
  - WF-0002
  - WF-0006
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

## Translation Scaffolding via Doc New with --translation-of

### Story

As a Developer, I want to create a translation document by referencing its source document ID using `doc new --lang <target> --translation-of <ID>`, so that translations are created with correct routing, metadata, and structure without manual setup.

### Acceptance Criteria

- The command creates the translation file at the correct location for the chosen i18n layout.
- The translation file contains correct metadata linking it to the source ID.
- `--dry-run` reports the planned target path and metadata without writing.
- The operation fails with a clear diagnostic if the source document cannot be found.
- The created translation preserves arqix markup directives and structural elements according to the scaffold strategy.

### Notes

The `arqix doc new <kind> --lang <target> --translation-of <ID>` workflow is intended to provide a deterministic and low-friction way to scaffold translation documents from an existing source note. It should resolve the source document using `source_lang`, derive the destination path from the configured i18n layout, and create the translation with the metadata needed to keep the source and translation linked over time. That metadata should include `lang`, `translation_of`, `source_updated`, and, where appropriate, optional fields such as `translated` and `translation_status` so that downstream linting and maintenance workflows can reason about freshness and state.

The command should also respect language-specific templates when available, while still allowing a fallback strategy so the translation scaffold can be created consistently even if a dedicated template is missing. A `--dry-run` mode is valuable here because it lets contributors and automation agents inspect the planned id, path, and metadata without mutating the vault, which makes the operation safer to use in CI and review-driven workflows.

In scope is the full scaffolding flow for translation creation, including source lookup, deterministic routing, metadata generation, and preservation of structural elements and arqix markup directives according to the scaffold strategy. Out of scope is any automatic translation of prose or rewriting of existing translations, since this command is meant to establish the correct document shape and bookkeeping rather than perform linguistic work. This remains a focused document-creation capability rather than a separate top-level i18n command.
