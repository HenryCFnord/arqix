---
id: REQ-01-01-02-02
title: Support Global Unit IDs
slug: support-global-unit-ids
iri: arqix:requirements/req-01-01-02-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-02
      - arqix:user-stories/us-02-01-02
      - arqix:user-stories/us-05-01-01
      - arqix:user-stories/us-06-01-01
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A unit can carry a global ID in either supported form, and the ID participates in global uniqueness linting.

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

The arqix CLI SHALL support unit files that declare a global ID in frontmatter or via a supported directive.

### Notes

Derived from the acceptance criteria of US-01-01-02 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`).
Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
