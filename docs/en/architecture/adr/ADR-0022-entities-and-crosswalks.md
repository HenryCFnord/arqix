---
id: ADR-0022
title: Entities and Crosswalks
slug: entities-and-crosswalks
iri: arqix:adrs/adr-0022

rdf:
  type:
    - arqix:classes/adr

triples: []

properties:
  decision-status: accepted

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-19
  updated: 2026-07-19
  lang: en
  translation-of:
  generated: false
---

## Entities and Crosswalks

### Context

Two related gaps close together here.
First, the corpus cannot say that several documents describe one thing: the document is the graph node (ADR-0007), so a term described once per bounded context has no shared identity, and a mapping onto an external standard has no canonical place to live.
Second, mappings onto external vocabularies are expressible but ungoverned: a declared triple whose predicate lies outside the `arqix:` namespace passes every check silently — a typo in `skos:exactMatch` is invisible, and no projection collects the mapping edges into a reviewable crosswalk.

### Decision

**Entities are documents of their own kind; mappings are declared module vocabulary; external predicates join the declared-list discipline; the crosswalk is a report unit.**

1. **The entity kind.**
   A domain entity is a corpus document (`arqix:classes/entity`, a knowledge artefact) with its own kind contract — directory, template, id pattern — exactly like source and claim records.
   Because the entity is a document, the one identity rule survives untouched: the entity node is a document node, and no second identity mechanism appears.
   Descriptions attach through `arqix:properties/describes`, whose declared range is the entity class — several documents describing one entity is now plain declared triples, resolved by ONT-003 and range-checked by ONT-007.
   Entities are opt-in: a corpus without entity documents loses nothing.
2. **The mapping vocabulary.**
   The knowledge-base module carries five mapping properties: `maps-to` as the general edge and `exact-match`, `close-match`, `broader-match`, `narrower-match` as its refinements — SKOS-inspired arqix vocabulary, defined as ordinary ontology documents.
   The mapping target stays the external IRI; domain and range stay undeclared, so any document may map (the entity is the canonical home, not the only one).
   A semantic SKOS projection can fall out later, exactly as the PROV mapping falls out of the provenance carriers (ADR-0019).
3. **External predicates are declared.**
   `[frontmatter].allowed-external-properties` lists the external predicates a corpus accepts; a declared triple naming an undeclared external predicate is ONT-010 — the same discipline `allowed-external-types` already applies to `rdf.type` (ONT-002).
   The shipped default is empty: a corpus using only arqix vocabulary is unaffected.
4. **The crosswalk unit.**
   A question unit (ADR-0008) projects every mapping edge — the mapping document, the mapping property, the external target — grouped by the target's namespace, under the snapshot drift gate like every unit.

### Alternatives Considered

- **A `describes` convention without an entity kind** (entity IRIs as bare names): rejected — it creates graph nodes no document defines, a second identity rule beside ADR-0007, and nothing for ONT-003 to resolve.
- **Keeping the document as the only node** (mappings on the describing documents alone): rejected — the shared-entity case has no home, and the crosswalk fragments across contexts.
- **SKOS predicates directly** (`skos:exactMatch` as the mapping edge): rejected for the declaration mechanism it would need — external property definitions with domain and range — while arqix-namespaced properties get validation from the existing ONT machinery for free; the projection recovers SKOS when an export needs it.
- **A configurable mapping-predicate list instead of shipped vocabulary:** rejected — corpora stop sharing mapping semantics, and the crosswalk unit degrades to a per-project query.

### Consequences

- The knowledge-base module grows the entity class, `describes`, and the five mapping properties; the entity kind gets directory, template, and id pattern as project configuration.
- The ONT family gains ONT-010; external-predicate typos surface at lint time.
- The crosswalk joins the gated report units; a corpus without mappings renders an empty crosswalk.
- The graph explorer and every other projection see entities and mapping edges without change — they are ordinary documents and triples.
