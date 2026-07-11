---
id: REQ-03-01-05-03
title: Read Unit Frontmatter Links
slug: read-unit-frontmatter-links
iri: arqix:requirements/req-03-01-05-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-03-01-05
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Frontmatter link fields of units contribute edges to the trace graph.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-02
  updated: 2026-07-10
  lang: en
  translation-of:
  generated: false
---

## Requirement

When `arqix trace scan` runs, arqix SHALL read unit frontmatter links such as requirements, stories, ADRs, and refs.

### Notes

Derived from the acceptance criteria of US-03-01-05 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
