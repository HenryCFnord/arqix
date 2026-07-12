---
id: PER-09
title: Bernadette Builder
slug: bernadette-builder
iri: arqix:personas/per-09

rdf:
  type:
    - arqix:classes/persona

triples: []

properties:
  role: engineering user who changes the corpus
  description: Writes code, documentation, architecture decisions, and CI wiring in one flow. Values speed, deterministic local checks that match CI, and reproducible publishing.

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-12
  updated: 2026-07-12
  lang: en
  translation-of:
  generated: false
---

## Bernadette Builder

Bernadette is the engineering user who changes the corpus: code and documentation in the same flow, architecture decisions and glossary terms as they happen, CI gates and publishing as part of the change.

### Goals

- Create conforming documents with one command and keep code, tests, and docs linked via stable IDs.
- Record architecture decisions and terminology with traceable reasoning while they are fresh.
- Validate changes locally with exactly the checks CI runs, and publish reproducibly.
- Spend no time on formatting debates or manual stitching.

### Success Looks Like

- New docs are created from templates with correct structure on the first try.
- `arqix verify` locally is the same gate as CI — no surprises after the push.
- The site and the PDFs regenerate deterministically from the same corpus.

### Notes

This persona consolidates the developer, DevOps, and architect viewpoints in the persona merge of 2026-07-12 — three angles on one activity: building the system and its documentation together.
