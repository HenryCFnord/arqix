---
id: ADR-0019
title: Provenance Layers
slug: provenance-layers
iri: arqix:adrs/adr-0019

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

## Provenance Layers

### Context

A claim names what supports a statement — source, confidence, locus.
Provenance answers the questions above that: who produced the claim (a person, or which agent), when, whether a person reviewed it and with what verdict, and which representation of the source the analysis actually read.
For an agent-built knowledge base these answers separate extracted-and-verified from generated-and-unreviewed, which is the difference between a knowledge repository and a plausible-looking pile.

The tension is cost.
Claims are sparse and opt-in — one marker line above the supported block — while full provenance is a record with half a dozen fields.
Forcing the record onto every claim collapses the sparse authoring model; skipping provenance entirely leaves the differentiator unbuilt.

Three observations shape the resolution.
The repository's history already carries machine provenance for free: every marker line has an author, a date, and a commit, and agent involvement is visible in commit trailers — computed facts, in the spirit of the marker-freshness arithmetic (ADR-0015).
The claim marker already carries a validated attribute dictionary, so inline fields cost one key each.
And the record pattern exists: source records (`arqix:classes/source`) show how a first-class kind carries a validated field contract.

### Decision

**One provenance vocabulary, three carriers of increasing depth, derivable upward.**

The vocabulary names the provenance facts once: the producing agent, the activity, the reviewer, the review date, the review verdict, and the analyzed representation with its digest.
Each carrier holds a subset, and the same key means the same thing on every carrier.

1. **The computed floor: history.**
   Author, date, commit, and agent involvement are read from the repository history on demand — an informational surface, never part of a gated snapshot, because history-derived values change with every commit (the report-freshness lesson).
   A corpus without history simply lacks the floor; nothing else degrades.
2. **The inline dictionary: the marker.**
   A claim marker may carry the provenance keys directly (`reviewed-by=…`, `review-status=confirmed`, …).
   The marker grammar validates them: unknown keys stay findings, and the review verdict must lie in the declared review vocabulary (shipped default `unreviewed`, `confirmed`, `rejected`).
3. **The record: the fullest form.**
   A claim record (`arqix:classes/claim`, its own kind) carries the whole vocabulary plus the analysis block — which representation of the source was read, pinned by its digest — and its review verdict is a declared domain state validated like every other declared vocabulary.
   A marker references its record through the `record=` attribute; several markers may share one record (the one-to-one case is the special case of one-to-many), while locus and confidence stay per marker.
   The reference must resolve to a claim document.
4. **Records are derivable.**
   A record can be scaffolded from what the lower carriers already know — the marker's inline fields and the history's computed facts pre-fill it — so promoting a claim to the fullest form is completion, not transcription.

`supported-by` stays the only edge form: the marker's derived triple keeps pointing at the source, and the record is a provenance sidecar, not a second kind of edge.

### Alternatives Considered

- **History only:** rejected as the whole answer — it binds provenance quality to repository discipline, carries no claim-specific review verdict, and cannot name the analyzed representation; it is the right floor and the wrong ceiling.
- **Inline only:** rejected — a marker that carries agent, activity, reviewer, verdict, representation, and digest is a record in the wrong medium, and the sparse anchor line stops being one.
- **Records only:** rejected — a file per claim collapses the sparse model; agents generate such records carelessly and humans not at all.
- **Distinct vocabularies per carrier:** rejected — three names for one fact guarantee drift; one vocabulary makes depth a choice instead of a translation.

### Consequences

- The claim-marker grammar grows the provenance keys; the review verdict joins the effective-vocabulary machinery with a shipped default.
- The ontology gains `arqix:classes/claim`; the record kind gets a directory, a template, and its field contract, following the source-record pattern.
- The `record=` reference is validated: it must resolve to a claim document.
- The history surface lands as an on-demand projection next to the claims export, present only when a repository history exists.
- The claims export and the evidence-coverage unit stay gated and history-free; the two layers never mix.
- The PROV mapping falls out per carrier: commits are activities, authors and trailers are agents, records are the reviewed entities — the semantic projection keeps a source without new bookkeeping.
