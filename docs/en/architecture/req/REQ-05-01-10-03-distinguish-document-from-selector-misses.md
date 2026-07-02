---
id: REQ-05-01-10-03
title: Distinguish Document from Selector Misses
slug: distinguish-document-from-selector-misses
iri: arqix:requirements/req-05-01-10-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-05-01-10
      - arqix:user-stories/us-08-01-09
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: The failure diagnostic names the missing element class (document vs selector).

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

If a read fails, then the diagnostic SHALL identify whether the document or the selector was not found.

### Notes

Derived from the acceptance criteria of US-05-01-10, US-08-01-09 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10). Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
