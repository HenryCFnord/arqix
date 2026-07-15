---
id: REQ-01-01-17-01
title: Exclude Configured Skip-Dirs from Discovery
slug: exclude-configured-skip-dirs-from-discovery
iri: arqix:requirements/req-01-01-17-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-17
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: low
  fit-criterion: A document inside a directory named by `skip-dirs` does not appear in the catalog, while documents outside it do.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-08
  updated: 2026-07-15
  lang: en
  translation-of:
  generated: false
---

## Requirement

Where `skip-dirs` is configured, arqix SHALL exclude the named directories from document discovery.

### Notes

Discovery means the document store walk under the configured roots (`doc list/read/search`, lint, fmt, assemble).
The trace corpus walk is out of scope here: its skip set was pinned to the now-retired Python oracle; consolidating it is post-retirement refactor work (arc42 chapter 8).
