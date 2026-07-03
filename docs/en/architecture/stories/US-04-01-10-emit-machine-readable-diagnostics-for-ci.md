---
id: US-04-01-10
title: Emit Machine-Readable Diagnostics for CI
slug: emit-machine-readable-diagnostics-for-ci
iri: arqix:user-stories/us-04-01-10

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-04
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-01
      - arqix:requirements/req-00-00-00-02
      - arqix:requirements/req-00-00-00-03
      - arqix:requirements/req-04-01-10-01
      - arqix:requirements/req-04-01-10-02
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-04-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-30
  updated: 2026-07-02
  lang: en
  translation-of:
  generated: false
---


## Emit Machine-Readable Diagnostics for CI

As a DevOps engineer, I want arqix commands to emit machine-readable diagnostics, so that CI can classify failures reliably and surface actionable build feedback without scraping human-oriented output.

### Acceptance Criteria

- [ ] Each supported command accepts `--format json` or an equivalent option to emit JSON diagnostics.
- [ ] JSON diagnostics include at minimum `severity`, `code`, `message`, `source.path`, and `source.line` when available.
- [ ] Commands produce exit codes consistent with the documented failure contract.
- [ ] Diagnostics output is deterministic for identical inputs.

### Notes

In scope are JSON diagnostics for `fmt`, `lint`, `trace scan`, `trace matrix`, and coverage reporting, including stable ordering and source locations. Out of scope are full structured logging, rich UI formatting, and auto-fix behaviour beyond existing formatting. The main value for Daria is stable automation contracts for CI and pipeline reporting.
