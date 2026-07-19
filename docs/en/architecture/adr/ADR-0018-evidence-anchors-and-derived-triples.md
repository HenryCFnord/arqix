---
id: ADR-0018
title: Evidence Anchors and Derived Triples
slug: evidence-anchors-and-derived-triples
iri: arqix:adrs/adr-0018

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

## Evidence Anchors and Derived Triples

### Context

The corpus can name its sources (`arqix:classes/source`, byte-verified local copies) and export its normative statements as data, but nothing connects a statement in a document body to the source that supports it.
For an agent-built knowledge base that connection is the product: every claim answers where it came from, or it is indistinguishable from a hallucination.

Two constraints shape the mechanism.
Evidence is anchored at statement level, sparse and opt-in: an annotation sits directly above the block it supports — a sentence, a paragraph, or a section, depending on placement — and coverage is a report number, never a completeness gate.
Declared frontmatter triples are the source of truth for relations (ADR-0012), and the graph machinery — target resolution (ONT-003), domain/range contracts (ONT-007), search, exports — reads them there; an edge that lives only in a body comment is invisible to all of it.

A single mechanism must therefore serve two masters: position (the exact text the evidence supports) and graph membership (a queryable, validated edge).

### Decision

A claim is **a body anchor plus a derived frontmatter edge**, one declared by the author, the other generated from it.

1. **The anchor** is a body marker directly above the block it supports, in the established marker grammar:
   `<!-- arqix:claim supported-by=<source-iri> confidence=<value> anchor="<locus in the source>" -->`.
   `supported-by` is required; `confidence` and `anchor` are optional.
   Position-bound attributes stay in the marker: the confidence of this statement, the locus inside the source.
2. **The edge** is generated: `fmt` lifts every claim marker into a `derived-triples` frontmatter section (the same shape as `triples`, predicate `arqix:properties/supported-by`), deduplicating markers that share a target.
   The section is owned by the formatter — it contains exactly the lifted set, is absent when no markers exist, and a hand edit does not survive the next `fmt` run.
   `fmt --check` is thereby the drift gate: marker and edge cannot disagree while the gate is green.
3. **Derived triples are graph members like declared ones**: ONT-003 resolves their objects, ONT-007 checks them against `supported-by`'s declared range (`arqix:classes/source`), and every reader of the graph sees the edge without a special path.
4. **Confidence is a declared vocabulary** with the shipped default `high`, `inferred`, `estimated` — the substance of the check stays code, the values are configuration (ADR-0017), validated at the marker (CLM rules) rather than in `properties`.
5. **Only claim markers are lifted.** The existing `references-artefact` markers keep their current trace-graph path; folding them into the same lifting is a named follow-up with its own compatibility plan, not a side effect.

### Alternatives Considered

- **Everything in the marker, no frontmatter edge:** rejected — the claim never becomes a graph node; resolution, range contracts, search, and exports would each need a parallel marker path, and the one graph (ADR-0012) splits in two.
- **A claim document per annotation:** rejected for the default case — a file per supported statement collapses the sparse, opt-in authoring model; agents generate such records carelessly and humans not at all.
  The marker grammar deliberately leaves room for a future reference-valued form (a claim record carrying review provenance) without breaking the inline form.
- **Edge attributes in the frontmatter (reified triples):** rejected — a triple stays predicate and objects; per-anchor data belongs to the anchor, and readers that need it take it from the scan, not the header.
- **Hand-maintained `supported-by` triples without markers:** rejected — it loses the statement position that makes the evidence checkable against the text, and nothing keeps prose and edge in step.

### Consequences

- The frontmatter contract grows the optional `derived-triples` key after `triples`; hand-written and generated edges stay structurally separated.
- The rewriter gains the lifting as its second mechanical mutation next to `finalise` (ADR-0004: one mutator, mechanical only); `fmt --check` reports a document whose derived section disagrees with its markers.
- The CLM rule family enters `lint frontmatter`: a malformed claim marker and a confidence value outside the effective vocabulary are findings.
- The ontology gains `arqix:properties/supported-by` with `arqix:classes/source` as its declared range.
- Claim coverage (how many normative statements carry evidence) is a report number, joining the export machinery — never a gate.
- Entity identity stays out: a claim connects a statement in a document to a source; consolidating claims across documents that describe one concept is the crosswalk work's question.
