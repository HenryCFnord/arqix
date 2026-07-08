---
id: US-06-01-02
title: Trace Document Assembly Structure
slug: trace-document-assembly-structure
iri: arqix:user-stories/us-06-01-02

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-06
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-03
      - arqix:requirements/req-00-00-00-06
      - arqix:requirements/req-04-01-01-02
      - arqix:requirements/req-04-01-01-03
      - arqix:requirements/req-04-01-01-04
      - arqix:requirements/req-04-01-01-05
      - arqix:requirements/req-06-01-02-01
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-06-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-29
  updated: 2026-07-02
  lang: en
  translation-of:
  generated: false
---

## Trace Document Assembly Structure

As an architect, I want a machine-readable log during assembly, so that I can trace how modular documents are composed from includes and outputs.

### Acceptance Criteria

- [ ] `arqix assemble build` writes a JSONL log during assembly.
- [ ] The log path is configurable.
- [ ] Each assembly step emits one stable JSONL record.
- [ ] Each record contains at least `doc`, `chapter_id`, `out`, `include`, `sha256`, `bytes`, and `at_line`.
- [ ] The log allows document composition to be reviewed without inferring hidden assembly steps.

### Notes

This is done when each assembly step emits one stable JSONL record that makes include structure and generated outputs traceable.
Add a test that checks the required keys and verifies the logged hash and byte count against a known include.
If logging can be disabled or redirected, document the behavior clearly in CLI help and author-facing examples.
The main value for an architect is explainable assembly of modular architecture documents.
