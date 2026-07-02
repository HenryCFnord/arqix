---
id: REQ-01-01-09-02
title: Define Editing Constraints for PLANS.md
slug: define-editing-constraints-for-plans-md
iri: arqix:requirements/req-01-01-09-02

rdf:
  type:
    - arqix:classes/constraint

triples:
  - predicate: arqix:properties/derived-from
    object: arqix:user-stories/us-01-01-09
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: AGENTS.md states which parts of PLANS.md agents edit and which verification commands must pass.

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

AGENTS.md SHALL define editing constraints for PLANS.md and the required arqix verification loop.

### Notes

Derived from the acceptance criteria of US-01-01-09 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`). Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
