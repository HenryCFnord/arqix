---
id: REQ-01-01-14-01
title: Configure Translation-Required Kinds
slug: configure-translation-required-kinds
iri: arqix:requirements/req-01-01-14-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-14
      - arqix:user-stories/us-04-01-04
      - arqix:user-stories/us-05-01-05
      - arqix:user-stories/us-08-01-11
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Changing the configured set changes which documents the i18n lint profile requires translations for.

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

The set of translation-required kinds or domains SHALL be configurable in `arqix.toml`.

### Notes

Derived from the acceptance criteria of US-01-01-14 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`). Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
