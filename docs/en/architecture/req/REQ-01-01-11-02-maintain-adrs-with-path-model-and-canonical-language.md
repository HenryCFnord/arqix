---
id: REQ-01-01-11-02
title: Maintain ADRs with Path Model and Canonical Language
slug: maintain-adrs-with-path-model-and-canonical-language
iri: arqix:requirements/req-01-01-11-02

rdf:
  type:
    - arqix:classes/constraint

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-11
      - arqix:user-stories/us-06-01-07
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Every ADR follows the path model and is authored in the canonical governance language.

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

ADRs SHALL be maintained using the path model with a canonical governance language.

### Notes

Derived from the acceptance criteria of US-01-01-11 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`). Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
