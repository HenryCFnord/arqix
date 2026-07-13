---
id: ADR-0016
title: Render Architecture Views from the Model
slug: render-architecture-views
iri: arqix:adrs/adr-0016
rdf:
  type:
    - arqix:classes/adr
triples:
  - predicate: arqix:properties/guides-implementation-of
    object:
      - arqix:requirements/req-04-01-18-01
      - arqix:requirements/req-04-01-18-02
properties:
  decision-status: accepted
external-references: []
meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-13
  updated: 2026-07-13
  lang: en
  translation-of:
  generated: false
---

## Render Architecture Views from the Model

### Context

`workspace.dsl` is the single source of truth for the C4 model, and each embedded Mermaid view carries a `derived from … workspace.dsl` marker (ADR-0002).
That marker is a promise nothing enforced, and the C4 audit found drift by hand.
A first take on item 8 shipped an in-process drift checker that parsed the committed C4 Mermaid and validated it structurally against the model.
It worked, but a subsequent review found the class of problem it invites: a hand-rolled C4-Mermaid parser has to keep up with every dialect form (directional `Rel_Up`/`Rel_Down` macros, `Rel_Back` direction, `BiRel`), and each gap is a silent false accept or false reject.

The cleaner design removes the hand-authored diagram entirely.
arqix already orchestrates external renderers — Pandoc for PDF, a configured site command such as Zensical — so rendering the C4 views from the model as images and embedding those fits the established pattern: a diagram that is generated from the single source cannot drift from it, and there is no dialect to parse.
`structurizr-cli` and Kroki emit a flowchart Mermaid dialect, not the `C4Context` dialect the diagrams used, so the views become rendered images rather than regenerated Mermaid.
This reverses the "committing rendered Structurizr images (PNG/SVG)" alternative that ADR-0002 rejected as "not diffable, invisible in code review, and foreign to the Pages toolchain": a container render pipeline plus a regenerate-and-diff freshness gate answers the toolchain objection, and the drift the hand-authored Mermaid invited outweighs the reduced diffability of a generated SVG.

### Decision

**The C4 views are rendered from `workspace.dsl` to committed SVG images and embedded in the docs; a CI freshness gate regenerates and diffs them. The in-process derivation checker is withdrawn.**

- Rendering runs a containerised renderer — Kroki, which takes the Structurizr DSL and emits SVG — invoked through a `just` target so the local command is byte-for-byte the pipeline command (arqix's existing `just ci` == CI philosophy).
- The generated SVGs are committed under `docs/en/architecture/model/generated/` and embedded in the arc42 chapters, replacing the hand-authored C4 Mermaid blocks.
- The drift guarantee moves from "hand-diagram vs model" to "committed image vs fresh render": CI regenerates the images and fails on any difference, exactly the regenerate-and-diff freshness gate already used for the report snapshots (REQ-04-01-18-02).
  This gate is the decided design; it is not yet enforcing — the diagrams workflow is manual-dispatch and non-blocking while the render is confirmed (roadmap items 5 and 8), and REQ-04-01-18-02 is carried as a planned claim until it becomes a required check.
- Rendering needs a container runtime, so generation and the freshness gate live in CI and in `just` (with Docker), not in the offline `arqix verify` binary — the same posture as Pandoc and the site command, which the binary orchestrates but never bundles.
- Mermaid stays available for non-C4 diagrams; only the C4 architecture views become generated images.

### Alternatives Considered

- **In-process derivation checker (the withdrawn first take):** it solved drift for hand-authored C4 Mermaid, but keeping a C4-dialect parser correct is a standing liability (the review confirmed real false-accept/false-reject gaps), and generating the views removes the hand-authored diagram — a stronger guarantee with no parser to maintain.
- **`structurizr-cli` (its stated CI mechanism, ADR-0002):** viable — it is an orchestrated external tool like Pandoc, not a crate in the tree — but it exports diagram *definitions* (PlantUML, graph-Mermaid), so it needs a second render step to reach SVG; Kroki renders the DSL to SVG in one service, so it is the smaller pipeline.
- **A full DSL→C4-Mermaid generator:** rejected — neither structurizr-cli nor Kroki emits the `C4Context` dialect, so there is no faithful Mermaid to regenerate; images are the pragmatic render.
- **Generate on the fly in CI without committing:** rejected — the docs would not render on GitHub or the site without a container run; committing the images plus a freshness gate keeps the repository self-contained (the report-snapshot pattern).

### Consequences

- The closed dependency tree (ADR-0014) is unaffected: the renderer is an orchestrated external tool, not a linked crate.
- Rendering needs Docker in CI and locally; it is not part of the offline binary, and the freshness gate lives with the other pipeline steps.
- The in-process derivation checker, its module, and its `LNT-DRV` codes are removed; the `derived from` marker convention (ADR-0002) stands, now honoured by generation rather than a checker.
- SVG keeps the views scalable, small, and theme-friendly; a later slice may regenerate on model change and wire the images into the PDF and site builds.
