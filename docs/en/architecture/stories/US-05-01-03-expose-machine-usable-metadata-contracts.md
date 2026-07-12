---
id: US-05-01-03
title: Expose Machine-Usable Metadata Contracts
slug: expose-machine-usable-metadata-contracts
iri: arqix:user-stories/us-05-01-03

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-05
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-01
      - arqix:requirements/req-00-00-00-05
      - arqix:requirements/req-01-01-10-01
      - arqix:requirements/req-01-01-10-02
      - arqix:requirements/req-01-01-10-03
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-05-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: retired
  owner: hcf
  created: 2026-03-30
  updated: 2026-07-11
  lang: en
  translation-of:
  generated: false
---

## Expose Machine-Usable Metadata Contracts

As an AIOps engineer, I want schema-backed metadata contracts for each document kind, so that tooling and agents can rely on stable, machine-readable metadata expectations.

### Acceptance Criteria

- [ ] Document kinds can declare required and optional metadata fields in a schema contract.
- [ ] Lint surfaces missing, extra, and type-invalid fields deterministically.
- [ ] Templates and validation use the same contract source.

### Notes

Acceptance should verify that metadata contracts are authoritative for document kinds and that templates and validation stay aligned over time.
Add fixtures for missing fields, extra fields, and type-invalid values across multiple document kinds.
Keep the contract source singular so frontmatter drift is caught early and templates remain enforceable.
The main value for Alex is predictable metadata for automation, search, and downstream systems.

Retired in the consolidation sweep of 2026-07-11: this story is a persona clone — its non-cross-cutting requirements are canonically owned by US-01-01-10, and the requirements' derived-from provenance keeps this story's contribution on record.
