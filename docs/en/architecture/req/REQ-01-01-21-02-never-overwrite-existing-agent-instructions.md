---
id: REQ-01-01-21-02
title: Never Overwrite Existing Agent Instructions
slug: never-overwrite-existing-agent-instructions
iri: arqix:requirements/req-01-01-21-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-21
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: doc init in a repository with an existing AGENTS.md leaves the file byte-identical.

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

Where an agent instruction document already exists, `arqix doc init` SHALL leave it unchanged.

### Notes

Derived from US-01-01-21 (agent-onboarding strand).
The same never-overwrite discipline the template scaffold follows (REQ-01-01-20-02): authored content always wins over generated starting points.
