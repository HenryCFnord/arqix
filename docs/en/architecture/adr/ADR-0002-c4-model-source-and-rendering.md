---
id: ADR-0002
title: C4 Model Source and Rendering
slug: c4-model-source-and-rendering
iri: arqix:adrs/adr-0002

rdf:
  type:
    - arqix:classes/adr

triples:
  - predicate: arqix:properties/guides-design-of
    object:
  - predicate: arqix:properties/guides-implementation-of
    object:
      - arqix:requirements/req-01-01-11-04
  - predicate: arqix:properties/guides-verification-of
    object:

properties:
  decision-status: accepted

external-references:
  - type: specification
    label: "Structurizr DSL"
    uri: https://docs.structurizr.com/dsl
  - type: specification
    label: "C4 model"
    uri: https://c4model.com/

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-03
  updated: 2026-07-03
  lang: en
  translation-of:
  generated: false
---

## C4 Model Source and Rendering

### Context

REQ-01-01-11-04 requires architecture views to use Mermaid diagrams in a C4-oriented style. That requirement is owed to the publishing target: GitHub Pages renders Mermaid natively, so embedded views must be Mermaid. Mermaid, however, is a drawing notation, not a model — hand-maintained Mermaid diagrams drift apart because each diagram repeats the same elements and relationships without a shared source.

### Decision

- `docs/en/architecture/model/workspace.dsl` (Structurizr DSL) is the single source of truth for the C4 model: people, systems, containers, components, relationships, and views are defined there and only there.
- Views embedded in documentation (arc42 chapters, handbook) are Mermaid diagrams *derived from* the workspace. Every embedded diagram carries a `derived from docs/en/architecture/model/workspace.dsl` marker comment.
- Until a `structurizr-cli export -format mermaid` step runs in CI, the Mermaid views are derived by hand. Hand-derived views change only together with the workspace; a review that touches one must touch both.
- Diagram scope follows C4 levels: system context and container views for chapter 3/5 overviews, component views only where a chapter needs them.

### Alternatives Considered

- Mermaid as the source: rejected — no semantic model, duplicated elements across diagrams, no way to validate consistency between views.
- Committing rendered Structurizr images (PNG/SVG): rejected — not diffable, invisible in code review, and foreign to the Pages toolchain.
- PlantUML/C4-PlantUML: rejected — requires a rendering toolchain Pages does not provide natively.

### Consequences

- Model changes happen in one file; diagrams follow. Divergence between DSL and embedded Mermaid is a review-visible defect, not a silent drift.
- Adding the `structurizr-cli` export to CI later removes the manual derivation step without changing any convention.
- The workspace can later drive additional outputs (e.g. Structurizr site) without touching the documentation.
