---
id: REQ-04-01-08-03
title: Ship a Minimal CI Workflow Template
slug: ship-a-minimal-ci-workflow-template
iri: arqix:requirements/req-04-01-08-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-08
      - arqix:user-stories/us-08-01-15
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: If shipped, the template references only supported commands and passes on a conforming repository.

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

The arqix CLI MAY ship a minimal GitHub Actions template for typical gates, aligned with supported commands only.

### Notes

Derived from the acceptance criteria of US-04-01-08, US-08-01-15 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10). Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
