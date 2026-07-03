---
id: REQ-04-01-01-02
title: Write JSONL Log during Assembly
slug: write-jsonl-log-during-assembly
iri: arqix:requirements/req-04-01-01-02

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
  fit-criterion: An assembly run leaves a JSONL log file behind.

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

When `arqix assemble build` runs, arqix SHALL write a JSONL log during assembly.

### Notes

Derived from the acceptance criteria of US-04-01-01, US-05-01-02, US-06-01-02, US-08-01-02 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10). Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
