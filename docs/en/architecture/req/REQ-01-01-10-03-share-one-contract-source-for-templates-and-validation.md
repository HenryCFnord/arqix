---
id: REQ-01-01-10-03
title: Share One Contract Source for Templates and Validation
slug: share-one-contract-source-for-templates-and-validation
iri: arqix:requirements/req-01-01-10-03

rdf:
  type:
    - arqix:classes/constraint

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-10
      - arqix:user-stories/us-05-01-03
      - arqix:user-stories/us-08-01-10
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Changing a contract updates both template scaffolding and lint validation without a second edit.

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

The template and validation subsystems SHALL use the same contract source.

### Notes

Derived from the acceptance criteria of US-01-01-10 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`).
Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
