---
id: REQ-02-01-11-01
title: Generate Assembled Outputs under Pages
slug: generate-assembled-outputs-under-pages
iri: arqix:requirements/req-02-01-11-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-02-01-11
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Assembly of a doc package produces its outputs below the package's `pages/` directory.

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

When `arqix assemble build` runs for a doc package, arqix SHALL generate the assembled outputs under `pages/`.

### Notes

Derived from the acceptance criteria of US-02-01-11 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10). Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
