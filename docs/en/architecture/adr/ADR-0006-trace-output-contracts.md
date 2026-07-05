---
id: ADR-0006
title: Trace Output Contracts
slug: trace-output-contracts
iri: arqix:adrs/adr-0006

rdf:
  type:
    - arqix:classes/adr

triples:
  - predicate: arqix:properties/guides-design-of
    object:
  - predicate: arqix:properties/guides-implementation-of
    object:
      - arqix:requirements/req-03-01-05-04
      - arqix:requirements/req-01-01-08-03
      - arqix:requirements/req-00-00-00-03
      - arqix:requirements/req-04-01-12-02
  - predicate: arqix:properties/guides-verification-of
    object:

properties:
  decision-status: accepted

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-05
  updated: 2026-07-05
  lang: en
  translation-of:
  generated: false
---

## Trace Output Contracts

### Context

The trace family emits machine-readable output: `trace scan` a graph, `trace coverage` a report, `trace matrix` CSV, `report bundle` audit packages. The Python oracle (PR #13) sketched first JSON shapes; before they harden into the Rust contract, the question is what each output is *for* — the schema follows from the consumer, and three different consumers pull in three different directions: downstream projections, agent/CI automation, and audit stability.

### Decision

Trace output is layered; each layer has its own consumer and its own stability promise.

1. **Canonical core graph (`trace scan`).** The graph is the one lossless source: node and edge collections (REQ-03-01-05-04) from which coverage, matrices, and bundles are documented projections. Its contract is *complete and deterministic*, not *designed for external comfort* — external tools may read it, but convenience fields for particular consumers do not belong here.
2. **Diagnostics projection (`trace coverage`).** A coverage gap is a diagnostic in the tool-wide findings format — severity, stable code, message, source location (REQ-00-00-00-03) — plus a summary by requirement kind. Agents and CI parse one findings format across the whole tool. Coverage follows the red-skeleton lifecycle: a `verifies` marker on an active test **verifies** a requirement, a marker on an `#[ignore]`d test only **plans** its verification, no marker leaves it **uncovered** — the summary reports all three states per kind, because marker existence alone overstates progress. Stable codes:
   - `TRC-COV-001` (error): a functional requirement is uncovered — no `verifies` marker at all.
   - `TRC-COV-002` (warning): a functional requirement is only planned — every `verifies` marker sits on an ignored test.
   - `TRC-KIND-001` (warning): a requirement declares no kind and is treated as functional — the strictest default, made visible instead of silent.
   Exit code 1 is driven by error diagnostics only.
3. **Audit products (`trace matrix`, `report bundle`).** Stable headers and row models (REQ-03-01-02-03), stable export schemas (REQ-04-01-12-02), generation metadata (REQ-04-01-12-03). Schema changes here are contract breaches, not improvements.

Versioning: every JSON output carries a `schema_version` field starting now. Until the Rust port passes the conformance suite, schemas may change with a version bump; from Rust conformance on, layers 1 and 2 follow the general stability contract and layer 3 is hard-stable from the first Rust release.

### Alternatives Considered

- **Public exchange format from day one:** rejected — design effort for consumers that do not exist yet; the core-graph contract already lets external tools build on it.
- **Scan as contract-free debug output:** rejected — it breaks the "everything derivable from one source" property that keeps the projections honest.
- **Separate report format for coverage findings:** rejected — a second findings format next to the diagnostics contract doubles every consumer's parsing surface.
- **Freezing the PR #13 schemas immediately:** rejected — a schema that never had a real consumer should survive first contact with one before it becomes a promise.

### Consequences

- The Rust Trace Engine inherits the layers; the conformance suite asserts `schema_version` along with the payload.
- The coverage command's text and JSON outputs stay projections of the same findings (REQ-01-01-08-03) — one model, two serialisations.
- New trace-family outputs must declare their layer, and with it their stability promise, before implementation.
