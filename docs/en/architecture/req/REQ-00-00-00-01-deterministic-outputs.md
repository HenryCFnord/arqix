---
id: REQ-00-00-00-01
title: Deterministic Outputs
slug: deterministic-outputs
iri: arqix:requirements/req-00-00-00-01

rdf:
  type:
    - arqix:classes/constraint

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-03
      - arqix:user-stories/us-01-01-04
      - arqix:user-stories/us-01-01-08
      - arqix:user-stories/us-01-01-10
      - arqix:user-stories/us-01-01-12
      - arqix:user-stories/us-01-01-16
      - arqix:user-stories/us-02-01-03
      - arqix:user-stories/us-02-01-04
      - arqix:user-stories/us-02-01-06
      - arqix:user-stories/us-02-01-09
      - arqix:user-stories/us-02-01-11
      - arqix:user-stories/us-03-01-01
      - arqix:user-stories/us-03-01-02
      - arqix:user-stories/us-03-01-03
      - arqix:user-stories/us-03-01-04
      - arqix:user-stories/us-03-01-07
      - arqix:user-stories/us-03-01-08
      - arqix:user-stories/us-04-01-06
      - arqix:user-stories/us-04-01-10
      - arqix:user-stories/us-04-01-11
      - arqix:user-stories/us-05-01-03
      - arqix:user-stories/us-05-01-04
      - arqix:user-stories/us-05-01-06
      - arqix:user-stories/us-05-01-08
      - arqix:user-stories/us-05-01-09
      - arqix:user-stories/us-05-01-10
      - arqix:user-stories/us-05-01-11
      - arqix:user-stories/us-05-01-14
      - arqix:user-stories/us-06-01-04
      - arqix:user-stories/us-06-01-08
      - arqix:user-stories/us-06-01-09
      - arqix:user-stories/us-06-01-10
      - arqix:user-stories/us-07-01-01
      - arqix:user-stories/us-07-01-02
      - arqix:user-stories/us-07-01-03
      - arqix:user-stories/us-07-01-05
      - arqix:user-stories/us-07-01-06
      - arqix:user-stories/us-08-01-01
      - arqix:user-stories/us-08-01-03
      - arqix:user-stories/us-08-01-04
      - arqix:user-stories/us-08-01-06
      - arqix:user-stories/us-08-01-07
      - arqix:user-stories/us-08-01-09
      - arqix:user-stories/us-08-01-10
      - arqix:user-stories/us-08-01-20
      - arqix:user-stories/us-08-01-21
      - arqix:user-stories/us-08-01-22
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Two consecutive runs on identical inputs and configuration produce byte-identical artefacts, reports, and ordering.

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

The arqix CLI SHALL produce byte-identical outputs for identical inputs and configuration.

### Notes

Curated from the acceptance-criteria sweep for deterministic, idempotent, reproducible, or stably ordered outputs.
Identity determinism (IDs and slugs) is covered separately by REQ-00-00-00-04; exit-code stability by REQ-00-00-00-02.

Contributing stories: 47 (see `derived-from`).
Approved via `docs/en/plans/requirements-derivation-2026-07-02/CROSS-CONCERNS.md`.
