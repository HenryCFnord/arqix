---
id: REQ-04-01-07-02
title: Diagnose Site Toolchain Failures
slug: diagnose-site-toolchain-failures
iri: arqix:requirements/req-04-01-07-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-07
      - arqix:user-stories/us-05-01-13
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A toolchain failure yields exit code 2 and a diagnostic naming the tool and invocation context.

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

If the site toolchain fails, then arqix SHALL return exit code 2 with diagnostics identifying the failing tool invocation context.

### Notes

Derived from the acceptance criteria of US-04-01-07, US-05-01-13 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
