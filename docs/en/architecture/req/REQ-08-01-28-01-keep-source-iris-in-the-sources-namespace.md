---
id: REQ-08-01-28-01
title: Keep Source IRIs in the Sources Namespace
slug: keep-source-iris-in-the-sources-namespace
iri: arqix:requirements/req-08-01-28-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-28
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: A document typed arqix:classes/source whose iri is not arqix:sources/ followed by its lowercased id makes the check exit non-zero and names the expected iri.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-16
  updated: 2026-07-16
  lang: en
  translation-of:
  generated: false
---

## Requirement

When `arqix lint frontmatter` runs, arqix SHALL report every source document whose iri does not lie in the sources namespace derived from its id.

### Notes

Rule SRC-001; the identity convention every other first-class kind carries (FM-003 for the architecture families), keyed on the class rather than a directory.
Derived from US-08-01-28 (knowledge-repository slice K3, gap G5).
