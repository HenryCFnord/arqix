---
id: REQ-03-01-02-03
title: Keep CSV Headers and Row Model Stable
slug: keep-csv-headers-and-row-model-stable
iri: arqix:requirements/req-03-01-02-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-03-01-02
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Headers and the row model per matrix type do not change between runs or minor releases.

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

The exported CSV SHALL use stable headers and a deterministic row model for each supported matrix type.

### Notes

Derived from the acceptance criteria of US-03-01-02 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
