---
id: REQ-01-01-03-02
title: Normalise Directives without Semantic Changes
slug: normalise-directives-without-semantic-changes
iri: arqix:requirements/req-01-01-03-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-03
      - arqix:user-stories/us-02-01-03
      - arqix:user-stories/us-08-01-03
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Directive attribute order and whitespace are canonical after formatting and assembly output is semantically unchanged.

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

When `arqix fmt` runs, arqix SHALL normalise directives, including attribute order and whitespace, without semantic changes.

### Notes

Derived from the acceptance criteria of US-01-01-03 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`). Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
