---
id: REQ-04-01-07-04
title: Stage Assets at Document-Relative Paths
slug: stage-assets-at-document-relative-paths
iri: arqix:requirements/req-04-01-07-04

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-07
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: An asset configured under a doc root's language directory is staged at the same language-root-relative path as the pages, so a page's relative link to it resolves.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-13
  updated: 2026-07-13
  lang: en
  translation-of:
  generated: false
---

## Requirement

When staging a configured asset that lives under a doc root's language directory, arqix SHALL place it at the same language-root-relative path as the staged pages.

### Notes

Pages stage with their `<root>/<lang>/` prefix stripped, so a page linking `../../model/generated/view.svg` resolves only if the asset lands at the matching relative location rather than under a verbatim `docs/en/…` path (found on arqix.dev, the embedded C4 views did not display).
Assets outside any doc root — brand images under `assets/`, for example — keep their configured path.
