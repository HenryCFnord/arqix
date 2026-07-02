---
id: US-01-01-06
title: Finalise Document Metadata Mechanically
slug: finalise-document-metadata-mechanically
iri: arqix:user-stories/us-01-01-06

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-01
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-08
      - arqix:requirements/req-01-01-06-01
      - arqix:requirements/req-01-01-06-02
      - arqix:requirements/req-01-01-06-03
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-01-01

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

## Finalise Document Metadata Mechanically

As a maintainer, I want to mechanically finalise metadata, so that `updated` is set consistently without rewriting content.

### Acceptance Criteria

- [ ] `arqix finalise` sets `updated` as an ISO-8601 date in `YYYY-MM-DD` format.
- [ ] `arqix finalise` performs only mechanical metadata changes and does not rewrite body text.
- [ ] Repeated runs only change metadata when the value actually changes.
- [ ] Files without supported frontmatter fail clearly, or the supported boundary is explicitly documented.

### Notes

This should be treated as a narrow metadata operation, not a content rewrite step.
Add tests showing that `updated` is written in `YYYY-MM-DD` format and that repeated runs only touch metadata when the value changes.
If files without frontmatter are unsupported, fail clearly and document that boundary.
This is a repository hygiene capability for safe, deterministic metadata maintenance.
