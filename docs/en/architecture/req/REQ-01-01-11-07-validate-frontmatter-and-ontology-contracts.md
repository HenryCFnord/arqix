---
id: REQ-01-01-11-07
title: Validate Frontmatter and Ontology Contracts
slug: validate-frontmatter-and-ontology-contracts
iri: arqix:requirements/req-01-01-11-07

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-11
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A document whose frontmatter opening, canonical key order, whitespace, dates, or language breaks the format contract, whose id, iri, slug, heading, required keys, or duplicate identity breaks the family scheme, or whose section-kind, lifecycle vocabulary, or ontology predicate, class, or index entry leaves the controlled vocabulary makes the check exit non-zero and names the offending document and rule.

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

When `arqix lint frontmatter` runs, arqix SHALL report every architecture and ontology document whose frontmatter, canonical formatting, or ontology-vocabulary use violates the declared corpus contract.

### Notes

Realises the documentation consistency check that REQ-01-01-11-05 records as an extension path, porting the reference checker `scripts/check_frontmatter.py` (the FMT, FM, and ONT rule families) into the binary as a self-hosting slice of the oracle policy (arc42 chapter 8, roadmap phase 5 item 9).
It complements the requirement-authoring check of REQ-01-01-11-06 and reads the same configured per-family contract the formatter reads, the one-source rule that REQ-01-01-19-02 governs.
Cross-cutting behaviour is linked via the story's `has-requirement`, not restated here.
</content>
</invoke>
