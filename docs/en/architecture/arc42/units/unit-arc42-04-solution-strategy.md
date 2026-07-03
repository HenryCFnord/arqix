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
  updated: 2026-07-03
  lang: en
  translation-of:
  generated: false
---

## Solution Strategy

1. **One deterministic CLI.** Every capability is a subcommand of a single Rust binary; every command resolves against the effective configuration (REQ-00-00-00-06) and honours the shared exit-code and diagnostics contracts (REQ-00-00-00-02/03). Determinism (REQ-00-00-00-01) is an architectural property, not a per-feature promise: no wall-clock, no randomness, stable ordering everywhere.
2. **Documents are data.** Frontmatter plus a small ontology (`docs/ontology/`) make structure and relationships machine-readable; IDs and slugs derive deterministically from configured policy (REQ-00-00-00-04). The corpus in this repository is arqix's first real corpus.
3. **Contracts first, implementation second.** The 142 requirements, the requirements style guide, and the two Python checker scripts define testable contracts before Rust code exists; the scripts are the reference specification for the Rust port and stay as oracles.
4. **Traceability as a graph.** Trace markers in code and tests, frontmatter links, and the derived-from/has-requirement symmetry form one graph that scan, matrix, coverage, and evidence-bundle commands project into views (REQ-03-01-*).
5. **Delegated rendering.** Pandoc and site toolchains stay external processes behind an orchestrator; arqix owns inputs, artefact placement, and error transparency, never the renderer (REQ-04-01-03-*).
