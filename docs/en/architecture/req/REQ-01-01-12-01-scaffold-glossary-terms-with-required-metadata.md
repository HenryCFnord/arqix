---
id: REQ-01-01-12-01
title: Scaffold Glossary Terms with Required Metadata
slug: scaffold-glossary-terms-with-required-metadata
iri: arqix:requirements/req-01-01-12-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object: arqix:user-stories/us-01-01-12
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A new glossary term lands in the configured location and carries all required metadata fields.

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

When `arqix doc new glossary` is invoked, arqix SHALL create a glossary term with the required metadata and route it to the configured location.

### Notes

Derived from the acceptance criteria of US-01-01-12 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`). Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
