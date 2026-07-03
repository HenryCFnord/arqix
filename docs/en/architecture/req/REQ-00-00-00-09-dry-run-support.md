---
id: REQ-00-00-00-09
title: Dry-Run Support
slug: dry-run-support
iri: arqix:requirements/req-00-00-00-09

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-13
      - arqix:user-stories/us-02-01-07
      - arqix:user-stories/us-02-01-10
      - arqix:user-stories/us-06-01-06
      - arqix:user-stories/us-08-01-14
      - arqix:user-stories/us-08-01-23
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: `--dry-run` reports the planned ID, target path, and metadata without writing any file.

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-02
  updated: 2026-07-02
  lang: en
  translation-of:
  generated: false
---

## Requirement

Where a command creates or modifies files, the command SHALL support a dry-run mode that reports planned changes without writing.

### Notes

Curated from acceptance criteria demanding `--dry-run` behaviour for document and translation creation.

Contributing stories: 6 (see `derived-from`). Approved via `docs/en/plans/requirements-derivation-2026-07-02/CROSS-CONCERNS.md`.
