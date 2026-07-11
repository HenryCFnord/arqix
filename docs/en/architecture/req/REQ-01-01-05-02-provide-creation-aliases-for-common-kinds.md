---
id: REQ-01-01-05-02
title: Provide Creation Aliases for Common Kinds
slug: provide-creation-aliases-for-common-kinds
iri: arqix:requirements/req-01-01-05-02

rdf:
  type:
    - arqix:classes/quality-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-05
      - arqix:user-stories/us-02-01-05
      - arqix:user-stories/us-06-01-03
      - arqix:user-stories/us-08-01-05
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Each of the aliases `req new`, `us new`, and `adr new` exists, or the absence of each missing alias is clearly documented in the `doc new` help.

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

The arqix CLI SHOULD provide the aliases `req new`, `us new`, and `adr new` for template-based creation.

### Notes

Derived from the acceptance criteria of US-01-01-05 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`).
Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
