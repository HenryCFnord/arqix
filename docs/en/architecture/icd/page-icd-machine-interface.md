---
id: page-icd
title: Machine Interface Control Document
slug: machine-interface
iri: arqix:pages/page-icd

rdf:
  type:
    - arqix:classes/document-page

triples:
  - predicate: arqix:properties/includes-unit
    object:
      - arqix:units/unit-icd-01
      - arqix:units/unit-icd-02
      - arqix:units/unit-icd-03
      - arqix:units/unit-icd-04
      - arqix:units/unit-icd-05
      - arqix:units/unit-icd-06

properties:
  output-kind: interface-control-document

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-06
  updated: 2026-07-06
  lang: en
  translation-of:
  generated: false
---

## Machine Interface Control Document

This page assembles the machine-facing interface contract of arqix — the surface a caller, CI gate, or agent programs against — from one unit per interface concern. arc42 explains how the tool is built; this ICD fixes what it exposes and consumes.
It cites the deciding ADRs (0003–0006 for the command taxonomy, orchestration, rewriter, and trace/diagnostics contracts) rather than re-deciding them, and composes generated fragments (the Command Reference, the Diagnostics Registry) where those exist.

Sections one to four are the output side (command surface, exit codes, diagnostics, wire schemas); section five is the input side agents author (markers, directives, triples); section six fixes the not-yet-shipped contracts ahead of their code.

<!-- arqix:include units/unit-icd-01-command-surface.md -->

<!-- arqix:include units/unit-icd-02-exit-codes.md -->

<!-- arqix:include units/unit-icd-03-diagnostics.md -->

<!-- arqix:include units/unit-icd-04-wire-schemas.md -->

<!-- arqix:include units/unit-icd-05-input-grammars.md -->

<!-- arqix:include units/unit-icd-06-forward-contracts.md -->
