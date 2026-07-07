---
id: REQ-01-01-15-02
title: Document SemVer and Versioned Contracts
slug: document-semver-and-versioned-contracts
iri: arqix:requirements/req-01-01-15-02

rdf:
  type:
    - arqix:classes/constraint

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-15
      - arqix:user-stories/us-04-01-09
      - arqix:user-stories/us-08-01-17
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: RELEASING.md states how each of the three versions increments and what triggers a major change.

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

The release process SHALL document SemVer rules for the product version and the separate `config_version` and `schema_version`.

### Notes

Derived from the acceptance criteria of US-01-01-15 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`).
Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
