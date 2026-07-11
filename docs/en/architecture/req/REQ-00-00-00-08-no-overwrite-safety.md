---
id: REQ-00-00-00-08
title: No-Overwrite Safety
slug: no-overwrite-safety
iri: arqix:requirements/req-00-00-00-08

rdf:
  type:
    - arqix:classes/constraint

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-01
      - arqix:user-stories/us-01-01-06
      - arqix:user-stories/us-02-01-01
      - arqix:user-stories/us-02-01-08
      - arqix:user-stories/us-08-01-01
      - arqix:user-stories/us-08-01-06
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Creation commands refuse to overwrite existing files unless an explicit override is provided, and `finalise` performs only mechanical metadata changes without rewriting body text.

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

The arqix CLI SHALL NOT overwrite existing files without explicit approval.

### Notes

Curated from acceptance criteria on overwrite refusal and mechanical-only metadata finalisation.

Contributing stories: 6 (see `derived-from`).
Approved via `docs/en/plans/requirements-derivation-2026-07-02/CROSS-CONCERNS.md`.
