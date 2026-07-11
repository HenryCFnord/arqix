---
id: US-01-01-19
title: Configure Frontmatter Contracts
slug: configure-frontmatter-contracts
iri: arqix:user-stories/us-01-01-19

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-01
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-06
      - arqix:requirements/req-01-01-19-01
      - arqix:requirements/req-01-01-19-02
      - arqix:requirements/req-01-01-19-03
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-01-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-11
  updated: 2026-07-11
  lang: en
  translation-of:
  generated: false
---

## Configure Frontmatter Contracts

As a maintainer, I want the per-family frontmatter contract — canonical key order and required meta keys — declared once in configuration, so that the formatter and the validators can never disagree about what a conforming document looks like.

### Acceptance Criteria

- [ ] `arqix.toml` declares the canonical key order per document family; `fmt` orders keys by it.
- [ ] Frontmatter validation reads the same configured source as the formatter; no second copy of the contract lives in code.
- [ ] The required meta keys come from the same configuration, and the document templates satisfy them.
- [ ] Without configuration, the defaults reproduce today's contract and `fmt` stays byte-identical on the existing corpus.

### Notes

This story carries the audit rows C1, C2, and C6 (key order per family, family-to-directory mapping, required meta keys).
C6 is the motivating incident: the two reference checkers had already diverged on the required meta keys before the parity fix — double bookkeeping demonstrably does not hold.
The formatter/checker coupling is the sharp edge (ADR-0011): `fmt` must never format what the checker then flags.
