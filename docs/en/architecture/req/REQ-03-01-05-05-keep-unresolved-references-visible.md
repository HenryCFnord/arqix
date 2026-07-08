---
id: REQ-03-01-05-05
title: Keep Unresolved References Visible
slug: keep-unresolved-references-visible
iri: arqix:requirements/req-03-01-05-05

rdf:
  type:
    - arqix:classes/constraint

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-03-01-05
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: An unresolved reference appears in the report marked as unresolved.

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

The arqix CLI SHALL NOT silently drop unresolved references from trace reports.

### Notes

Derived from the acceptance criteria of US-03-01-05 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
