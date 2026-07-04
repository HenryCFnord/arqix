---
id: unit-arc42-02
title: Architecture Constraints
slug: architecture-constraints
iri: arqix:units/unit-arc42-02

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

## Architecture Constraints

| Constraint | Consequence | Source |
| --- | --- | --- |
| Rust, single binary | All behaviour ships in one deterministic CLI; the Python checker scripts are reference implementations to be ported | repository layout, `scripts/` |
| GPL-3.0-or-later | Licensing of all shipped code and templates | `LICENSE` |
| Docs-as-code | All artefacts are Markdown + YAML frontmatter in git; no database, no service | `docs/en/index.md` |
| Agent-neutral process | Normative process rules live in AGENTS.md only; agent-specific files are thin adapters | ADR-0001 |
| Filesystem containment | The CLI never accesses files outside the repository root and configured allowed roots | REQ-00-00-00-13 |
| No content execution | Document content never triggers process execution; external tools run only when explicitly configured | REQ-00-00-00-14 |
| Versioned contracts | SemVer for the product plus separately versioned `config_version` and `schema_version` | REQ-01-01-15-02 |
| GitHub Pages rendering | Embedded diagrams must be Mermaid; the C4 model source is Structurizr DSL | ADR-0002, REQ-01-01-11-04 |
