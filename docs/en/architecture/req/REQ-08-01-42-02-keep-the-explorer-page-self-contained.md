---
id: REQ-08-01-42-02
title: Keep the Explorer Page Self-Contained
slug: keep-the-explorer-page-self-contained
iri: arqix:requirements/req-08-01-42-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-42
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: The generated page embeds the graph data and the vendored layout engine inline and references no external script, stylesheet, or other remote resource.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-19
  updated: 2026-07-19
  lang: en
  translation-of:
  generated: false
---

## Requirement

When `arqix report graph` writes the explorer page, arqix SHALL embed the graph data and the vendored layout engine inline so that the page references no external resource.

### Notes

Self-containment is the ADR-0020 form: the page works from the filesystem, from the published site, and offline alike; the vendored engine is pinned and digest-recorded through its source record.
Derived from US-08-01-42.
