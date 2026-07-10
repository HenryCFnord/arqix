---
id: REQ-04-01-14-03
title: Default Coverage to Informational
slug: default-coverage-to-informational
iri: arqix:requirements/req-04-01-14-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-14
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Without any verify configuration, uncovered requirements do not fail the loop while a lint or format finding does.

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-10
  updated: 2026-07-10
  lang: en
  translation-of:
  generated: false
---

## Requirement

The default verify configuration SHALL treat coverage as informational and every other sub-step as gating.

### Notes

Derived from US-04-01-14.
Rationale recorded on the roadmap and in ADR-0010's discussion: a fully specified, not-yet-implemented corpus is a healthy state, not a failure.
