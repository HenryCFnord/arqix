---
id: REQ-03-01-11-01
title: Detect Stale Trace Markers
slug: detect-stale-markers
iri: arqix:requirements/req-03-01-11-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-03-01-11
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: In a repository where a requirement was committed after its verifying test, arqix trace freshness lists that marker as possibly stale; when the test is the later commit, it does not.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-13
  updated: 2026-07-13
  lang: en
  translation-of:
  generated: false
---

## Requirement

When `arqix trace freshness` finds an active marker whose target requirement changed in version control after the marker's own file, arqix SHALL report the marker as possibly stale.

### Notes

Derived from US-03-01-11.
"Active" excludes ignored skeleton markers; the comparison is file-level over the last commit that touched each path (ADR-0015).
The requirement document is the compared contract; the owning story is a grouping layer whose churn is not a staleness signal, so it is deliberately not compared (ADR-0015).
