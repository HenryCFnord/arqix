---
id: US-02-01-10
title: Scaffold Translations During Implementation
slug: scaffold-translations-during-implementation
iri: arqix:user-stories/us-02-01-10

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-02
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-02-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-30
  updated: 2026-04-06
  lang: en
  translation-of:
  generated: false
---

## Scaffold Translations During Implementation

As a developer, I want to create a translation document by referencing its source document ID, so that translations are created with correct routing, metadata, and structure without manual setup.

### Acceptance Criteria

- [ ] The command creates the translation file at the correct location for the chosen i18n layout.
- [ ] The translation file contains correct metadata linking it to the source ID.
- [ ] `--dry-run` reports the planned target path and metadata without writing.
- [ ] The operation fails with a clear diagnostic if the source document cannot be found.
- [ ] The created translation preserves arqix markup directives and structural elements according to the scaffold strategy.

### Notes

The `arqix doc new <kind> --lang <target> --translation-of <ID>` workflow should provide a deterministic and low-friction way to scaffold translation documents from an existing source note.
It should resolve the source document using `source_lang`, derive the destination path from the configured i18n layout, and create the translation with the metadata needed to keep source and translation linked over time.
A `--dry-run` mode should let contributors and automation agents inspect the planned id, path, and metadata without mutating the repository.
Out of scope is any automatic translation of prose. The main value for a developer is low-friction creation of compliant translation artefacts in the normal authoring flow.
