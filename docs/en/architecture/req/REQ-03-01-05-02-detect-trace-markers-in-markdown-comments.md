---
id: REQ-03-01-05-02
title: Detect Trace Markers in Markdown Comments
slug: detect-trace-markers-in-markdown-comments
iri: arqix:requirements/req-03-01-05-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-03-01-05
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Markers inside Markdown HTML comments are found.

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

When `arqix trace scan` runs, arqix SHALL detect trace markers in Markdown HTML comments.

### Notes

Derived from the acceptance criteria of US-03-01-05 under the canonical-owner model (see `docs/en/plans/requirements-derivation-2026-07-02/PLANS.md`, decision 10). Cross-cutting behaviour is linked via the stories' `has-requirement`, not restated here.
