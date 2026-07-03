---
id: US-05-01-04
title: Parse Document Structure Deterministically for Automation
slug: parse-document-structure-deterministically-for-automation
iri: arqix:user-stories/us-05-01-04

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-05
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-01
      - arqix:requirements/req-00-00-00-06
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-05-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-30
  updated: 2026-07-02
  lang: en
  translation-of:
  generated: false
---


## Parse Document Structure Deterministically for Automation

As an AIOps engineer, I want to use chapter and include directives in Markdown, so that document structure can be interpreted reliably by tooling and downstream automation.

### Acceptance Criteria

- [ ] Directives `<!-- arqix:chapter ... -->` and `<!-- arqix:include ... -->` are parsed.
- [ ] Include targets are restricted to allowed roots via configuration.
- [ ] Glob includes are expanded deterministically using configured sorting.

### Notes

Treat directive parsing as complete only when valid chapter and include markers survive formatting and invalid forms fail with a clear diagnostic. Add tests for root restriction enforcement and for deterministic expansion order when a glob matches multiple files. Keep the directive grammar small and document unsupported attributes rather than inferring behaviour implicitly. The main value for Alex is deterministic structural parsing for retrieval and automation.
