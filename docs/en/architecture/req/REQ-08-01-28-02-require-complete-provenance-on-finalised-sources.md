---
id: REQ-08-01-28-02
title: Require Complete Provenance on Finalised Sources
slug: require-complete-provenance-on-finalised-sources
iri: arqix:requirements/req-08-01-28-02

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
  priority: high
  fit-criterion: A source document whose lifecycle-status is not draft and which lacks properties.uri or properties.accessed makes the check exit non-zero and names each missing field, and a document carrying local-copy without sha256 (or the reverse) does so in any lifecycle state.

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

When `arqix lint frontmatter` runs, arqix SHALL report every finalised source document that lacks the uri or the access date and every source document that carries only one of local-copy and sha256.

### Notes

Rule SRC-002; finalised means the record has left draft (ADR-0010 prose lifecycle), so a fresh `doc new source` skeleton passes the default gates while an incomplete published record does not.
A source whose licence forbids redistribution finalises without a local copy — no path, no digest; when a copy is held, path and digest travel together (owner ruling 2026-07-16).
The optional fields licence and anchor carry no completeness rule.
Derived from US-08-01-28 (knowledge-repository slice K3, gap G5).
