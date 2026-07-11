---
id: REQ-03-01-04-01
title: Export Evidence Bundles by ID Scope
slug: export-evidence-bundles-by-id-scope
iri: arqix:requirements/req-03-01-04-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-03-01-04
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: The bundle contains only evidence belonging to the selected IDs.

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

When an evidence bundle export is invoked with one or more requirement or story IDs, arqix SHALL export a bundle scoped to those IDs.

### Notes

Derived from the acceptance criteria of US-03-01-04 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
