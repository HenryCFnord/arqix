---
id: US-04-01-01
title: Emit a CI-Friendly Assembly Log
slug: emit-ci-friendly-assembly-log
iri: arqix:user-stories/us-04-01-01

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-04
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-03
      - arqix:requirements/req-00-00-00-06
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-04-01

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


## Emit a CI-Friendly Assembly Log

As a DevOps engineer, I want a machine-readable log during assembly, so that I can capture deterministic build evidence and diagnose include-related failures in CI.

### Acceptance Criteria

- [ ] `arqix assemble build` writes a JSONL log during assembly.
- [ ] The log path is configurable.
- [ ] Each assembly step emits one stable JSONL record.
- [ ] Each record contains at least `doc`, `chapter_id`, `out`, `include`, `sha256`, `bytes`, and `at_line`.
- [ ] The log can be collected as a CI artefact without post-processing or field-name guessing.

### Notes

This is done when each assembly step emits one stable JSONL record that CI workflows and downstream tools can consume directly.
Add a test that checks the required keys and verifies the logged hash and byte count against a known include.
If logging can be disabled or redirected, document that behavior in CLI help and CI examples.
The main value for a DevOps engineer is reproducible automation, actionable pipeline diagnostics, and stable artefact contracts.