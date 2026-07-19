---
id: REQ-08-01-40-06
title: Carry Provenance Inline on the Marker
slug: carry-provenance-inline-on-the-marker
iri: arqix:requirements/req-08-01-40-06

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-40
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: A claim marker carrying reviewed-by, reviewed, review-status, agent, activity, representation, or representation-sha256 passes; review-status outside the effective vocabulary (default unreviewed/confirmed/rejected) is CLM-003; unknown keys stay CLM-001.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-19
  updated: 2026-07-19
  lang: en
  translation-of:
  generated: false
---

## Requirement

When `arqix lint frontmatter` checks a claim marker, arqix SHALL accept the provenance keys of the shared vocabulary and report every `review-status` outside the effective review vocabulary.

### Notes

Rule CLM-003; the effective vocabulary is `[frontmatter].claim-review-status`, defaulting to unreviewed, confirmed, rejected (ADR-0019 carrier two).
Derived from US-08-01-40.
