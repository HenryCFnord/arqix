---
id: REQ-08-01-43-03
title: Render the Crosswalk Unit
slug: render-the-crosswalk-unit
iri: arqix:requirements/req-08-01-43-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-43
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: report snapshot renders crosswalk.md with one row per mapping edge (document, mapping property, external target) grouped by target namespace; the unit sits under the snapshot drift gate and renders empty for a corpus without mappings.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-19
  updated: 2026-07-19
  lang: en
  translation-of:
  generated: false
---

## Requirement

When `arqix report snapshot` runs, arqix SHALL render the crosswalk unit with one row per mapping edge — the mapping document, the mapping property, and the external target.

### Notes

An ADR-0008 question unit over the five mapping predicates (ADR-0022), grouped by the target's namespace prefix, gated like every unit.
Derived from US-08-01-43.
