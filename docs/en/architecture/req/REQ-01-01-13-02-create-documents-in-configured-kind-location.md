---
id: REQ-01-01-13-02
title: Create Documents in Configured Kind Location
slug: create-documents-in-configured-kind-location
iri: arqix:requirements/req-01-01-13-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-13
      - arqix:user-stories/us-02-01-07
      - arqix:user-stories/us-08-01-23
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: The created file appears at the target path the kind's routing configuration defines.

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

When `arqix doc new <kind>` is invoked with a title, arqix SHALL create the document in the configured location for that kind.

### Notes

Derived from the acceptance criteria of US-01-01-13 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`). Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
