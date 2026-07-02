---
id: REQ-06-01-02-01
title: Make Assembly Reviewable from the Log
slug: make-assembly-reviewable-from-the-log
iri: arqix:requirements/req-06-01-02-01

rdf:
  type:
    - arqix:classes/quality-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-06-01-02
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A reviewer can reconstruct the composition from the log alone.

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

The assembly log SHOULD allow document composition to be reviewed without inferring hidden assembly steps.

### Notes

Derived from the acceptance criteria of US-06-01-02 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10). Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
