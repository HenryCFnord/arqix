---
id: REQ-02-01-09-01
title: Parse Include Directives with Level Arguments
slug: parse-include-directives-with-level-arguments
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
  lifecycle-status: active
  owner: hcf
  created: 2026-07-02
  updated: 2026-07-11
  lang: en
  translation-of:
  generated: false
---

## Requirement

The arqix CLI SHALL parse `<!-- arqix:include ... -->` directives, including their optional heading-level argument.

### Notes

Derived from the acceptance criteria of US-02-01-09 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
Reworded with ADR-0013: the `arqix:chapter` directive is retired from the grammar, the heading-level argument (absolute or relative) joins it.
