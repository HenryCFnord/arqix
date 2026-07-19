---
id: REQ-08-01-42-03
title: Stage the Explorer With the Published Site
slug: stage-the-explorer-with-the-published-site
iri: arqix:requirements/req-08-01-42-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-42
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: After publish site for the default language, the staging tree carries the generated explorer page alongside the staged pages.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-19
  updated: 2026-07-19
  lang: en
  translation-of:
  generated: false
---

## Requirement

When `arqix publish site` stages the default language's corpus, arqix SHALL stage the generated graph explorer page alongside the staged pages.

### Notes

The page reaches readers through the site without a committed copy (ADR-0020): staging regenerates it from the current corpus, exactly as the specification catalogue is staged on demand.
Derived from US-08-01-42.
