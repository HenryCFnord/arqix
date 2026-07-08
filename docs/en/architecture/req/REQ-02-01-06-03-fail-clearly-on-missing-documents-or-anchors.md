---
id: REQ-02-01-06-03
title: Fail Clearly on Missing Documents or Anchors
slug: fail-clearly-on-missing-documents-or-anchors
iri: arqix:requirements/req-02-01-06-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-02-01-06
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A miss yields a failing status and a diagnostic that names the missing document or anchor.

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

If a document or anchor cannot be found, then arqix SHALL fail with a diagnostic naming the missing element.

### Notes

Derived from the acceptance criteria of US-02-01-06 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10).
Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
