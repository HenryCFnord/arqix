---
id: REQ-04-01-05-03
title: Emit per-Step Results in JSON Mode
slug: emit-per-step-results-in-json-mode
iri: arqix:requirements/req-04-01-05-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-05
      - arqix:user-stories/us-08-01-13
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: The JSON output lists every executed sub-step with its result and diagnostic references.

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

Where JSON mode is enabled, the verification loop SHALL emit per-step results and diagnostic references.

### Notes

Derived from the acceptance criteria of US-04-01-05, US-08-01-13 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
