---
id: US-07-01-08
title: Export the Normative-Statement Classification
slug: export-the-normative-statement-classification
iri: arqix:user-stories/us-07-01-08

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-10
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-07-01-08-01
      - arqix:requirements/req-07-01-08-02
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-07-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-16
  updated: 2026-07-16
  lang: en
  translation-of:
  generated: false
---

## Export the Normative-Statement Classification

As an assessor, I want every requirement's normative-sentence classification exported as data, so that the requirement language itself — modality, EARS pattern, subject — becomes reviewable and queryable evidence instead of living only inside the checker.

### Acceptance Criteria

- [ ] `arqix report statements` prints one CSV row per requirement — id, kind, modality, EARS pattern, subject — derived from the same classification the requirements checker enforces.
- [ ] Identical corpus state produces a byte-identical export.
- [ ] The committed export lives with the report snapshots, and `report snapshot --check` reports it when it is stale or missing.

### Notes

Slice K4 of the knowledge-repository program (`docs/en/plans/knowledge-repository-2026-07-15/`, proposal P5): a projection of the classification the checker already computes — no new parsing.
The export is the data face of the two normative bases captured as source records (RFC 2119 modality, EARS patterns; SRC-0002, SRC-0003).
The band-3 query surface consumes this projection; landing it pre-release keeps band 3 additive.
