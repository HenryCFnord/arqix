---
id: REQ-04-01-03-07
title: Forward Render Tool Errors Transparently
slug: forward-render-tool-errors-transparently
iri: arqix:requirements/req-04-01-03-07

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-03
      - arqix:user-stories/us-06-01-05
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: The tool's error output reaches the user unaltered together with the failing invocation context.

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

If an external rendering tool fails, then arqix SHALL forward the tool error transparently.

### Notes

Derived from the acceptance criteria of US-04-01-03, US-06-01-05 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
