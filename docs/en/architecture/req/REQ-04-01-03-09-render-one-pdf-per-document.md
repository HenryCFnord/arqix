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
Each document stages body-only pages.
A family's `index.md` landing page is dropped from the staged inputs when the family carries other content, and every staged page drops its own leading title heading and re-levels the remaining body so its first real section lands at H1, so the body opens at the document's actual chapters rather than nesting them under a repeated wrapper title.
The document title is passed to the renderer as explicit metadata, one title page and running header per PDF.
Fragments that another member includes are dropped from the document's staged inputs, exactly as the assembler inlines them.
