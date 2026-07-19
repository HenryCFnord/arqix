---
id: REQ-08-01-30-01
title: Report Dangling Triple Objects
slug: report-dangling-triple-objects
iri: arqix:requirements/req-08-01-30-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-30
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A corpus containing a triple whose arqix-namespace object matches no document iri exits non-zero with a finding naming the object and the file; a non-arqix object produces no finding.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-19
  updated: 2026-07-19
  lang: en
  translation-of:
  generated: false
---

## Requirement

When `arqix lint frontmatter` runs, arqix SHALL report every declared triple whose object is an `arqix:` IRI that no corpus document carries as its `iri`.

### Notes

Rule ONT-007; the frontmatter-graph counterpart of the body-marker resolution (LNT-003).
Derived from US-08-01-30.
