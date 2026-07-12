---
id: US-04-01-16
title: Configure the Snapshot Strategy
slug: configure-the-snapshot-strategy
iri: arqix:user-stories/us-04-01-16

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-09
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-04-01-16-01
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-04-01

external-references: []

meta:
  lifecycle-status: done
  owner: hcf
  created: 2026-07-12
  updated: 2026-07-12
  lang: en
  translation-of:
  generated: false
---

## Configure the Snapshot Strategy

As a builder, I want the report snapshot strategy and the ratchet baseline source declared in configuration, so that parallel branches are not forced into a rebase-before-merge dance by committed snapshots unless the repository chooses that trade-off.

### Acceptance Criteria

- [x] `[policies.verify] ratchet-baseline` names the baseline file the ratchet compares against; an explicit `--baseline` argument still overrides it.
- [x] `[policies.reports] snapshot-strategy` declares how the freshness gate treats committed snapshots: `committed` (gate everywhere, the default), `main-only` (gate only on the default branch), or `on-demand` (never gate).
- [x] Without configuration, the defaults reproduce the present behaviour: committed snapshots, freshness gated everywhere, the committed matrix as baseline.

### Notes

Carries config-audit row C17 (added 2026-07-11 after the parallel-merge friction became visible: six mechanical snapshot rebases in one review round).
The freshness gate lives in the reference sequencer until the self-hosting slice, so `snapshot-strategy` is read there; the ratchet baseline is the product's own concern and is specified as REQ-04-01-16-01.
The `main-only` strategy shifts snapshot regeneration to the default branch; how main regains freshness after a merge (auto-commit vs. next change) is an open owner decision recorded in the configuration schema.
