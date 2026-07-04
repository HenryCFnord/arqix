---
id: unit-arc42-05
title: Building Block View
slug: building-block-view
iri: arqix:units/unit-arc42-05

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

## Building Block View

<!-- derived from ../model/workspace.dsl (view: Containers) -->
```mermaid
C4Container
    title arqix — Containers
    Person(agent, "Coding Agent", "")
    System_Boundary(arqixB, "arqix") {
        Container(cli, "arqix binary", "Rust", "All commands; effective-config resolution")
        Container(config, "arqix.toml", "TOML", "Kinds, templates, roots, policies, i18n")
        Container(corpus, "Documentation Corpus", "Markdown + YAML", "Documents, ontology, trace markers")
    }
    Rel(agent, cli, "invokes commands")
    Rel(cli, config, "resolves at startup")
    Rel(cli, corpus, "reads, creates, formats, assembles")
```

The binary decomposes into fifteen components: the CLI entrypoint as composition root, the document parser as shared reading layer, the verification orchestrator sequencing the quality gate, and twelve feature components cut along the requirement clusters:

| Component | Responsibility | Requirement cluster |
| --- | --- | --- |
| CLI Entrypoint & Dispatch | Argument parsing, subcommand routing, composition root (config → component → diagnostics/exit code) | REQ-00-00-00-02/03/06 |
| Document Parser | Single deterministic parse: lossless concrete syntax + semantic document model (frontmatter, sections/anchors, directives, markers) | REQ-02-01-09-*, REQ-05-01-10-*, REQ-01-01-03-03 |
| Verification Orchestrator | Sequences the configured verify sub-steps (format, lint, trace scan, coverage) via the stable command interface; fail-fast/aggregate modes, per-step JSON results; never implements a check itself ([ADR-0003](../../adr/ADR-0003-verification-orchestrator.md)) | REQ-04-01-05-* |
| Config Resolver | Effective configuration from defaults + overrides, validation | REQ-01-01-16-*, REQ-00-00-00-06 |
| Document Store & Catalog | Discovery, ID/slug policy, JSON catalog | REQ-00-00-00-04, REQ-05-01-08-* |
| Template Engine | Kind-based creation, placeholder substitution | REQ-00-00-00-05, REQ-01-01-05-* |
| Formatter | Canonical key order, directive normalisation | REQ-01-01-03-* |
| Linter | Includes, metadata contracts, IDs, i18n profile | REQ-01-01-04-*, REQ-01-01-10-*, REQ-00-00-00-10 |
| Assembler | Chapter/include directives, glob expansion, cycle detection, JSONL log | REQ-02-01-09-*, REQ-02-01-11-*, REQ-04-01-01-* |
| Trace Engine | Marker scan, trace graph, matrices, coverage | REQ-03-01-05-*, REQ-03-01-02-*, REQ-01-01-08-* |
| Report & Export | Audit exports, evidence bundles, stable schemas | REQ-04-01-12-*, REQ-03-01-04-* |
| Publish & Render Orchestrator | Pandoc/site orchestration per language | REQ-04-01-03-*, REQ-04-01-07-* |
| Policy Checker | Changed files vs declared change scope | REQ-01-01-07-*, REQ-00-00-00-07 |
| MCP Server | search/read/list over stdio, transport-separated | REQ-05-01-12-* |
| Diagnostics & Exit Codes | Machine-readable diagnostics, 0/1/2 contract | REQ-00-00-00-02/03, REQ-04-01-08-*, REQ-04-01-10-* |

Shared spine: the CLI Entrypoint invokes every feature component and is the only place that turns results into exit codes; every component reports through Diagnostics & Exit Codes, reads configuration through the Config Resolver, and reads documents through the Document Parser; the Verification Orchestrator sequences the quality-gate sub-steps through the same command interface the entrypoint uses (ADR-0003). These five are the components that make the cross-cutting contracts (chapter 8) enforceable in one place; lateral coupling between feature components is limited to Publish → Assembler and Report → Trace Engine — the orchestrators' edges are command-API orchestration, not implementation coupling.
