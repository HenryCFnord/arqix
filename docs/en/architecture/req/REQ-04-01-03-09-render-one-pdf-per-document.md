---
id: REQ-04-01-03-09
title: Render One PDF per Document
slug: render-one-pdf-per-document
iri: arqix:requirements/req-04-01-03-09

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-03
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A PDF render emits one artefact per configured or auto-discovered top-level document, not one per package.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-14
  updated: 2026-07-14
  lang: en
  translation-of:
  generated: false
---

## Requirement

When rendering a PDF, arqix SHALL produce one artefact per top-level document.

### Notes

A top-level document is a content family (a directory with an `index.md`, whose subtree is collected) or a standalone top-level page.
Boundaries are declared by `[policies.render] documents` (`{ name, path, title? }` entries), or auto-discovered when that list is absent (ADR-0013).
Each document stages body-only pages whose first heading is re-levelled to H1, so its title lands as a real `#` heading and its sections number cleanly from `1`; the document title is passed to the renderer as explicit metadata.
Fragments that another member includes are dropped from the document's staged inputs, exactly as the assembler inlines them.
