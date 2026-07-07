---
id: REQ-01-01-06-01
title: Set Updated as ISO Date on Finalise
slug: set-updated-as-iso-date-on-finalise
iri: arqix:requirements/req-01-01-06-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-06
      - arqix:user-stories/us-02-01-08
      - arqix:user-stories/us-08-01-06
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: After finalise, `updated` matches `YYYY-MM-DD` and reflects the finalisation date.

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

When `arqix finalise` runs, arqix SHALL set `updated` to an ISO-8601 date in `YYYY-MM-DD` format.

### Notes

Derived from the acceptance criteria of US-01-01-06 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`).
Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
