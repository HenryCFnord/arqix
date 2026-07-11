---
id: REQ-02-01-06-01
title: Provide Full-Text Search
slug: provide-full-text-search
iri: arqix:requirements/req-02-01-06-01

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
  fit-criterion: A query matching document content returns the containing documents; full-text matching is sufficient for the first version.

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

The arqix CLI SHALL provide full-text search over the documentation.

### Notes

Derived from the acceptance criteria of US-02-01-06 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
