---
id: REQ-00-00-00-03
title: Machine-Readable Diagnostics
slug: machine-readable-diagnostics
iri: arqix:requirements/req-00-00-00-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-07
      - arqix:user-stories/us-01-01-08
      - arqix:user-stories/us-01-01-14
      - arqix:user-stories/us-03-01-03
      - arqix:user-stories/us-03-01-05
      - arqix:user-stories/us-03-01-06
      - arqix:user-stories/us-03-01-08
      - arqix:user-stories/us-04-01-01
      - arqix:user-stories/us-04-01-02
      - arqix:user-stories/us-04-01-04
      - arqix:user-stories/us-04-01-05
      - arqix:user-stories/us-04-01-07
      - arqix:user-stories/us-04-01-10
      - arqix:user-stories/us-04-01-12
      - arqix:user-stories/us-05-01-02
      - arqix:user-stories/us-05-01-05
      - arqix:user-stories/us-05-01-07
      - arqix:user-stories/us-05-01-08
      - arqix:user-stories/us-05-01-13
      - arqix:user-stories/us-05-01-14
      - arqix:user-stories/us-06-01-02
      - arqix:user-stories/us-07-01-01
      - arqix:user-stories/us-07-01-04
      - arqix:user-stories/us-07-01-06
      - arqix:user-stories/us-07-01-07
      - arqix:user-stories/us-08-01-02
      - arqix:user-stories/us-08-01-07
      - arqix:user-stories/us-08-01-08
      - arqix:user-stories/us-08-01-11
      - arqix:user-stories/us-08-01-13
      - arqix:user-stories/us-08-01-16
      - arqix:user-stories/us-08-01-19
      - arqix:user-stories/us-08-01-21
      - arqix:user-stories/us-08-01-22
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Every diagnostic is available in a documented JSON form including severity, stable code, message, and source location where available.

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

When arqix emits a diagnostic, arqix SHALL provide it in a documented machine-readable format.

### Notes

Curated from acceptance criteria demanding JSON/JSONL output, structured diagnostics, or stable diagnostic codes. Criteria that only demand clear or actionable human-facing diagnostics were excluded.

Contributing stories: 34 (see `derived-from`). Approved via `docs/en/plans/requirements-derivation-2026-07-02/CROSS-CONCERNS.md`.
