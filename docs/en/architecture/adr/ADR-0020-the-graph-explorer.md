---
id: ADR-0020
title: The Graph Explorer
slug: the-graph-explorer
iri: arqix:adrs/adr-0020

rdf:
  type:
    - arqix:classes/adr

triples: []

properties:
  decision-status: accepted

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-19
  updated: 2026-07-19
  lang: en
  translation-of:
  generated: false
---

## The Graph Explorer

### Context

The corpus is a graph, and the tooling already serialises it: the trace core graph carries every document node and every resolved edge under a stable schema (ADR-0006), with one identity rule for its nodes (ADR-0007).
The existing projections answer counting questions — matrices, coverage numbers, catalogs.
Navigation questions stay unanswered: what surrounds this story, which clusters exist, where are the hubs and the orphans.
A node-link view with filters answers them at a glance; a table answers them not at all.

Two constraints shape the form.
Published artefacts must stand alone — a page that loads its engine from a third-party network breaks offline reading and ties publication to someone else's availability.
And the dependency bar (ADR-0014) demands the judgement per ingredient: vendor what removes real algorithmic work, write what is plain project code.

### Decision

**The corpus graph ships as one self-contained interactive HTML page, generated on demand.**

- `arqix report graph [--out <path>]` writes the page — an export product on the `report` noun (ADR-0005) — and `publish site` stages it with the published site.
- The page embeds the trace core graph, enriched with each document's title and declared lifecycle status; enrichment decorates existing nodes and never invents new ones (ADR-0007).
- The layout engine is vendored: the four d3-force modules (d3-dispatch, d3-quadtree, d3-timer, d3-force), pinned minified builds concatenated into one file, recorded as a source record whose digest the SRC contract verifies.
  The interaction shell — canvas rendering, pan and zoom, filters, search, neighbourhood highlight — is project code in the page.
- The default view shows document nodes, coloured by type and filterable by type and lifecycle status; artefact nodes and their marker edges are a switchable code layer, off by default.
- The page is never committed: its embedded data changes with every corpus edit, so a committed copy either churns every change under a freshness gate or rots without one.

### Alternatives Considered

- **A committed snapshot page:** rejected — graph data ages with every commit; the snapshot treadmill buys no reader value here.
- **A hand-rolled layout:** rejected — a usable force layout means Barnes–Hut approximation, velocity integration, and collision handling; d3-force delivers exactly that in seventeen kilobytes, which is the case where vendoring beats rewriting.
- **Loading the library from a CDN:** rejected — the page stops being self-contained, and offline reading dies with the network.
- **Vendoring the full d3 bundle:** rejected — selection, scales, and transitions serve nothing here; the shell is a few hundred lines over canvas.
- **A server-rendered or baked SVG view:** rejected — a static picture answers none of the navigation questions; the filters and the highlight are the point.

### Consequences

- The Report & Export component owns `report graph`; the publisher stages the page for the default language.
- Third-party code enters the repository once, pinned and digest-recorded; updating it is a deliberate re-vendoring with a new digest, never a silent drift.
- The explorer reads the same core graph every other projection reads — a new data layer (claims, freshness) is a new toggle over the same embedded model, not a new pipeline.
- The page works from the filesystem, from the published site, and offline alike.
