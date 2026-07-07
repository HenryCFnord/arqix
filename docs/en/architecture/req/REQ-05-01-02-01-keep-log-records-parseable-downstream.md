---
id: REQ-05-01-02-01
title: Keep Log Records Parseable Downstream
slug: keep-log-records-parseable-downstream
iri: arqix:requirements/req-05-01-02-01

rdf:
  type:
    - arqix:classes/quality-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-05-01-02
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A downstream parser can rely on the documented field names alone.

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

The assembly log records SHOULD be parseable by downstream tooling without guessing field names.

### Notes

Derived from the acceptance criteria of US-05-01-02 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
