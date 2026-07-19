---
id: REQ-08-01-28-04
title: Verify Local Copy Bytes Against the Recorded Digest
slug: verify-local-copy-bytes-against-the-recorded-digest
iri: arqix:requirements/req-08-01-28-04

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-28
      - arqix:user-stories/us-08-01-34
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A source record whose local-copy bytes hash to the recorded sha256 passes; a flipped digest or a missing file is an SRC-006 finding naming the path; a record without the local-copy/sha256 pair is unaffected.

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

When `arqix lint frontmatter` checks a source record carrying a well-formed `local-copy` and `sha256` pair, arqix SHALL report the record unless the file exists and its bytes hash to the recorded digest.

### Notes

Rule SRC-006; it runs only when SRC-004 and SRC-005 are clean, so every defect has exactly one finding.
Derived from US-08-01-28.
