---
id: unit-arc42-08
title: Crosscutting Concepts
slug: crosscutting-concepts
iri: arqix:units/unit-arc42-08

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

## Crosscutting Concepts

Each concept below is a system-wide contract with its own cross-cutting requirement; components implement them via the shared spine (chapter 5).

- **Determinism** — byte-identical outputs for identical inputs and configuration; stable ordering, no ambient state (REQ-00-00-00-01).
- **Diagnostics contract** — every diagnostic is available as documented JSON with severity, stable code, message, and source location (REQ-00-00-00-03, REQ-04-01-10-*).
- **Exit codes** — `0` success, `1` findings or gate failure, `2` usage error; stable across releases (REQ-00-00-00-02, REQ-04-01-08-01).
- **Effective configuration** — one resolution path from `arqix.toml` through defaults and overrides; `config show` renders exactly what commands act on (REQ-00-00-00-06).
- **ID and slug policy** — deterministic derivation from configured policy, global duplicate detection, stable anchors (REQ-00-00-00-04).
- **i18n model** — source documents with linked translations; missing, unresolved, and outdated translations are lint findings and CI gates (REQ-00-00-00-10).
- **Guardrails** — declared change scope via policy files, no overwrites without approval, filesystem containment, no content execution (REQ-00-00-00-07/08/13/14).
- **Performance budgets** — sub-second search/read and a ten-second verification loop on a 1000-document corpus as calibratable budgets (REQ-00-00-00-11/12).
