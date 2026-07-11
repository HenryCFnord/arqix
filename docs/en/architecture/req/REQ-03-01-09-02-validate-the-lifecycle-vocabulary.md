---
id: REQ-03-01-09-02
title: Validate the Lifecycle Vocabulary
slug: validate-the-lifecycle-vocabulary
iri: arqix:requirements/req-03-01-09-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-03-01-09
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A document declaring a lifecycle-status outside its nature's vocabulary is a frontmatter error.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-10
  updated: 2026-07-10
  lang: en
  translation-of:
  generated: false
---

## Requirement

When a document declares a `lifecycle-status` outside the controlled vocabulary for its document nature, arqix SHALL report a frontmatter error.

### Notes

Derived from US-03-01-09; the per-nature vocabularies are fixed in ADR-0010.
Stories: draft, specified, in-implementation, done, retired. Requirements: active, retired (the gate refutes draft — nothing half-authored can merge). Prose documents: draft, final, retired.
