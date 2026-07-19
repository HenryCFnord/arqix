---
id: REQ-08-01-30-02
title: Check Edges Against Declared Domains and Ranges
slug: check-edges-against-declared-domains-and-ranges
iri: arqix:requirements/req-08-01-30-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-30
      - arqix:user-stories/us-08-01-36
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A triple whose subject types lie outside the predicate's declared domain (subclass closure included) is an ONT-007 finding, as is a resolvable object outside the declared range; predicates without declarations and external objects produce no finding.

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

When `arqix lint frontmatter` checks a declared triple whose predicate declares a domain or range, arqix SHALL report every subject or resolvable object whose types lie outside the declared classes, subclass closure included.

### Notes

Rule ONT-007; declaring `rdfs.domain`/`rdfs.range` opts a property into the contract.
Derived from US-08-01-30.
