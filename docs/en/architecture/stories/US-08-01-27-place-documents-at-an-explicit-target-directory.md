---
id: US-08-01-27
title: Place Documents at an Explicit Target Directory
slug: place-documents-at-an-explicit-target-directory
iri: arqix:user-stories/us-08-01-27

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-08-01-27-01
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-08-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-15
  updated: 2026-07-15
  lang: en
  translation-of:
  generated: false
---

## Place Documents at an Explicit Target Directory

As a coding agent, I want to pass an explicit target directory to `doc new`, so that a term lands directly at a bounded-context path such as `contexts/geo/terms/` without moving it after creation.

### Acceptance Criteria

- [ ] `doc new <kind> --dir <path>` creates (and `--dry-run` plans) the document under the given repository-relative directory.
- [ ] The explicit directory wins over the kind's declared `dir` and the `<first-root>/<kind>/` default.
- [ ] An absolute path or a path containing `..` is a usage error (exit 2) and nothing is written — filesystem containment holds (REQ-00-00-00-13).

### Notes

Third slice of the authoring-ergonomics band from the knowledge-repository intake (`docs/en/plans/knowledge-repository-2026-07-15/`, gap G3, plan slice K2).
Builds on US-01-01-22 (declared kind directories); the explicit argument covers the per-document case a per-family contract cannot.
