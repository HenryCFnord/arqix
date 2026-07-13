---
id: ADR-0016
title: In-Process Derivation Check
slug: in-process-derivation-check
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

## In-Process Derivation Check

### Context

`workspace.dsl` is the single source of truth for the C4 model, and each embedded Mermaid view carries a `<!-- derived from … workspace.dsl (view: X) -->` marker (ADR-0002).
That marker is a promise no gate enforces: the C4 audit found three phantom or misdirected edges in the model by hand, and nothing stops a hand-edited diagram from drifting away from the model in the same way.
ADR-0002 anticipated closing this with a `structurizr-cli export -format mermaid` step in CI.

Two facts make that mechanism the wrong first move.
`structurizr-cli` is a Java tool, absent from this environment and unable to run offline without provisioning, and adopting it cuts against the deliberately closed dependency tree ADR-0014 spent a whole decision defending; its Mermaid export is also not the `C4Context`/`C4Container` dialect the committed diagrams use.
And the committed diagrams are hand-abbreviated: element ids are shortened (`render` for `renderToolchain`), descriptions are truncated, relationship labels are reworded, and the container view shows Structurizr's implied `agent -> cli` edge that is not literally in the model — so no generator reproduces them byte-for-byte without first rewriting every diagram.

### Decision

**A lint check parses `workspace.dsl` and each derived Mermaid view in-process and verifies structural derivability; it does not generate or regenerate the diagrams.**

- The check is in-process Rust over the small DSL subset the model uses (`person`, `softwareSystem` with the `External` tag, `container`, `component`, relationships, and the three view forms) plus the C4 Mermaid dialect — no new runtime dependency, offline, deterministic like every other check.
- Elements are matched to the model by **display name and kind**, not by id: the names survive the diagrams' abbreviation while the ids do not, so the current hand-authored views validate without being rewritten (REQ-04-01-18-01).
  The `External` tag maps to `System_Ext`, and a container declared inside a `System_Boundary` maps to that system's containers.
- Relationships are justified by a model edge between the resolved endpoints, **direct or implied**: a container view inherits Structurizr's pushed-down edges, so a system-level edge justifies an edge to a container of that system (`agent -> arqix` justifies `agent -> cli`, REQ-04-01-18-02).
- Free-text labels and descriptions are not compared — only topology (element identity, kind, relationship endpoints).
- The check is gating (findings are errors) and rides the existing `lint` sub-step, so it joins `verify` with no new command surface (ADR-0003, ADR-0005).

### Alternatives Considered

- **`structurizr-cli export` then diff (ADR-0002's stated mechanism):** deferred — an external Java toolchain that is absent, offline-hostile, and against the closed dependency tree (ADR-0014); its Mermaid export is not the committed C4 dialect, so it would need a normaliser before it could compare at all.
- **A full in-process DSL-to-Mermaid generator with exact-match gating:** deferred — it is the stronger long-term contract (the diagrams become literally generated), but it must first rewrite every committed diagram to the generator's canonical form, discarding the deliberate hand-abbreviations; a checker earns the gate now without touching the corpus, and the generator can build on the same DSL parser later.
- **Exact-label matching:** rejected for the first slice — the diagrams are hand-abbreviated on purpose, so label comparison would flag the correct diagrams; topology is the derivable invariant.

### Consequences

- The dependency tree stays closed; the check runs offline and deterministically inside `lint`.
- Structural drift — an element or relationship the model cannot justify, the class the C4 audit caught by hand — now fails the gate; free-text drift (a stale description) is out of scope until label matching or the generator lands.
- The DSL parser this introduces is the foundation the future generator and exact-match gate build on; adopting either later revisits this decision without changing the derived-from convention (ADR-0002).
- Component views are unsupported until a document embeds one; only the two derived views (SystemContext, Containers) exist today.
