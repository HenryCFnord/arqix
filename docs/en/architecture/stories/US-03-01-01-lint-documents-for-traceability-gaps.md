---


id: US-03-01-01
title: Lint Documents for Traceability Gaps
slug: lint-documents-for-traceability-gaps
iri: arqix:user-stories/us-03-01-01

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-03
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-03-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-29
  updated: 2026-04-04
  lang: en
  translation-of:
  generated: false
---


## Lint Documents for Traceability Gaps

As a QA, I want to lint documents, so that include, metadata, and ID errors are detected early as objective quality findings.

### Acceptance Criteria

- [ ] `arqix lint run` checks include targets for existence.
- [ ] `arqix lint run` reports forbidden frontmatter keys in units according to an allowlist.
- [ ] `arqix lint run` reports duplicate IDs globally across units, requirements, user stories, ADRs, and glossary entries.
- [ ] All configured checks report precise file and line context.
- [ ] Invalid input returns a failing status.
- [ ] Lint output is deterministic across repeated runs on the same input.

### Notes

The lint pass is ready when all configured checks report precise file and line context and return a failing status for invalid input.
Add targeted fixtures for missing includes, forbidden unit metadata keys, and duplicate IDs across document types.
Keep the output deterministic so findings are reproducible and easy to compare in reviews and quality dashboards.
The main value for QA is objective diagnostics and reliable evidence of structural quality issues.