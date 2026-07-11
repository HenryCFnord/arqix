---
id: REQ-04-01-13-02
title: Resolve Architecture Cross-Links Consistently
slug: resolve-architecture-cross-links-consistently
iri: arqix:requirements/req-04-01-13-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-13
      - arqix:user-stories/us-06-01-11
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Every cross-link of the three artefact kinds resolves to the same target in every output.

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

Cross-links between ADRs, glossary terms, and architecture pages SHALL resolve consistently in assembled outputs.

### Notes

Derived from the acceptance criteria of US-04-01-13, US-06-01-11 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
