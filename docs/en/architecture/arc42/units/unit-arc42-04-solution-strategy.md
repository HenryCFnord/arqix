---
id: unit-arc42-04
title: Solution Strategy
slug: solution-strategy
iri: arqix:units/unit-arc42-04

rdf:
  type:
    - arqix:classes/unit

triples: []

properties:
  section-kind: arc42-chapter

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-03
  updated: 2026-07-04
  lang: en
  translation-of:
  generated: false
---

## Solution Strategy

1. **One deterministic CLI.**
   Every capability is a subcommand of a single Rust binary; every command resolves against the effective configuration (REQ-00-00-00-06) and honours the shared exit-code and diagnostics contracts (REQ-00-00-00-02/03).
   Determinism (REQ-00-00-00-01) is an architectural property, not a per-feature promise: no wall-clock, no randomness, stable ordering everywhere.
2. **Documents are data.**
   Frontmatter plus a small ontology (`docs/ontology/`) make structure and relationships machine-readable; IDs and slugs derive deterministically from configured policy (REQ-00-00-00-04).
   The corpus in this repository is arqix's first real corpus.
3. **Contracts first, implementation second.**
   The 144 requirements, the requirements style guide, and the Python checker scripts (requirements, frontmatter, trace markers, and the trace oracle) define testable contracts before Rust code exists; the scripts are the reference specification for the Rust port and stay as oracles until the cross-check retires them (chapter 8).
4. **Traceability as a graph.**
   Trace markers in code and tests, frontmatter links, and the derived-from/has-requirement symmetry form one graph that scan, matrix, coverage, and evidence-bundle commands project into views (REQ-03-01-*).
5. **One parser, two layers.**
   Documents are parsed exactly once by a shared Document Parser: a lossless concrete-syntax layer that the formatter needs for meaning-preserving rewrites (REQ-01-01-03-03), and a semantic document model (frontmatter, sections and anchors, directives, trace markers) that store, linter, assembler, and trace engine consume (REQ-02-01-09-*, REQ-05-01-10-*).
   One grammar, one implementation — parser drift across components would silently break determinism.
6. **Delegated rendering.**
   Pandoc and site toolchains stay external processes behind an orchestrator; arqix owns inputs, artefact placement, and error transparency, never the renderer (REQ-04-01-03-*).
