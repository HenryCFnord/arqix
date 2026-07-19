---
id: US-08-01-40
title: Anchor Derive and Account for Evidence
slug: anchor-derive-and-account-for-evidence
iri: arqix:user-stories/us-08-01-40

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-08-01-40-01
      - arqix:requirements/req-08-01-40-02
      - arqix:requirements/req-08-01-40-03
      - arqix:requirements/req-08-01-40-04
      - arqix:requirements/req-08-01-40-05
      - arqix:requirements/req-08-01-40-06
      - arqix:requirements/req-08-01-40-07
      - arqix:requirements/req-08-01-40-08
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-08-01

properties:
  priority: high
  edge-case: false

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

## Anchor Derive and Account for Evidence

As a knowledge engineer, I want evidence anchored in the text, derived into the graph, and accounted for as data, so that a claim is position-exact, queryable, and countable without maintaining anything twice.

### Acceptance Criteria

- [ ] `<!-- arqix:claim supported-by=<source-iri> -->` above a block anchors it; `confidence=<value>` and `anchor="<locus>"` are optional attributes; a malformed marker (missing `supported-by`, unknown key) is CLM-001, a confidence outside the effective vocabulary (`[frontmatter].claim-confidence`, default high/inferred/estimated) is CLM-002.
- [ ] `arqix fmt` lifts the markers into the `derived-triples` frontmatter section (predicate `arqix:properties/supported-by`, targets deduplicated and sorted); the section is formatter-owned — absent without markers, hand edits do not survive, and `fmt --check` reports drift.
- [ ] Derived triples take part in the graph checks like declared ones: ONT-003 resolves the target, ONT-007 checks it against the property's declared range.
- [ ] A document without claim markers is byte-identical under `fmt`.
- [ ] `report claims` prints one deterministic CSV row per claim marker under the snapshot drift gate; the evidence-coverage unit reports claim, document, and source counts — numbers, never a gate.
- [ ] Provenance travels on three carriers of one vocabulary: computed from history on demand (`report claims --provenance`), inline on the marker (CLM-003 holds `review-status` to the declared vocabulary), and the claim record as the fullest form, referenced by any number of markers through `record=` (CLM-004 resolves it to the claim class).

### Notes

The mechanism of ADR-0018: the marker carries position and its attributes, the derived edge carries graph membership, and the formatter keeps them in step (ADR-0004, one mechanical mutator).
