---

id: US-06-01-06
title: Create linked translation documents for architecture content
slug: create-linked-translation-documents-for-architecture-content
iri: arqix:user-stories/us-06-01-06

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-06
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-06-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-30
  updated: 2026-04-04
  lang: en
  translation-of:
  generated: false
---

## User-story

As a Aria Architect, I want to create a translation document by referencing its source document ID, so that ADRs and other architecture artefacts can be translated with correct structure and linkage from the start.

### Acceptance Criteria

- [ ] The command creates the translation file at the correct location for the chosen i18n layout.
- [ ] The translation file contains correct metadata linking it to the source ID.
- [ ] `--dry-run` reports the planned target path and metadata without writing.
- [ ] The operation fails with a clear diagnostic if the source document cannot be found.
- [ ] The created translation preserves arqix markup directives and structural elements according to the scaffold strategy.

### Notes

The `arqix doc new <kind> --lang <target> --translation-of <ID>` workflow should provide a deterministic and low-friction way to scaffold translation documents from an existing source note. It should resolve the source document using `source_lang`, derive the destination path from the configured i18n layout, and create the translation with the metadata needed to keep source and translation linked over time. A `--dry-run` mode should let contributors and automation agents inspect the planned id, path, and metadata without mutating the repository. Out of scope is any automatic translation of prose. The main value for Aria is consistent bilingual architecture documentation.
