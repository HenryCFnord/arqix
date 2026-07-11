---
id: REQ-04-01-01-03
title: Configure Assembly Log Path
slug: configure-assembly-log-path
iri: arqix:requirements/req-04-01-01-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-01
      - arqix:user-stories/us-05-01-02
      - arqix:user-stories/us-06-01-02
      - arqix:user-stories/us-08-01-02
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Changing the configured path changes where the log is written.

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

The assembly log path SHALL be configurable.

### Notes

Derived from the acceptance criteria of US-04-01-01, US-05-01-02, US-06-01-02, US-08-01-02 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
