---
id: REQ-01-01-07-02
title: Evaluate Changed Files against Policy
slug: evaluate-changed-files-against-policy
iri: arqix:requirements/req-01-01-07-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object: arqix:user-stories/us-01-01-07
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Changed files outside the declared scope produce violations; files inside the scope pass.

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

When `arqix policy check` is invoked with a list of changed files, arqix SHALL evaluate them against the declared policy.

### Notes

Derived from the acceptance criteria of US-01-01-07 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`). Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
