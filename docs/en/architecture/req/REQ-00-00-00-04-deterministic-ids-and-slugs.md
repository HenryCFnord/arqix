---
id: REQ-00-00-00-04
title: Deterministic IDs and Slugs
slug: deterministic-ids-and-slugs
iri: arqix:requirements/req-00-00-00-04

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-01
      - arqix:user-stories/us-01-01-04
      - arqix:user-stories/us-01-01-05
      - arqix:user-stories/us-01-01-12
      - arqix:user-stories/us-01-01-18
      - arqix:user-stories/us-02-01-01
      - arqix:user-stories/us-02-01-04
      - arqix:user-stories/us-02-01-05
      - arqix:user-stories/us-03-01-01
      - arqix:user-stories/us-05-01-10
      - arqix:user-stories/us-06-01-03
      - arqix:user-stories/us-06-01-10
      - arqix:user-stories/us-08-01-01
      - arqix:user-stories/us-08-01-04
      - arqix:user-stories/us-08-01-05
      - arqix:user-stories/us-08-01-09
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: The same title and slug-rule configuration always yields the same ID and slug, duplicates are detected globally, and section anchors remain stable.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-02
  updated: 2026-07-11
  lang: en
  translation-of:
  generated: false
---

## Requirement

The arqix CLI SHALL derive document IDs and slugs deterministically from the configured policy.

### Notes

Curated from acceptance criteria on deterministic id/slug derivation, placeholder substitution, duplicate-ID detection, and stable anchors/selectors.

Contributing stories: 16 (see `derived-from`).
Approved via `docs/en/plans/requirements-derivation-2026-07-02/CROSS-CONCERNS.md`.
