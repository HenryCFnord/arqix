---
id: REQ-01-01-13-01
title: Generate Unique IDs from Configured Policy
slug: generate-unique-ids-from-configured-policy
iri: arqix:requirements/req-01-01-13-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-13
      - arqix:user-stories/us-02-01-07
      - arqix:user-stories/us-08-01-23
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Creation without an explicit ID yields a policy-conforming ID that collides with no existing document.

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-02
  updated: 2026-07-09
  lang: en
  translation-of:
  generated: false
---

## Requirement

When a document is created without an explicit ID, arqix SHALL generate an ID from the configured policy and verify its uniqueness.

### Notes

Derived from the acceptance criteria of US-01-01-13 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`).
Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
