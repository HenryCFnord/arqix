---


id: US-08-01-19
title: Detect Missing Trace Markers for a Requirement
slug: detect-missing-trace-markers-for-a-requirement
iri: arqix:user-stories/us-08-01-19

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-08-01

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


## Detect Missing Trace Markers for a Requirement

As a Casey Coding Agent, I want arqix to detect missing `implements` and `verifies` markers for a given requirement across code and tests, so that I can add only the missing annotations and avoid unnecessary edits.

### Acceptance Criteria

- [ ] A command such as `arqix trace check --req REQ-xxxx` reports whether any `implements` markers exist for the given requirement.
- [ ] The command reports whether any `verifies` markers exist for the given requirement.
- [ ] The command reports locations of existing markers with path and line context.
- [ ] Output can be emitted as JSON for automation consumption.

### Notes

In scope is a focused command that reports existing and missing marker coverage for a chosen requirement ID. Out of scope are automatic insertion of markers and language-specific parsing beyond marker detection. The main value is scoped, minimal editing.
