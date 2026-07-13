---
id: unit-arc42-01
title: Introduction and Goals
slug: introduction-and-goals
iri: arqix:units/unit-arc42-01

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
  updated: 2026-07-12
  lang: en
  translation-of:
  generated: false
---

## Introduction and Goals

arqix is a deterministic documentation-as-code toolchain: it creates, formats, lints, assembles, traces, and publishes modular Markdown documentation whose structure and relationships are machine-readable.
Documentation is treated as a dataset with a contract, not as prose with goodwill.

The system exists because "related" links between artefacts are not enough (see the blog post *why arqix had to exist*): requirements, stories, code, and tests need verifiable, tool-enforced traceability that survives automation.

### Top Quality Goals

| Priority | Quality goal | Anchored by |
| --- | --- | --- |
| 1 | Determinism — identical inputs and configuration produce byte-identical outputs | REQ-00-00-00-01 |
| 2 | Machine-readability — diagnostics, catalogs, logs, and reports are documented data contracts | REQ-00-00-00-03, REQ-00-00-00-02 |
| 3 | Safety and containment — no writes outside declared scope, no overwrites without approval, no content execution | REQ-00-00-00-07/08/13/14 |
| 4 | Traceability — every artefact participates in a verifiable US → REQ → code/test graph | REQ-01-01-08-*, REQ-03-01-05-* |
| 5 | Bilingual quality — translation completeness and drift are lintable | REQ-00-00-00-10 |

### Stakeholders

The four current personas under `../personas/` (six earlier ones retired in the 2026 merge) condense into four C4 roles: the Documentation Maintainer (PER-01, standards and governance), the Builder (PER-09, corpus alongside code, pipelines, and architecture), the Assessor (PER-10, evaluates and consumes the corpus), and the Coding Agent (PER-08, deterministic story-by-story automation; includes CI).
Coverage per persona is tracked in the requirements review table.
