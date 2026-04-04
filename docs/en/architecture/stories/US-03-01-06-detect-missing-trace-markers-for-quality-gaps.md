---

id: US-03-01-06
title: Detect missing trace markers for quality gaps
slug: detect-missing-trace-markers-for-quality-gaps
iri: arqix:user-stories/us-03-01-06

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-03
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-03-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-30
  updated: 2026-04-04
  lang: en
  translation-of:
  generated: false
---

## User-story

As a Quinn QA, I want arqix to detect missing `implements` and `verifies` markers for a given requirement, so that structural traceability gaps can be identified objectively before review or audit.

### Acceptance Criteria

- [ ] A command such as `arqix trace check --req REQ-xxxx` reports whether any `implements` markers exist for the given requirement.
- [ ] The command reports whether any `verifies` markers exist for the given requirement.
- [ ] The command reports locations of existing markers with path and line context.
- [ ] Output can be emitted as JSON for automation consumption.

### Notes

The main value for Quinn is objective detection of missing trace markers with precise locations, so gaps can be assigned and resolved reproducibly.
