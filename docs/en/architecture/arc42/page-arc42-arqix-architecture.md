---
id: page-arc42
title: arqix Architecture (arc42)
slug: arqix-architecture
iri: arqix:pages/page-arc42

rdf:
  type:
    - arqix:classes/document-page

triples:
  - predicate: arqix:properties/includes-unit
    object:
      - arqix:units/unit-arc42-01
      - arqix:units/unit-arc42-02
      - arqix:units/unit-arc42-03
      - arqix:units/unit-arc42-04
      - arqix:units/unit-arc42-05
      - arqix:units/unit-arc42-06
      - arqix:units/unit-arc42-07
      - arqix:units/unit-arc42-08
      - arqix:units/unit-arc42-09
      - arqix:units/unit-arc42-10
      - arqix:units/unit-arc42-11
      - arqix:units/unit-arc42-12

properties:
  output-kind: architecture-document

external-references:
  - type: specification
    label: "arc42 template"
    uri: https://arc42.org/

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-03
  updated: 2026-07-03
  lang: en
  translation-of:
  generated: false
---

## arqix Architecture (arc42)

This page assembles the arc42 architecture document from one unit per chapter (REQ-01-01-11-01).
Until `arqix assemble build` exists, the directives below document the intended assembly; afterwards they drive it.

<!-- arqix:chapter 1 -->
<!-- arqix:include units/unit-arc42-01-introduction-and-goals.md -->

<!-- arqix:chapter 2 -->
<!-- arqix:include units/unit-arc42-02-architecture-constraints.md -->

<!-- arqix:chapter 3 -->
<!-- arqix:include units/unit-arc42-03-context-and-scope.md -->

<!-- arqix:chapter 4 -->
<!-- arqix:include units/unit-arc42-04-solution-strategy.md -->

<!-- arqix:chapter 5 -->
<!-- arqix:include units/unit-arc42-05-building-block-view.md -->

<!-- arqix:chapter 6 -->
<!-- arqix:include units/unit-arc42-06-runtime-view.md -->

<!-- arqix:chapter 7 -->
<!-- arqix:include units/unit-arc42-07-deployment-view.md -->

<!-- arqix:chapter 8 -->
<!-- arqix:include units/unit-arc42-08-crosscutting-concepts.md -->

<!-- arqix:chapter 9 -->
<!-- arqix:include units/unit-arc42-09-architecture-decisions.md -->

<!-- arqix:chapter 10 -->
<!-- arqix:include units/unit-arc42-10-quality-requirements.md -->

<!-- arqix:chapter 11 -->
<!-- arqix:include units/unit-arc42-11-risks-and-technical-debt.md -->

<!-- arqix:chapter 12 -->
<!-- arqix:include units/unit-arc42-12-glossary.md -->
