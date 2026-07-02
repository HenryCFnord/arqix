---
id: REQ-01-01-16-01
title: Validate Configuration against Schema
slug: validate-configuration-against-schema
iri: arqix:requirements/req-01-01-16-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-16
      - arqix:user-stories/us-04-01-11
      - arqix:user-stories/us-05-01-11
      - arqix:user-stories/us-08-01-20
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: An invalid configuration produces findings naming each violated schema rule or contract.

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

When `arqix config validate` runs, arqix SHALL report schema and contract violations.

### Notes

Derived from the acceptance criteria of US-01-01-16 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`). Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
