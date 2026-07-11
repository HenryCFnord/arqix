---
id: US-04-01-15
title: Gate Coverage as a Ratchet
slug: gate-coverage-as-a-ratchet
iri: arqix:user-stories/us-04-01-15

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-04
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-04-01-15-01
      - arqix:requirements/req-04-01-15-02
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
  updated: 2026-07-11
  lang: en
  translation-of:
  generated: false
---

## Gate Coverage as a Ratchet

As a DevOps engineer, I want changes that reduce verified coverage to fail the gate, so that specification growth stays free while regressions cannot land silently.

### Acceptance Criteria

- [ ] A change that removes the last active verifying test of a requirement fails the ratchet, naming that requirement.
- [ ] A change that only adds stories or requirements passes the ratchet.
- [ ] The committed report snapshots serve as the baseline; no second bookkeeping file is introduced.

### Notes

The report-freshness gate already forces the snapshots to be regenerated with every change, so the baseline is always the pre-change state of the same files.
The snapshot strategy and the ratchet baseline source are configuration (config audit C17): committed snapshots by default; a baseline computed from the merge target removes the rebase-before-merge constraint for parallel work.
