---
id: REQ-01-01-01-01
title: Create Standard Doc-Package Scaffold
slug: create-standard-doc-package-scaffold
iri: arqix:requirements/req-01-01-01-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object: arqix:user-stories/us-01-01-01
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Initialising an empty target path produces exactly the standard directory and file scaffold without manual cleanup.

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

When `arqix doc init <path>` is invoked, arqix SHALL create the standard doc-package scaffold with `index.md`, `units/`, `pages/`, `artefacts/`, `logs/`, and `.arqix/`.

### Notes

Derived from the acceptance criteria of US-01-01-01 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`). Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
