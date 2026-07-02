---
id: REQ-01-01-03-03
title: Preserve Document Meaning when Formatting
slug: preserve-document-meaning-when-formatting
iri: arqix:requirements/req-01-01-03-03

rdf:
  type:
    - arqix:classes/constraint

triples:
  - predicate: arqix:properties/derived-from
    object: arqix:user-stories/us-01-01-03
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Formatting any conforming document leaves its rendered content and semantics unchanged.

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

The arqix CLI SHALL NOT change document meaning during formatting beyond canonical ordering and whitespace normalisation.

### Notes

Derived from the acceptance criteria of US-01-01-03 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`). Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
