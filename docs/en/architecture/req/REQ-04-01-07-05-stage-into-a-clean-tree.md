---
id: REQ-04-01-07-05
title: Stage into a Clean Tree
slug: stage-into-a-clean-tree
iri: arqix:requirements/req-04-01-07-05

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-07
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A stale page present in the staging directory before publish site is absent afterwards, so a local build and a fresh-checkout CI build stage byte-identical trees.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-16
  updated: 2026-07-16
  lang: en
  translation-of:
  generated: false
---

## Requirement

When `arqix publish site` stages a language, arqix SHALL remove that language's previously staged tree first.

### Notes

Staging is generated output; carrying stale pages forward made a local build (old site-src) disagree with CI (fresh checkout) — the scoreboard 404 hid behind exactly that divergence.
Derived from US-04-01-07.
