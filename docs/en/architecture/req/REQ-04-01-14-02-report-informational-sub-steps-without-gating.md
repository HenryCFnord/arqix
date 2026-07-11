---
id: REQ-04-01-14-02
title: Report Informational Sub-Steps without Gating
slug: report-informational-sub-steps-without-gating
iri: arqix:requirements/req-04-01-14-02

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
  fit-criterion: A failing sub-step marked informational appears in the per-step report while the overall exit code stays 0.

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

Where a verify sub-step is configured as informational, arqix SHALL report its findings without affecting the exit code.

### Notes

Derived from US-04-01-14.
The per-step wire schema (ICD) carries the gating flag so consumers can distinguish the channels.
