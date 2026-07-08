---
id: REQ-01-01-17-02
title: Exclude Default Skip-Dirs from Discovery
slug: exclude-default-skip-dirs-from-discovery
iri: arqix:requirements/req-01-01-17-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-17
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: low
  fit-criterion: With no `skip-dirs` override present, a document inside `node_modules` under a root does not appear in the catalog.

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-08
  updated: 2026-07-08
  lang: en
  translation-of:
  generated: false
---

## Requirement

While no `skip-dirs` override is configured, arqix SHALL exclude `.git`, `target`, `node_modules`, `__pycache__`, and `fixtures` from document discovery.

### Notes

This pins the previously hardcoded default set as the configured default, so behaviour without an override is unchanged.
The same set stays hardcoded in the trace corpus walk for oracle conformance (see REQ-01-01-17-01).
