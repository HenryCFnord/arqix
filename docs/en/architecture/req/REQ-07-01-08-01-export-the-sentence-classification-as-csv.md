---
id: REQ-07-01-08-01
title: Export the Sentence Classification as CSV
slug: export-the-sentence-classification-as-csv
iri: arqix:requirements/req-07-01-08-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-07-01-08
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: Running arqix report statements on the corpus prints a CSV header and one row per requirement document carrying id, kind, modality, EARS pattern, and subject, byte-identical across repeated runs on the same corpus state.

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

When `arqix report statements` runs, arqix SHALL print every requirement's normative-sentence classification as a CSV row carrying the requirement id, the kind, the modality, the EARS pattern, and the subject.

### Notes

A projection of the classification `lint requirements` already computes (REQ-01-01-11-06) — the export and the checker read the same functions, so they cannot disagree.
A requirement without exactly one normative sentence exports with empty classification fields; the checker owns the finding for that state.
Derived from US-07-01-08 (knowledge-repository slice K4, proposal P5).
