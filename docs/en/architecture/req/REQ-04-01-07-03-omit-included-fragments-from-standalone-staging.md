---
id: REQ-04-01-07-03
title: Omit Included Fragments from Standalone Staging
slug: omit-included-fragments-from-standalone-staging
iri: arqix:requirements/req-04-01-07-03

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
  fit-criterion: When a page includes a fragment, publishing the site stages the page but not the fragment as its own file.

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

When publishing the site, arqix SHALL NOT stage a document that another published page includes; the fragment reaches the site through its parent page.

### Notes

Without this, the site carries both the stitched page and each raw fragment — the arc42 overview appeared once as the assembled page and again as its individual units (found on arqix.dev).
The stitched page is the unit of publication; its fragments are content within it.
