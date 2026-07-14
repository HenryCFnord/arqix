---
id: REQ-04-01-03-08
title: Support per-Package Render Configuration
slug: support-per-package-render-configuration
iri: arqix:requirements/req-04-01-03-08

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
  fit-criterion: A doc package can override the repository render configuration for its own outputs.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-02
  updated: 2026-07-14
  lang: en
  translation-of:
  generated: false
---

## Requirement

The arqix CLI SHALL support per-doc-package render configuration and overrides.

### Notes

Derived from the acceptance criteria of US-04-01-03, US-06-01-05 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
The `[policies.render]` table carries a `documents` list (`{ name, path, title? }` entries) declaring the top-level document boundaries (REQ-04-01-03-09), alongside the per-package overrides merged from `[policies.render.package.<name>]`.
