---
id: REQ-02-01-10-03
title: Fail Clearly on Missing Translation Source
slug: fail-clearly-on-missing-translation-source
iri: arqix:requirements/req-02-01-10-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-02-01-10
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Scaffolding against a missing source yields a failing status and a diagnostic naming the source.

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

If the source document cannot be found, then the translation scaffolding SHALL fail with a clear diagnostic.

### Notes

Derived from the acceptance criteria of US-02-01-10 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
