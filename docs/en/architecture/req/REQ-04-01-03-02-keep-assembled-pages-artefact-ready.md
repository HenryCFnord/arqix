---
id: REQ-04-01-03-02
title: Keep Assembled Pages Artefact-Ready
slug: keep-assembled-pages-artefact-ready
iri: arqix:requirements/req-04-01-03-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-03
      - arqix:user-stories/us-06-01-05
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Assembled pages can be handed to the publishing toolchain without further transformation.

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

The assembled pages SHALL be artefact-ready for downstream publishing.

### Notes

Derived from the acceptance criteria of US-04-01-03, US-06-01-05 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
