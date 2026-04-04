---

id: US-05-01-02
title: Emit a machine-readable assembly log
slug: emit-machine-readable-assembly-log
iri: arqix:user-stories/us-05-01-02

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-05
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-05-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-29
  updated: 2026-04-04
  lang: en
  translation-of:
  generated: false
---

## User-story

As an AIOps engineer, I want a machine-readable log during assembly, so that I can trace include structure and outputs.

### Acceptance Criteria

- [ ] `arqix assemble build` writes a JSONL log during assembly.
- [ ] The log path is configurable.
- [ ] Each assembly step emits one stable JSONL record.
- [ ] Each record contains at least `doc`, `chapter_id`, `out`, `include`, `sha256`, `bytes`, and `at_line`.
- [ ] Downstream tooling can parse the records without guessing field names.

### Notes

This is done when each assembly step emits one stable JSONL record that downstream tooling can parse without guessing field names.
Add a test that checks the required keys and verifies the logged hash and byte count against a known include.
If logging can be disabled or redirected, capture that behavior in CLI help and examples.
This story defines a machine-readable assembly contract for automation and downstream analysis.
