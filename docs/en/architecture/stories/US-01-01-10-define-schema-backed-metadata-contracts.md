---
id: US-01-01-10
title: Define Schema-Backed Metadata Contracts
slug: define-schema-backed-metadata-contracts
iri: arqix:user-stories/us-01-01-10

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-01
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-01
      - arqix:requirements/req-00-00-00-05
      - arqix:requirements/req-01-01-10-01
      - arqix:requirements/req-01-01-10-02
      - arqix:requirements/req-01-01-10-03
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-01-01

properties:
  priority: high
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

## Define Schema-Backed Metadata Contracts

As a maintainer, I want schema-backed metadata contracts for each document kind, so that frontmatter drift is caught early and templates remain enforceable over time.

### Acceptance Criteria

- [ ] Document kinds can declare required and optional metadata fields in a schema contract.
- [ ] Lint surfaces missing, extra, and type-invalid fields deterministically.
- [ ] Templates and validation use the same contract source.

### Notes

Acceptance should verify that metadata contracts are authoritative for document kinds and that templates and validation stay aligned over time.
Add fixtures for missing fields, extra fields, and type-invalid values across multiple document kinds.
Keep the contract source singular so frontmatter drift is caught early and templates remain enforceable.
The main value for a maintainer is scalable documentation governance.
