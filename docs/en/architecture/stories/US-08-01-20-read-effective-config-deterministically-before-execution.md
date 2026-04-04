---



id: US-08-01-20
title: Read Effective Config Deterministically Before Execution
slug: read-effective-config-deterministically-before-execution
iri: arqix:user-stories/us-08-01-20

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


## Read Effective Config Deterministically Before Execution

As a coding agent, I want to validate repository configuration and inspect the effective config, so that I can execute work against the actual active rules without guessing defaults or overrides.

### Acceptance Criteria

- [ ] `arqix config validate` reports schema and contract violations deterministically.
- [ ] `arqix config show` renders the effective configuration after defaults and overrides are applied.
- [ ] Diagnostics identify the failing key and source file when possible.

### Notes

Acceptance should verify both validation and effective configuration rendering on realistic multi-layer setups with defaults and overrides. Add tests for invalid keys, invalid values, and precedence resolution. Keep diagnostics stable so automation and CI can compare failures reliably. The main value for Casey is deterministic execution against explicit configuration.
