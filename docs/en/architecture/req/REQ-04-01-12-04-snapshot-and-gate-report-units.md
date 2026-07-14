---
id: REQ-04-01-12-04
title: Snapshot and Gate Report Units
slug: snapshot-and-gate-report-units
iri: arqix:requirements/req-04-01-12-04

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-12
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Regenerating the committed units reproduces them byte-for-byte, and `--check` exits non-zero on any staled unit.

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

When `arqix report snapshot` runs, arqix SHALL regenerate the committed report units from the trace graph and exit non-zero under `--check` when any committed unit is stale.

### Notes

The question-driven report units are deterministic projections of the trace model (ADR-0008): identical corpus, identical bytes, with the snapshot stamp injected rather than read from the wall clock.
The Python `scripts/arqix_report.py` remains the conformance oracle for the grace period (arc42 chapter 8, oracle policy); the Rust command must reproduce its output byte-for-byte.
Derived from the acceptance criteria of US-04-01-12 under the canonical-owner model.
