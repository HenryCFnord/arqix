---
id: REQ-01-01-06-03
title: Fail Clearly on Unsupported Frontmatter
slug: fail-clearly-on-unsupported-frontmatter
iri: arqix:requirements/req-01-01-06-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-06
      - arqix:user-stories/us-02-01-08
      - arqix:user-stories/us-08-01-06
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Finalising a file without supported frontmatter yields a failing status and a diagnostic naming the file.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-02
  updated: 2026-07-10
  lang: en
  translation-of:
  generated: false
---

## Requirement

If a file has no supported frontmatter, then `arqix finalise` SHALL fail with a clear diagnostic.

### Notes

Derived from the acceptance criteria of US-01-01-06 during the group-01 pilot derivation (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`).
Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
