---
id: REQ-01-01-12-03
title: Detect Duplicate or Malformed Glossary IDs
slug: detect-duplicate-or-malformed-glossary-ids
iri: arqix:requirements/req-01-01-12-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-12
      - arqix:user-stories/us-06-01-10
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A duplicate or malformed glossary ID produces a lint finding.

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

When `arqix lint run` executes, arqix SHALL detect duplicate or malformed glossary IDs.

### Notes

Derived from the acceptance criteria of US-01-01-12 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`).
Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
