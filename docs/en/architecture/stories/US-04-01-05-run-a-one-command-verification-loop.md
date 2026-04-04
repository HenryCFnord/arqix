---

id: US-04-01-05
title: Run a one-command verification loop
slug: run-a-one-command-verification-loop
iri: arqix:user-stories/us-04-01-05

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-04
  - predicate: arqix:properties/has-requirement
    object:
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
  updated: 2026-04-04
  lang: en
  translation-of:
  generated: false
---

## User-story

As a DevOps Daria, I want a single arqix command to run the standard verification loop, so that automation and CI can validate changes with one deterministic invocation.

### Acceptance Criteria

- [ ] `arqix check` or `arqix verify` runs the configured sub-steps of format, lint, trace scan, and coverage.
- [ ] The command can fail fast or aggregate results, according to configuration.
- [ ] Exit code reflects overall status using the standard contract.
- [ ] A JSON mode emits per-step results and diagnostic references.
- [ ] Rendering is not part of the default check loop.

### Notes

In scope is a single verification command that runs formatting, lint, trace scan, and coverage, and returns a consolidated summary. Out of scope are rendering in the default check loop and auto-fix beyond formatting. Add tests for fail-fast and aggregate modes, exit-code behavior, and structured JSON output. This is the canonical CI and automation orchestration view.
