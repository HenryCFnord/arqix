---
id: US-04-01-14
title: Configure the Verification Loop
slug: configure-the-verification-loop
iri: arqix:user-stories/us-04-01-14

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-09
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-04-01-14-01
      - arqix:requirements/req-04-01-14-02
      - arqix:requirements/req-04-01-14-03
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-04-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-10
  updated: 2026-07-12
  lang: en
  translation-of:
  generated: false
---

## Configure the Verification Loop

As a DevOps engineer, I want to configure which sub-steps `arqix verify` runs and how each result is treated, so that the loop fits the repository's lifecycle stage instead of blocking healthy spec-first states.

### Acceptance Criteria

- [ ] `arqix.toml` declares the verify sub-steps and their order; `verify` runs exactly those.
- [ ] A sub-step can be marked informational: its findings are reported but do not affect the exit code.
- [ ] Without configuration, coverage is informational and every other sub-step gates.

### Notes

Coverage measures project progress, so an absolute number must never gate a change (ADR-0010 discussion, refinement 2026-07-09).
Once shipped, CI switches its gate from `scripts/arqix verify` to `arqix verify`; the Python sequencer is demoted to a cross-check per the oracle policy (arc42 chapter 8).
