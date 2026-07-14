---
id: REQ-02-01-12-05
title: Omit Included Fragment Frontmatter
slug: omit-included-fragment-frontmatter
iri: arqix:requirements/req-02-01-12-05

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-02-01-12
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: A page that includes a fragment carrying `---`-delimited frontmatter assembles to a body that contains the fragment's content but none of its frontmatter keys.

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

When arqix stitches an included fragment into a document, it SHALL omit the fragment's YAML frontmatter from the assembled output.

### Notes

A stitched document carries only the root document's frontmatter; a fragment's own frontmatter is metadata, not content, so inlining it renders a stray YAML block in the page (found on arqix.dev, the arc42 overview).
The root document's frontmatter is untouched — the toolchain still consumes it for the page title.
