---
id: REQ-01-01-15-04
title: Require Migration Notes for Breaking Changes
slug: require-migration-notes-for-breaking-changes
iri: arqix:requirements/req-01-01-15-04

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
  fit-criterion: Every breaking change in a release maps to a migration note and a changelog entry.

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

If a release contains breaking changes, then the release preparation SHALL include migration notes and changelog entries.

### Notes

Derived from the acceptance criteria of US-01-01-15 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`).
Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
