---
id: REQ-01-01-16-02
title: Render Effective Configuration
slug: render-effective-configuration
iri: arqix:requirements/req-01-01-16-02

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
  fit-criterion: The rendered configuration equals what commands actually act on, including defaults and overrides.

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

When `arqix config show` runs, arqix SHALL render the effective configuration after defaults and overrides are applied.

### Notes

Derived from the acceptance criteria of US-01-01-16 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`).
Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
