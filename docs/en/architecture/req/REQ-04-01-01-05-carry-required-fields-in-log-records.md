---
id: REQ-04-01-01-05
title: Carry Required Fields in Log Records
slug: carry-required-fields-in-log-records
iri: arqix:requirements/req-04-01-01-05

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
  fit-criterion: Every record carries all seven fields; absent values are explicit, not missing keys.

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

Each assembly log record SHALL contain at least `doc`, `chapter_id`, `out`, `include`, `sha256`, `bytes`, and `at_line`.

### Notes

Derived from the acceptance criteria of US-04-01-01, US-05-01-02, US-06-01-02, US-08-01-02 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10). Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
