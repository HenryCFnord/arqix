---



id: US-08-01-13
title: Run One-Command Verification in Agent Workflows
slug: run-one-command-verification-in-agent-workflows
iri: arqix:user-stories/us-08-01-13

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


## Run One-Command Verification in Agent Workflows

As a coding agent, I want a single arqix command to run the standard verification loop, so that I can validate story-scoped changes with one deterministic invocation and one consolidated result.

### Acceptance Criteria

- [ ] `arqix check` or `arqix verify` runs the configured sub-steps of format, lint, trace scan, and coverage.
- [ ] The command can fail fast or aggregate results, according to configuration.
- [ ] Exit code reflects overall status using the standard contract.
- [ ] A JSON mode emits per-step results and diagnostic references.
- [ ] Rendering is not part of the default check loop.

### Notes

In scope is a single verification command that runs formatting, lint, trace scan, and coverage, and returns a consolidated summary. Out of scope are rendering in the default check loop and auto-fix beyond formatting. Add tests for fail-fast and aggregate modes, exit-code behavior, and structured JSON output. The main value for Casey is deterministic verification with minimal workflow branching.
