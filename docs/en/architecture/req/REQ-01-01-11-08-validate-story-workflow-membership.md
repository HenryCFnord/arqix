---
id: REQ-01-01-11-08
title: Validate Story Workflow Membership
slug: validate-story-workflow-membership
iri: arqix:requirements/req-01-01-11-08

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-11
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: A story whose id and is-part-of-workflow triple name different workflows makes the check exit non-zero and names the story and the workflow the id encodes.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-15
  updated: 2026-07-15
  lang: en
  translation-of:
  generated: false
---

## Requirement

When `arqix lint requirements` runs, arqix SHALL report every user story whose id does not encode the workflow named by its `is-part-of-workflow` triple.

### Notes

The story ID scheme encodes the owning workflow (`US-<WW>-<SS>-<NN>` sits in `WF-WW-SS`); the triple stays the source of truth and the id is the checked label, mirroring how REQ-LNK-001 binds a requirement id to its owning story (ADR-0012).
A story that names no workflow, or more than one, is reported under the same rule.
Corpora without a workflow directory skip the check, so the checker stays usable outside this repository.
