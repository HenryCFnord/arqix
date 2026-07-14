---
id: REQ-04-01-14-05
title: Gate Report Freshness by Snapshot Strategy
slug: gate-report-freshness-by-snapshot-strategy
iri: arqix:requirements/req-04-01-14-05

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
  fit-criterion: With `snapshot-strategy = "main-only"` off the default branch, `verify --format json` marks the `report-freshness` sub-step skipped and it does not affect the exit code.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-13
  updated: 2026-07-13
  lang: en
  translation-of:
  generated: false
---

## Requirement

Where the configured snapshot strategy does not gate report freshness in the current context, `arqix verify` SHALL skip the report-freshness sub-step.

### Notes

Derived from US-04-01-14 and the snapshot strategy (config-audit row C17, US-04-01-16).
The strategy `committed` gates everywhere, `on-demand` never gates, and `main-only` gates only on the default branch — the resolution the reference sequencer performs (`scripts/arqix`, step 9).
A skipped sub-step is reported in the per-step result (`skipped: true`) so the gate is visible, and it never contributes a finding, so a legitimately stale snapshot on a parallel branch never fails the loop.
