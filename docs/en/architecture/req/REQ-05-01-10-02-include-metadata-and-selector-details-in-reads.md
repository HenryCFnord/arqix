---
id: REQ-05-01-10-02
title: Include Metadata and Selector Details in Reads
slug: include-metadata-and-selector-details-in-reads
iri: arqix:requirements/req-05-01-10-02

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
  fit-criterion: Both the document metadata and the resolved selector appear in the structured output.

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

The structured read output SHALL include the resolved document metadata and selector details.

### Notes

Derived from the acceptance criteria of US-05-01-10, US-08-01-09 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10). Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
