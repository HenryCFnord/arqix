---
id: REQ-04-01-05-04
title: Exclude Rendering from Default Verification
slug: exclude-rendering-from-default-verification
iri: arqix:requirements/req-04-01-05-04

rdf:
  type:
    - arqix:classes/constraint

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-05
      - arqix:user-stories/us-08-01-13
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A default verify run finishes without invoking any rendering toolchain.

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

The default verification loop SHALL NOT include rendering.

### Notes

Derived from the acceptance criteria of US-04-01-05, US-08-01-13 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
