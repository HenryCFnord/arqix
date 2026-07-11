---
id: REQ-00-00-00-07
title: Change-Scope Guardrails
slug: change-scope-guardrails
iri: arqix:requirements/req-00-00-00-07

rdf:
  type:
    - arqix:classes/constraint

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
  fit-criterion: `arqix policy check` evaluates a list of changed files against the declared policy and reports violations as structured diagnostics; arqix commands write only within their declared scope.

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

The arqix CLI SHALL NOT modify files outside the declared change scope.

### Notes

Curated manually: the guardrail stories phrase their acceptance criteria in terms of `arqix policy check` and policy files, so the regex sweep missed them.
Report-scoping criteria (evidence bundles by scope) were excluded.

Contributing stories: 3 (see `derived-from`).
Approved via `docs/en/plans/requirements-derivation-2026-07-02/CROSS-CONCERNS.md`.
