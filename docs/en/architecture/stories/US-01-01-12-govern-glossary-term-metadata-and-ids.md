---


id: US-01-01-12
title: Govern Glossary Term Metadata and IDs
slug: govern-glossary-term-metadata-and-ids
iri: arqix:user-stories/us-01-01-12

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-01
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-01-01

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


## Govern Glossary Term Metadata and IDs

As a Mara Maintainer, I want glossary terms to be scaffolded with stable IDs and linted metadata, so that terminology artefacts remain compliant and traceable across the repository.

### Acceptance Criteria

- [ ] `arqix doc new glossary` creates a glossary term with required metadata and deterministic routing.
- [ ] Glossary terms can be referenced by stable ID from ADRs and other documents.
- [ ] `arqix lint run` detects duplicate or malformed glossary IDs.

### Notes

The main value for Mara is metadata governance and uniqueness enforcement for glossary artefacts.
