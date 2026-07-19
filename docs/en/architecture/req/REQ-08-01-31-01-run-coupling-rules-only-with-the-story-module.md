---
id: REQ-08-01-31-01
title: Run Coupling Rules Only With the Story Module
slug: run-coupling-rules-only-with-the-story-module
iri: arqix:requirements/req-08-01-31-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-31
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: A corpus with [process].modules = ["knowledge-base"] reports no US-WF findings on a story whose declared workflow contradicts its id; the same corpus without the [process] section reports US-WF-001.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-19
  updated: 2026-07-19
  lang: en
  translation-of:
  generated: false
---

## Requirement

When `arqix lint requirements` runs, arqix SHALL apply the story-workflow coupling rules exactly when the story-driven process module is effective — listed in the configured `[process].modules`, or unconfigured entirely.

### Notes

Rules US-WF-001 and US-PER-001; the first module binding of the process catalog (ADR-0017).
Derived from US-08-01-31.
