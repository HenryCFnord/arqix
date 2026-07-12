---
id: PER-10
title: Adrian Assessor
slug: adrian-assessor
iri: arqix:personas/per-10

rdf:
  type:
    - arqix:classes/persona

triples: []

properties:
  role: evaluating user who consumes the corpus as evidence and data
  description: Traces requirements to implementation and verification, evaluates reproducible evidence chains, and feeds machine-readable documentation into automated and agentic workflows.

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

## Adrian Assessor

Adrian is the evaluating user: the corpus is input, not output.
Whether the question is a quality gap, a compliance chain, or a retrieval pipeline for agentic tooling, Adrian needs deterministic reports, stable identifiers, and machine-readable interfaces.

### Goals

- Trace requirements, implementation, and tests, and surface gaps as machine-readable findings.
- Evaluate reproducible evidence chains from requirement to verification.
- Feed documentation into automated and agentic workflows via stable IDs and predictable interfaces.

### Success Looks Like

- Coverage and trace reports are deterministic and diffable across runs.
- An evidence question is a query against the graph, not an archaeology project.
- Exports and JSON interfaces are stable enough to build automation on.

### Notes

This persona consolidates the QA, auditor, and automation/RAG viewpoints in the persona merge of 2026-07-12 — three angles on one activity: evaluating and consuming the corpus.
