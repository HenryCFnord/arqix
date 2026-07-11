---
id: REQ-02-01-06-02
title: Read Documents by ID
slug: read-documents-by-id
iri: arqix:requirements/req-02-01-06-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-02-01-06
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Reading by ID returns the document; adding a section or anchor returns only that part.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-02
  updated: 2026-07-10
  lang: en
  translation-of:
  generated: false
---

## Requirement

When `arqix doc read` is invoked with a document ID, arqix SHALL return the document, optionally scoped to a section or anchor.

### Notes

Derived from the acceptance criteria of US-02-01-06 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
