---
id: REQ-01-01-02-01
title: Create Unit Files in Configured Location
slug: create-unit-files-in-configured-location
iri: arqix:requirements/req-01-01-02-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object: arqix:user-stories/us-01-01-02
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A new unit file appears in the configured unit location with optional frontmatter following repository configuration.

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

When `arqix unit new` is invoked, arqix SHALL create a unit file in the configured unit location.

### Notes

Derived from the acceptance criteria of US-01-01-02 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`). Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
