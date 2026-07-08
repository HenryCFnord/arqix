---
id: US-07-01-04
title: Review Trace Graphs as Audit Evidence
slug: review-trace-graphs-as-audit-evidence
iri: arqix:user-stories/us-07-01-04

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-07
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-03
      - arqix:requirements/req-00-00-00-06
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-07-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-30
  updated: 2026-07-02
  lang: en
  translation-of:
  generated: false
---

## Review Trace Graphs as Audit Evidence

As an auditor, I want to scan traceability information, so that evidence chains between requirements, implementation, and verification can be reviewed reproducibly.

### Acceptance Criteria

- [ ] `arqix trace scan` detects markers in Rust comments, with markers configurable.
- [ ] `arqix trace scan` detects markers in Markdown HTML comments.
- [ ] `arqix trace scan` reads unit frontmatter links such as requirements, stories, ADRs, and refs.
- [ ] Trace scan outputs a graph of nodes and edges as JSON.
- [ ] Unresolved references remain visible in the report instead of being silently dropped.

### Notes

Acceptance should show that the scanner merges references from code comments, Markdown markers, and unit frontmatter into one consistent graph model.
Add fixture-based tests that cover configurable marker syntax and verify node and edge stability in the JSON output.
Keep unresolved references visible in the report instead of silently dropping them.
The main value for Avery is drill-down capable audit evidence.
