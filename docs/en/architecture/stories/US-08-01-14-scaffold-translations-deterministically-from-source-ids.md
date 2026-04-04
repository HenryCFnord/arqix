---


id: US-08-01-14
title: Scaffold Translations Deterministically from Source IDs
slug: scaffold-translations-deterministically-from-source-ids
iri: arqix:user-stories/us-08-01-14

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-08-01

properties:
  priority: high
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


## Scaffold Translations Deterministically from Source IDs

As a Casey Coding Agent, I want to create a translation document by referencing its source document ID, so that I can generate correctly linked translation artefacts without guessing routing, metadata, or document shape.

### Acceptance Criteria

- [ ] The command creates the translation file at the correct location for the chosen i18n layout.
- [ ] The translation file contains correct metadata linking it to the source ID.
- [ ] `--dry-run` reports the planned target path and metadata without writing.
- [ ] The operation fails with a clear diagnostic if the source document cannot be found.
- [ ] The created translation preserves arqix markup directives and structural elements according to the scaffold strategy.

### Notes

The `arqix doc new <kind> --lang <target> --translation-of <ID>` workflow should provide a deterministic and low-friction way to scaffold translation documents from an existing source note. It should resolve the source document using `source_lang`, derive the destination path from the configured i18n layout, and create the translation with the metadata needed to keep source and translation linked over time. A `--dry-run` mode should let contributors and automation agents inspect the planned id, path, and metadata without mutating the repository. Out of scope is any automatic translation of prose. The main value for Casey is deterministic scaffolding and clear failure behaviour for missing sources.
