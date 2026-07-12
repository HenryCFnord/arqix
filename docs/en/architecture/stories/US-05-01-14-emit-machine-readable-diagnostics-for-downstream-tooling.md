---
id: US-05-01-14
title: Emit Machine-Readable Diagnostics for Downstream Tooling
slug: emit-machine-readable-diagnostics-for-downstream-tooling
iri: arqix:user-stories/us-05-01-14

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-10
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-01
      - arqix:requirements/req-00-00-00-02
      - arqix:requirements/req-00-00-00-03
      - arqix:requirements/req-04-01-10-01
      - arqix:requirements/req-04-01-10-02
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-05-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: retired
  owner: hcf
  created: 2026-03-30
  updated: 2026-07-12
  lang: en
  translation-of:
  generated: false
---

## Emit Machine-Readable Diagnostics for Downstream Tooling

As an AIOps engineer, I want arqix commands to emit machine-readable diagnostics, so that automation and retrieval tooling can consume failures and warnings through a stable contract.

### Acceptance Criteria

- [ ] Each supported command accepts `--format json` or an equivalent option to emit JSON diagnostics.
- [ ] JSON diagnostics include at minimum `severity`, `code`, `message`, `source.path`, and `source.line` when available.
- [ ] Commands produce exit codes consistent with the documented failure contract.
- [ ] Diagnostics output is deterministic for identical inputs.

### Notes

In scope are JSON diagnostics for `fmt`, `lint`, `trace scan`, `trace matrix`, and coverage reporting, including stable ordering and source locations.
Out of scope are full structured logging, rich UI formatting, and auto-fix behaviour beyond existing formatting.
The main value for Alex is a predictable interface for downstream systems and agent orchestration.

Retired in the consolidation sweep of 2026-07-11: this story is a persona clone — its non-cross-cutting requirements are canonically owned by US-04-01-10, and the requirements' derived-from provenance keeps this story's contribution on record.
