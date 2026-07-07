---
id: REQ-00-00-00-02
title: Stable Exit Codes
slug: stable-exit-codes
iri: arqix:requirements/req-00-00-00-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-14
      - arqix:user-stories/us-04-01-04
      - arqix:user-stories/us-04-01-05
      - arqix:user-stories/us-04-01-07
      - arqix:user-stories/us-04-01-08
      - arqix:user-stories/us-04-01-10
      - arqix:user-stories/us-05-01-05
      - arqix:user-stories/us-05-01-13
      - arqix:user-stories/us-05-01-14
      - arqix:user-stories/us-08-01-11
      - arqix:user-stories/us-08-01-13
      - arqix:user-stories/us-08-01-15
      - arqix:user-stories/us-08-01-21
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Exit codes follow the documented contract (0 success, 1 findings or gate failure, 2 usage error), are stable across releases, and identical for repeated runs on the same input.

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

The arqix CLI SHALL signal command outcomes through documented, stable exit codes.

### Notes

Curated from acceptance criteria demanding exit-code contracts and CI-gate behaviour.

Contributing stories: 13 (see `derived-from`).
Approved via `docs/en/plans/requirements-derivation-2026-07-02/CROSS-CONCERNS.md`.
