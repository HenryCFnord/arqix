---
id: us-05-01-06
title: Build machine-readable trace graphs
slug: build-machine-readable-trace-graphs
iri: arqix:user-stories/us-05-01-06

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-05
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-05-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-30
  updated: 2026-03-30
  lang: en
  translation-of:
  generated: false
---

## User-story

As a Alex AIOps, I want to scan traceability information, so that documentation, code, and test references can be exposed as a machine-readable graph for downstream tooling.

### Acceptance Criteria

- [ ] `arqix trace scan` detects markers in Rust comments, with markers configurable.
- [ ] `arqix trace scan` detects markers in Markdown HTML comments.
- [ ] `arqix trace scan` reads unit frontmatter links such as requirements, stories, ADRs, and refs.
- [ ] Trace scan outputs a graph of nodes and edges as JSON.
- [ ] Unresolved references remain visible in the report instead of being silently dropped.

### Notes

Acceptance should show that the scanner merges references from code comments, Markdown markers, and unit frontmatter into one consistent graph model. Add fixture-based tests that cover configurable marker syntax and verify node and edge stability in the JSON output. Keep unresolved references visible in the report instead of silently dropping them. The main value for Alex is structured graph data for automation and knowledge systems.
