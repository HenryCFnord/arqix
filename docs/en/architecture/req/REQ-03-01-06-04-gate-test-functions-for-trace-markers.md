---
id: REQ-03-01-06-04
title: Gate Test Functions for Trace Markers
slug: gate-test-functions-for-trace-markers
iri: arqix:requirements/req-03-01-06-04

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-03-01-06
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A test without a verifies/plans marker or a no-requirement annotation, a marker to an unknown requirement, an ignored test naming an unknown story, or a broken derived-from/has-requirement backlink each makes the gate exit non-zero and names the offending location.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-13
  updated: 2026-07-15
  lang: en
  translation-of:
  generated: false
---

## Requirement

When `arqix trace markers` runs, arqix SHALL report every test function without a `verifies` or `plans` marker or a no-requirement annotation, every marker not resolving to an existing requirement, every ignored test whose reason does not name a known owning story, and every derived-from without its has-requirement backlink.

### Notes

Ports the TDD marker gate of the retired `scripts/check_trace_markers.py` (TRC-001 through TRC-006) into the binary as the self-hosting slice of the oracle policy (arc42 chapter 8, roadmap phase 5 item 9); its selftest cases are mirrored in the trace engine's test module.
Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
