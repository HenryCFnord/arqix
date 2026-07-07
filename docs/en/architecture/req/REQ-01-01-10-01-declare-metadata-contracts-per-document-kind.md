---
id: REQ-01-01-10-01
title: Declare Metadata Contracts per Document Kind
slug: declare-metadata-contracts-per-document-kind
iri: arqix:requirements/req-01-01-10-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-10
      - arqix:user-stories/us-05-01-03
      - arqix:user-stories/us-08-01-10
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A document kind can declare its required and optional fields in a schema contract that lint consumes.

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

The arqix CLI SHALL support schema contracts that declare required and optional metadata fields per document kind.

### Notes

Derived from the acceptance criteria of US-01-01-10 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`).
Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
