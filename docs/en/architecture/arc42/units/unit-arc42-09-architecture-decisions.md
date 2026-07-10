---
id: unit-arc42-09
title: Architecture Decisions
slug: architecture-decisions
iri: arqix:units/unit-arc42-09

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
  updated: 2026-07-05
  lang: en
  translation-of:
  generated: false
---

## Architecture Decisions

Decisions are kept as ADRs under `../adr/`, maintained per the path model with a canonical governance language (REQ-01-01-11-02).

| ADR | Decision | Status |
| --- | --- | --- |
| [ADR-0001](../adr/ADR-0001-agent-instruction-document-layout.md) | AGENTS.md is the canonical agent instruction document; CLAUDE.md is a thin adapter; extension points carry no normative rules | accepted |
| [ADR-0002](../adr/ADR-0002-c4-model-source-and-rendering.md) | Structurizr DSL is the C4 model source; embedded views are derived Mermaid | accepted |
| [ADR-0003](../adr/ADR-0003-verification-orchestrator.md) | The verify loop is its own orchestrator component: sequencer over the stable command interface, never a checker | accepted |
| [ADR-0004](../adr/ADR-0004-finalise-and-the-mechanical-rewriter.md) | finalise lives in the Formatter & Finaliser — the only mutator of existing source documents, mechanical only, with an injected clock | accepted |
| [ADR-0005](../adr/ADR-0005-command-taxonomy.md) | Noun–verb command scheme; every analysis exists exactly once (coverage is `trace coverage`); `report` reserved for export products; `verify` as top-level exception | accepted |
| [ADR-0006](../adr/ADR-0006-trace-output-contracts.md) | Trace output is layered — canonical core graph, diagnostics projection, audit products — each with its own stability promise; `schema_version` in every JSON output | accepted |
| [ADR-0007](../adr/ADR-0007-graph-node-identity.md) | One node identity rule: declared document ID, else repository-relative path; `file` as attribute everywhere; edges reference node ids | accepted |
| [ADR-0008](../adr/ADR-0008-question-driven-report-units.md) | Human-facing reports are assemblies of question units — one unit answers one named question; raw model dumps are machine artefacts, never reports | accepted |
| [ADR-0009](../adr/ADR-0009-documentation-production-policy.md) | Documentation production policy: new documents are units discriminated by `section-kind`; code→doc links via the `documented-by` marker; one generator surface (report units); rustdoc as a gated layer; `schema_version` per interface | accepted |
| [ADR-0010](../adr/ADR-0010-lifecycle-vocabularies.md) | Lifecycle vocabularies: declared states carry intent, computed states carry findings; per-nature vocabularies (stories draft→specified→in-implementation→done, requirements draft/retired, prose draft→final via `finalise`), terminal `retired`; ADR decision-status stays orthogonal | accepted |

Decision records predating the ADR directory live in the planning packages under `docs/en/plans/` (ID scheme, canonical-owner model, atomicity, subject conventions).
