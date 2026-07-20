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
  updated: 2026-07-17
  lang: en
  translation-of:
  generated: false
---

## Architecture Decisions

Decisions are kept as ADRs under `../adr/`, maintained per the path model with a canonical governance language (REQ-01-01-11-02).

| ADR | Decision | Status |
| --- | --- | --- |
| [ADR-0001](../../adr/ADR-0001-agent-instruction-document-layout.md) | AGENTS.md is the canonical agent instruction document; CLAUDE.md is a thin adapter; extension points carry no normative rules | accepted |
| [ADR-0002](../../adr/ADR-0002-c4-model-source-and-rendering.md) | Structurizr DSL is the C4 model source; embedded views are derived Mermaid | accepted |
| [ADR-0003](../../adr/ADR-0003-verification-orchestrator.md) | The verify loop is its own orchestrator component: sequencer over the stable command interface, never a checker | accepted |
| [ADR-0004](../../adr/ADR-0004-finalise-and-the-mechanical-rewriter.md) | finalise lives in the Formatter & Finaliser — the only mutator of existing source documents, mechanical only, with an injected clock | accepted |
| [ADR-0005](../../adr/ADR-0005-command-taxonomy.md) | Noun–verb command scheme; every analysis exists exactly once (coverage is `trace coverage`); `report` reserved for export products; `verify` as top-level exception | accepted |
| [ADR-0006](../../adr/ADR-0006-trace-output-contracts.md) | Trace output is layered — canonical core graph, diagnostics projection, audit products — each with its own stability promise; `schema_version` in every JSON output | accepted |
| [ADR-0007](../../adr/ADR-0007-graph-node-identity.md) | One node identity rule: declared document ID, else repository-relative path; `file` as attribute everywhere; edges reference node ids | accepted |
| [ADR-0008](../../adr/ADR-0008-question-driven-report-units.md) | Human-facing reports are assemblies of question units — one unit answers one named question; raw model dumps are machine artefacts, never reports | accepted |
| [ADR-0009](../../adr/ADR-0009-documentation-production-policy.md) | Documentation production policy: new documents are units discriminated by `section-kind`; code→doc links via the `documented-by` marker; one generator surface (report units); rustdoc as a gated layer; `schema_version` per interface | accepted |
| [ADR-0010](../../adr/ADR-0010-lifecycle-vocabularies.md) | Lifecycle vocabularies: declared states carry intent, computed states carry findings; per-nature vocabularies (stories draft→specified→in-implementation→done, requirements active/retired, prose draft→final via `finalise`), terminal `retired`; ADR decision-status stays orthogonal | accepted |
| [ADR-0011](../../adr/ADR-0011-configuration-boundary.md) | Configuration boundary: a value becomes configuration on double bookkeeping or legitimate per-repository variance, stays convention as tool identity, stability contract, or check substance; one source feeds engine and reference tools, defaults preserve the present | accepted |
| [ADR-0012](../../adr/ADR-0012-id-policy-model.md) | ID policy model: declared triples are the source of truth for relations, the ID is an opaque label; per-family patterns govern shape, uniqueness, and generation, named groups activate optional consistency checks; defaults reproduce the current shapes | accepted |
| [ADR-0013](../../adr/ADR-0013-stitching-model.md) | Stitching model: include directives declare heading levels (absolute or relative), the assembler re-levels whole fragments; heading ownership is corpus policy; site splits cut the assembled outline, never fragment boundaries; PDF is always single-page; `arqix:chapter` retired | accepted |
| [ADR-0014](../../adr/ADR-0014-mcp-transport.md) | MCP transport: `mcp serve` implements the required protocol subset directly over stdio (blocking JSON-RPC loop, `serde_json`), no SDK dependency; scripted-session tests own spec conformance; revisit trigger is a requirement beyond the subset (HTTP/SSE, resources, prompts) | accepted |
| [ADR-0015](../../adr/ADR-0015-marker-freshness.md) | Marker freshness by git arithmetic: a marker is possibly-stale when its target requirement was committed after the marker's own file; exposed as `trace freshness`, informational in `verify` | accepted |
| [ADR-0016](../../adr/ADR-0016-render-architecture-views.md) | Architecture views are generated from `workspace.dsl` via a containerised renderer (Kroki) into committed SVGs with a regenerate-and-diff freshness gate; the in-process C4-Mermaid checker is withdrawn | accepted |
| [ADR-0017](../../adr/ADR-0017-process-profiles-and-the-layered-ontology.md) | Process profiles and the layered ontology: rules stay code, their activation and binding become configuration, vocabulary becomes layered corpus data (reserved core, module vocabularies, project ontology) validated by an ONT meta-rule family; guarded lifecycle stays core, domain status is declared vocabulary | accepted |
| [ADR-0018](../../adr/ADR-0018-evidence-anchors-and-derived-triples.md) | Evidence anchors and derived triples: a claim is a body marker above the supported block plus a `derived-triples` edge that `fmt` generates from it; position-bound attributes stay on the marker, the edge joins the validated graph; confidence is a declared vocabulary; only claim markers are lifted | accepted |
| [ADR-0019](../../adr/ADR-0019-provenance-layers.md) | Provenance layers: one vocabulary, three carriers of increasing depth — computed from history (informational, never gated), inline on the marker (validated dictionary), and the claim record as the fullest form (analysis block, declared review verdict); records are derivable from the lower carriers, several markers may share one record, `supported-by` stays the only edge | accepted |
| [ADR-0023](../../adr/ADR-0023-the-query-surface.md) | The query surface: a query is a small structured filter set — kind, lifecycle, edge patterns with prefix targets, conjunctive, no query language; `doc query` and the MCP `query` tool answer identically from one function; edge matching reads raw frontmatter so external targets are first-class; disjunction, negation, and traversal wait for a driving question | accepted |

Decision records predating the ADR directory live in the planning packages under `docs/en/plans/` (ID scheme, canonical-owner model, atomicity, subject conventions).
