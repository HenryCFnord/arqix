---
id: REQ-04-01-17-01
title: Stage Catalogue Pages per Workflow Group
slug: stage-catalogue-pages-per-workflow-group
iri: arqix:requirements/req-04-01-17-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-17
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: With the catalogue enabled, staging produces one specification page per workflow group bundling that group's stories and requirements, and the source files stay excluded.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-12
  updated: 2026-07-12
  lang: en
  translation-of:
  generated: false
---

## Requirement

Where the publish policy enables the specification catalogue, `arqix publish site` SHALL stage generated catalogue pages that bundle the story and requirement sources one page per workflow group.

### Notes

Derived from US-04-01-17.
The workflow group comes from the story's declared `is-part-of-workflow` triple (the declared relations are the source of truth, ADR-0012); requirements join their owning story's group via `derived-from`.
