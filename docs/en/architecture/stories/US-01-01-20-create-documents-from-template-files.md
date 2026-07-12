---
id: US-01-01-20
title: Create Documents from Template Files
slug: create-documents-from-template-files
iri: arqix:user-stories/us-01-01-20

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-01
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-05
      - arqix:requirements/req-01-01-20-01
      - arqix:requirements/req-01-01-20-02
      - arqix:requirements/req-01-01-20-03
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-01-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: done
  owner: hcf
  created: 2026-07-11
  updated: 2026-07-11
  lang: en
  translation-of:
  generated: false
---

## Create Documents from Template Files

As a maintainer, I want `doc new` and `doc init` to instantiate template files from a configured directory, so that I can shape what a new document looks like by editing a file instead of forking string literals in the engine.

### Acceptance Criteria

- [x] `doc new` and `unit new` instantiate the template file configured for the kind; the string literals in the engine are removed.
- [x] `doc init` scaffolds the default template files into the configured template directory.
- [x] A missing configured template file fails with a diagnostic naming the expected path.
- [x] The placeholders `{id}`, `{title}`, and `{slug}` substitute exactly as before, and an unconfigured repository produces byte-identical documents.

### Notes

This story carries the audit rows C5 and C12 (templates as string literals, package scaffold directories).
Template files must satisfy the configured frontmatter contract of US-01-01-19 (key order, required meta) — the template is the first document a contract sees.
Templates are excluded from document discovery (the `.tpl.md` convention the store already skips).
