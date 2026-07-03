---
id: REQ-00-00-00-06
title: Effective Configuration as Baseline
slug: effective-configuration-as-baseline
iri: arqix:requirements/req-00-00-00-06

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-01
      - arqix:user-stories/us-01-01-02
      - arqix:user-stories/us-01-01-03
      - arqix:user-stories/us-01-01-04
      - arqix:user-stories/us-01-01-05
      - arqix:user-stories/us-01-01-13
      - arqix:user-stories/us-01-01-14
      - arqix:user-stories/us-01-01-16
      - arqix:user-stories/us-02-01-02
      - arqix:user-stories/us-02-01-03
      - arqix:user-stories/us-02-01-04
      - arqix:user-stories/us-02-01-05
      - arqix:user-stories/us-02-01-07
      - arqix:user-stories/us-02-01-09
      - arqix:user-stories/us-02-01-11
      - arqix:user-stories/us-03-01-01
      - arqix:user-stories/us-03-01-05
      - arqix:user-stories/us-03-01-08
      - arqix:user-stories/us-04-01-01
      - arqix:user-stories/us-04-01-03
      - arqix:user-stories/us-04-01-04
      - arqix:user-stories/us-04-01-05
      - arqix:user-stories/us-04-01-06
      - arqix:user-stories/us-04-01-07
      - arqix:user-stories/us-04-01-11
      - arqix:user-stories/us-05-01-01
      - arqix:user-stories/us-05-01-02
      - arqix:user-stories/us-05-01-04
      - arqix:user-stories/us-05-01-05
      - arqix:user-stories/us-05-01-07
      - arqix:user-stories/us-05-01-09
      - arqix:user-stories/us-05-01-11
      - arqix:user-stories/us-05-01-13
      - arqix:user-stories/us-06-01-01
      - arqix:user-stories/us-06-01-02
      - arqix:user-stories/us-06-01-03
      - arqix:user-stories/us-06-01-04
      - arqix:user-stories/us-06-01-05
      - arqix:user-stories/us-06-01-08
      - arqix:user-stories/us-07-01-04
      - arqix:user-stories/us-07-01-06
      - arqix:user-stories/us-08-01-01
      - arqix:user-stories/us-08-01-02
      - arqix:user-stories/us-08-01-03
      - arqix:user-stories/us-08-01-04
      - arqix:user-stories/us-08-01-05
      - arqix:user-stories/us-08-01-11
      - arqix:user-stories/us-08-01-13
      - arqix:user-stories/us-08-01-16
      - arqix:user-stories/us-08-01-20
      - arqix:user-stories/us-08-01-22
      - arqix:user-stories/us-08-01-23
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Every command reads its settings exclusively from the effective configuration, and `config show` renders exactly the configuration commands act on.

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

The arqix CLI SHALL resolve every command against the effective configuration.

### Notes

Curated from acceptance criteria demanding configuration-driven behaviour (configured locations, kinds, policies, profiles) or effective-config inspection.

Contributing stories: 52 (see `derived-from`). Approved via `docs/en/plans/requirements-derivation-2026-07-02/CROSS-CONCERNS.md`.
