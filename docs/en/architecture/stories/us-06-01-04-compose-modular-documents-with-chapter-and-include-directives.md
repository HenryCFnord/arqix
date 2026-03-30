---
id: us-06-01-04
title: Compose modular documents with chapter and include directives
slug: compose-modular-documents-with-chapter-and-include-directives
iri: arqix:user-stories/us-06-01-04

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-06
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-06-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-30
  updated: 2026-03-30
  lang: en
  translation-of:
  generated: false
---

## User-story

As a Aria Architect, I want to use chapter and include directives in Markdown, so that architecture documentation can be composed modularly and assembled into coherent larger documents.

### Acceptance Criteria

- [ ] Directives `<!-- arqix:chapter ... -->` and `<!-- arqix:include ... -->` are parsed.
- [ ] Include targets are restricted to allowed roots via configuration.
- [ ] Glob includes are expanded deterministically using configured sorting.

### Notes

Treat directive parsing as complete only when valid chapter and include markers survive formatting and invalid forms fail with a clear diagnostic. Add tests for root restriction enforcement and for deterministic expansion order when a glob matches multiple files. Keep the directive grammar small and document unsupported attributes rather than inferring behaviour implicitly. The main value for Aria is reliable modular composition of architecture narratives.
