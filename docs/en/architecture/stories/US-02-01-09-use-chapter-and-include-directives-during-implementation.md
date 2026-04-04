---

id: US-02-01-09
title: Use chapter and include directives during implementation
slug: use-chapter-and-include-directives-during-implementation
iri: arqix:user-stories/us-02-01-09

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-02
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-02-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-30
  updated: 2026-04-04
  lang: en
  translation-of:
  generated: false
---

## User-story

As a Dev Dan, I want to use chapter and include directives in Markdown, so that I can create documentation incrementally during development and assemble it reliably into larger documents.

### Acceptance Criteria

- [ ] Directives `<!-- arqix:chapter ... -->` and `<!-- arqix:include ... -->` are parsed.
- [ ] Include targets are restricted to allowed roots via configuration.
- [ ] Glob includes are expanded deterministically using configured sorting.

### Notes

Treat directive parsing as complete only when valid chapter and include markers survive formatting and invalid forms fail with a clear diagnostic. Add tests for root restriction enforcement and for deterministic expansion order when a glob matches multiple files. Keep the directive grammar small and document unsupported attributes rather than inferring behaviour implicitly. The main value for Dan is low-friction modular documentation in the implementation flow.
