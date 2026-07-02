---
id: REQ-01-01-11-03
title: Span Documentation Layers Consistently
slug: span-documentation-layers-consistently
iri: arqix:requirements/req-01-01-11-03

rdf:
  type:
    - arqix:classes/constraint

triples:
  - predicate: arqix:properties/derived-from
    object: arqix:user-stories/us-01-01-11
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Each layer exists and its scope is defined; no capability is documented in zero layers.

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

The documentation strategy SHALL span the handbook, CLI help, man page, and rustdoc layers.

### Notes

Derived from the acceptance criteria of US-01-01-11 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`). Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
