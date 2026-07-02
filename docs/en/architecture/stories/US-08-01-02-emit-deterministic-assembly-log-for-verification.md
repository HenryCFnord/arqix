---
id: US-08-01-02
title: Emit a Deterministic Assembly Log for Verification
slug: emit-deterministic-assembly-log-for-verification
iri: arqix:user-stories/us-08-01-02

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-03
      - arqix:requirements/req-00-00-00-06
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-08-01

properties:
  priority: high
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


## Emit a Deterministic Assembly Log for Verification

As a coding agent, I want a machine-readable log during assembly, so that I can verify include structure and outputs without ambiguity.

### Acceptance Criteria

- [ ] `arqix assemble build` writes a JSONL log during assembly.
- [ ] The log path is configurable.
- [ ] Each assembly step emits one stable JSONL record.
- [ ] Each record contains at least `doc`, `chapter_id`, `out`, `include`, `sha256`, `bytes`, and `at_line`.
- [ ] Failure and success cases can be interpreted from the log and command result without requiring human guesswork.

### Notes

This is done when each assembly step emits one stable JSONL record that automation can parse without guessing field names.
Add a test that checks the required keys and verifies the logged hash and byte count against a known include.
If logging can be disabled or redirected, document the exact contract and expected behavior for agent workflows.
The primary value for a coding agent is deterministic verification and clear stop conditions within scoped execution.