---
id: REQ-03-01-04-02
title: Include Linked Evidence in Bundles
slug: include-linked-evidence-in-bundles
iri: arqix:requirements/req-03-01-04-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-03-01-04
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Each evidence category (requirements, stories, diagnostics, trace outputs) is present for the chosen scope.

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

The exported bundle SHALL include the linked requirements, stories, diagnostics, and trace outputs relevant to the chosen scope.

### Notes

Derived from the acceptance criteria of US-03-01-04 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10). Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
