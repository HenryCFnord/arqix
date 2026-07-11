---
id: REQ-01-01-07-01
title: Declare Change Scope in Policy Files
slug: declare-change-scope-in-policy-files
iri: arqix:requirements/req-01-01-07-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-07
      - arqix:user-stories/us-04-01-02
      - arqix:user-stories/us-08-01-08
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A policy file in the documented format is parsed and drives `policy check` evaluation.

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

The arqix CLI SHALL support a policy file in minimal YAML or TOML that declares the allowed change scope.

### Notes

Derived from the acceptance criteria of US-01-01-07 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`).
Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
