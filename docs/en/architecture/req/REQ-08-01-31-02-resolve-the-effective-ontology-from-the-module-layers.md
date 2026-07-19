---
id: REQ-08-01-31-02
title: Resolve the Effective Ontology From the Module Layers
slug: resolve-the-effective-ontology-from-the-module-layers
iri: arqix:requirements/req-08-01-31-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-31
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A corpus without docs/ontology whose arqix.toml selects story-driven passes lint frontmatter with rdf.type arqix:classes/user-story resolved from the shipped module vocabulary; with modules = [] the same type is an ONT-002 finding; an unconfigured corpus has every shipped module; a corpus definition of a module IRI overrides the embedded one.

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

When `arqix lint frontmatter` validates a corpus, arqix SHALL resolve the effective ontology as three layers in rising precedence — the reserved core, the shipped vocabularies of the effective process modules, and the corpus's own ontology documents.

### Notes

The module vocabularies are embedded in the binary from the authoring corpus's ontology documents (ADR-0021); a corpus without a `docs/ontology` directory is valid, and deselecting a module removes its vocabulary from the effective ontology.
Derived from US-08-01-31.
