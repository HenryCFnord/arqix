---
id: US-06-01-10
title: Create Glossary Terms with Stable IDs
slug: create-glossary-terms-with-stable-ids
iri: arqix:user-stories/us-06-01-10

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


## Create Glossary Terms with Stable IDs

As an architect, I want to create glossary terms from a template with stable IDs, so that architecture vocabulary stays consistent across ADRs, handbooks, and requirements.

### Acceptance Criteria

- [ ] `arqix doc new glossary` creates a glossary term with required metadata and deterministic routing.
- [ ] Glossary terms can be referenced by stable ID from ADRs and other documents.
- [ ] `arqix lint run` detects duplicate or malformed glossary IDs.

### Notes

This is a gap-fill for terminology governance. The main value is consistent vocabulary that remains referenceable across architecture and governance documents.
