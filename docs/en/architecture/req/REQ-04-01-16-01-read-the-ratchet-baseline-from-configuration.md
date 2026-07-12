---
id: REQ-04-01-16-01
title: Read the Ratchet Baseline from Configuration
slug: read-the-ratchet-baseline-from-configuration
iri: arqix:requirements/req-04-01-16-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-16
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: With a ratchet-baseline path declared in configuration, the ratchet compares against that file; an explicit --baseline argument still overrides it.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-12
  updated: 2026-07-12
  lang: en
  translation-of:
  generated: false
---

## Requirement

Where the verify policy declares a ratchet baseline path, arqix SHALL compare the coverage ratchet against that baseline.

### Notes

Derived from US-04-01-16 (config-audit row C17: the snapshot strategy and the baseline source are configuration, not convention).
The resolution order is: explicit `--baseline` argument, configured path, the built-in default snapshot location.
