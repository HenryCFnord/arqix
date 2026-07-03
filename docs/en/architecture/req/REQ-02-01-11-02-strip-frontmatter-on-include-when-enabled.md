---
id: REQ-02-01-11-02
title: Strip Frontmatter on Include when Enabled
slug: strip-frontmatter-on-include-when-enabled
iri: arqix:requirements/req-02-01-11-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-02-01-11
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: With the option enabled, included content carries no frontmatter block; disabled, it is preserved.

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

Where `strip_frontmatter_on_include` is enabled, arqix SHALL strip frontmatter from included content.

### Notes

Derived from the acceptance criteria of US-02-01-11 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10). Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
