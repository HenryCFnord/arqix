---
id: REQ-02-01-09-01
title: Parse Chapter and Include Directives
slug: parse-chapter-and-include-directives
iri: arqix:requirements/req-02-01-09-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-02-01-09
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Both directive forms are recognised in document bodies and drive assembly.

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-02
  updated: 2026-07-02
  lang: en
  translation-of:
  generated: false
---

## Requirement

The arqix CLI SHALL parse `<!-- arqix:chapter ... -->` and `<!-- arqix:include ... -->` directives.

### Notes

Derived from the acceptance criteria of US-02-01-09 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
