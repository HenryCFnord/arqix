---
id: REQ-03-01-11-03
title: Report Freshness in the Verification Loop
slug: report-freshness-in-the-verification-loop
iri: arqix:requirements/req-03-01-11-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-03-01-11
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: With freshness in the configured sub-steps, arqix verify lists it as an informational step and a stale marker leaves the loop's overall result passing.

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

Where the verification loop runs the freshness sub-step, `arqix verify` SHALL report stale markers without failing the loop.

### Notes

Derived from US-03-01-11.
Freshness joins the loop as an informational sub-step like coverage (REQ-04-01-14-03): findings are surfaced and forgiven, so a possibly-stale marker prompts review without blocking the gate.
