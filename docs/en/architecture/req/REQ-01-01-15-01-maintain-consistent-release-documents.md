---
id: REQ-01-01-15-01
title: Maintain Consistent Release Documents
slug: maintain-consistent-release-documents
iri: arqix:requirements/req-01-01-15-01

rdf:
  type:
    - arqix:classes/constraint

triples:
  - predicate: arqix:properties/derived-from
    object: arqix:user-stories/us-01-01-15
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Both documents exist and describe the same release process and version history without contradiction.

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

The repository SHALL maintain `CHANGELOG.md` and `RELEASING.md` consistently with each other.

### Notes

Derived from the acceptance criteria of US-01-01-15 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`). Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
