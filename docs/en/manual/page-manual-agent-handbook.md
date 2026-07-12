---
id: page-manual
title: Manual
slug: manual
iri: arqix:pages/page-manual

rdf:
  type:
    - arqix:classes/document-page

triples:
  - predicate: arqix:properties/includes-unit
    object:
      - arqix:units/unit-manual-01

properties:
  output-kind: manual

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

## Manual

This page assembles the arqix manual from one unit per chapter (ADR-0009's document-set plan: the User Manual follows the ICD).
The first chapter onboards anyone — human or agent — into a repository governed by arqix; further chapters join as the command surface grows.

<!-- arqix:include units/unit-manual-01-working-in-an-arqix-governed-repository.md -->
