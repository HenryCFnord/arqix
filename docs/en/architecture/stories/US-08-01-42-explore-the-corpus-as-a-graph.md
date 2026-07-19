---
id: US-08-01-42
title: Explore the Corpus as a Graph
slug: explore-the-corpus-as-a-graph
iri: arqix:user-stories/us-08-01-42

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-08-01-42-01
      - arqix:requirements/req-08-01-42-02
      - arqix:requirements/req-08-01-42-03
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-08-01

properties:
  priority: medium
  edge-case: false

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

## Explore the Corpus as a Graph

As a knowledge engineer, I want the corpus as a navigable node-link view with filters, so that its structure — clusters, hubs, orphans, neighbourhoods — is visible at a glance instead of assembled from tables.

### Acceptance Criteria

- [ ] `arqix report graph [--out <path>]` writes a single self-contained HTML page: the graph data and the vendored layout engine are embedded, no external resource is referenced.
- [ ] Document nodes carry id, title, type, and declared lifecycle status; every edge carries its kind; artefact nodes and their marker edges ride along flagged as the code layer.
- [ ] The default view shows documents coloured by type, with type and lifecycle filters, search, and neighbourhood highlight; the code layer is a switch, off by default.
- [ ] `arqix publish site` stages the explorer page with the published site.

### Notes

The page projects the trace core graph (ADR-0006) under its identity rules (ADR-0007); the form — self-contained export, vendored engine, on-demand generation — is fixed in ADR-0020.
