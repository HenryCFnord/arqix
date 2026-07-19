---
id: class-claim
label: claim
iri: arqix:classes/claim

rdf:
  type:
    - rdfs:Class

rdfs:
  sub-class-of:
    - arqix:classes/knowledge-artefact

triples: []

properties: {}

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

## Claim

A claim record: the fullest carrier of a claim's provenance (ADR-0019).
It names the producing agent and activity, the reviewer, the review date and verdict, and the analyzed representation of the source pinned by its digest — the facts a marker's inline dictionary carries only partially and the repository history only as computed values.
Markers reference a record through their `record=` attribute; several markers may share one record, while locus and confidence stay per marker.
The record's `supported-by` triple points at the source record it accounts for; the review verdict is a declared domain state validated against the kind's vocabulary.
