---
id: REQ-01-01-05-01
title: Accept Only Configured Document Kinds
slug: accept-only-configured-document-kinds
iri: arqix:requirements/req-01-01-05-01

rdf:
  type:
    - arqix:classes/functional-requirement

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
  fit-criterion: A kind that is not configured is rejected with a diagnostic; configured kinds are accepted.

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

The arqix CLI SHALL accept only `<kind>` values that are defined in configuration.

### Notes

Derived from the acceptance criteria of US-01-01-05 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`).
Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
