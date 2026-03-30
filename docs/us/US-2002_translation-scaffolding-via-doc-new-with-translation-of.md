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

# Translation scaffolding via doc new with --translation-of

## Story

As a Developer, I want to create a translation document by referencing its source document ID using `doc new --lang <target> --translation-of <ID>`, so translations are created with correct routing, metadata, and structure without manual setup.

## Acceptance Criteria
- The command creates the translation file at the correct location for the chosen i18n layout.
- The translation file contains correct metadata linking it to the source ID.
- `--dry-run` reports the planned target path and metadata without writing.
- The operation fails with a clear diagnostic if the source document cannot be found.
- The created translation preserves arqix markup directives and structural elements according to the scaffold strategy.

## Notes

### In scope
- `arqix doc new <kind> --lang <target> --translation-of <ID>`:
  - resolves the source document in `source_lang`
  - routes the translation file deterministically using the configured i18n layout
  - writes translation metadata:
    - `lang`, `translation_of`, `source_updated`, `translated` (optional), `translation_status` (optional)
  - uses configured templates for the target language (or a fallback strategy)
  - supports `--dry-run` with planned id/path output

### Out of scope
- Automatic translation of prose
- Automatic rewriting of existing translations

This is intentionally not a separate top-level i18n command.
