---
id: REQ-00-00-00-11
title: Fast Documentation Retrieval
slug: fast-documentation-retrieval
iri: arqix:requirements/req-00-00-00-11

rdf:
  type:
    - arqix:classes/quality-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-02-01-06
      - arqix:user-stories/us-05-01-06
      - arqix:user-stories/us-06-01-09
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: On a 1000-document repository, `doc search` and `doc read` return within one second on developer hardware (initial budget, to be calibrated).

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-03
  updated: 2026-07-10
  lang: en
  translation-of:
  generated: false
---

## Requirement

Search and read commands SHOULD return results within one second on a repository of one thousand documents.

### Notes

Performance requirement from the NFR pass; traces to the search/read stories, including US-06-01-09 whose story sentence explicitly demands quick retrieval.
