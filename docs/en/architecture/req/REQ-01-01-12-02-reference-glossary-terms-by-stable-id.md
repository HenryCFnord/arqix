---
id: REQ-01-01-12-02
title: Reference Glossary Terms by Stable ID
slug: reference-glossary-terms-by-stable-id
iri: arqix:requirements/req-01-01-12-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object: arqix:user-stories/us-01-01-12
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A glossary reference by ID resolves from ADRs and other documents and survives renames of the term title.

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-02
  updated: 2026-07-02
  lang: en
  translation-of:
  generated: false
---

## Requirement

The arqix CLI SHALL support referencing glossary terms by stable ID from ADRs and other documents.

### Notes

Derived from the acceptance criteria of US-01-01-12 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`). Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
