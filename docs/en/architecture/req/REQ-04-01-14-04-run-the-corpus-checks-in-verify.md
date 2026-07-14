---
id: REQ-04-01-14-04
title: Run the Corpus Checks in Verify
slug: run-the-corpus-checks-in-verify
iri: arqix:requirements/req-04-01-14-04

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-14
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Under a profile that lists them, `verify --format json` reports the `requirements`, `frontmatter`, `markers`, and `report-freshness` sub-steps; a seeded requirements, frontmatter, or marker violation fails `verify` on that sub-step.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-13
  updated: 2026-07-13
  lang: en
  translation-of:
  generated: false
---

## Requirement

When `arqix verify` runs, arqix SHALL run the requirements, frontmatter, trace-marker, and report-freshness corpus checks as sub-steps of the configured profile.

### Notes

Derived from US-04-01-14: self-hosting slice 5 wires the ported corpus checks (`lint requirements`, `lint frontmatter`, `trace markers`, `report snapshot --check`) into the product's own orchestrator, so `arqix verify` covers the reference sequencer's corpus steps (arc42 chapter 8, oracle policy).
The checks are members of the configured `[policies.verify]` profile — arqix lists them in its own arqix.toml — rather than the hard-coded default, because they need a populated corpus and a fresh `doc init` package must still pass `verify` (REQ-08-01-01-02).
The corpus checkers gate like the reference sequencer treats them; report-freshness follows the snapshot strategy (REQ-04-01-14-05).
