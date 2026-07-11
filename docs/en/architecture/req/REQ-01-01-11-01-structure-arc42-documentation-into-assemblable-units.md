---
id: REQ-01-01-11-01
title: Structure Arc42 Documentation into Assemblable Units
slug: structure-arc42-documentation-into-assemblable-units
iri: arqix:requirements/req-01-01-11-01

rdf:
  type:
    - arqix:classes/constraint

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-11
      - arqix:user-stories/us-06-01-07
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Each arc42 chapter is a unit and `assemble` produces the complete architecture document.

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

The arc42 architecture document SHALL be structured into units per chapter and remain assemblable into one document.

### Notes

Derived from the acceptance criteria of US-01-01-11 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`).
Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
