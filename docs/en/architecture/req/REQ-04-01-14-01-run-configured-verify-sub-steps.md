---
id: REQ-04-01-14-01
title: Run Configured Verify Sub-Steps
slug: run-configured-verify-sub-steps
iri: arqix:requirements/req-04-01-14-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-14
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: With a custom step list in `arqix.toml`, `verify` runs exactly those steps in that order and no others.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-10
  updated: 2026-07-10
  lang: en
  translation-of:
  generated: false
---

## Requirement

When `arqix verify` is invoked, arqix SHALL run the sub-steps declared in the effective configuration in their configured order.

### Notes

Derived from US-04-01-14 (refinement 2026-07-09, decision D2/strand 1).
The shipped hard-coded step list (`src/verifier.rs`) becomes the default configuration.
