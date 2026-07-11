---
id: US-02-01-09
title: Use Include Directives During Implementation
slug: use-include-directives-during-implementation
iri: arqix:user-stories/us-02-01-09

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-02
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-01
      - arqix:requirements/req-00-00-00-06
      - arqix:requirements/req-02-01-09-01
      - arqix:requirements/req-02-01-09-02
      - arqix:requirements/req-02-01-09-03
      - arqix:requirements/req-00-00-00-13
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
  updated: 2026-07-11
  lang: en
  translation-of:
  generated: false
---

## Use Include Directives During Implementation

As a developer, I want to use include directives in Markdown, so that I can create documentation incrementally during development and assemble it reliably into larger documents.

### Acceptance Criteria

- [ ] `<!-- arqix:include ... -->` directives are parsed, including their optional heading-level argument.
- [ ] Include targets are restricted to allowed roots via configuration.
- [ ] Glob includes are expanded deterministically using configured sorting.

### Notes

Treat directive parsing as complete only when valid include markers survive formatting and invalid forms fail with a clear diagnostic.
Add tests for root restriction enforcement and for deterministic expansion order when a glob matches multiple files.
Keep the directive grammar small and document unsupported attributes rather than inferring behaviour implicitly.
The main value for a developer is low-friction modular documentation in the implementation flow.
The `arqix:chapter` directive is retired from the grammar (ADR-0013): it stayed decorative since the assembler shipped, and the heading-level argument supplies the semantics it never had; chapter identity remains frontmatter ids.
