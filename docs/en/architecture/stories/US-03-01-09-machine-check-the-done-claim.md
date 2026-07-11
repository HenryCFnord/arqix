---
id: US-03-01-09
title: Machine-Check the Done Claim
slug: machine-check-the-done-claim
iri: arqix:user-stories/us-03-01-09

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-03
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-03-01-09-01
      - arqix:requirements/req-03-01-09-02
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-03-01

properties:
  priority: high
  edge-case: false

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

## Machine-Check the Done Claim

As a QA engineer, I want a story marked done to be provably done, so that lifecycle states are claims the gate checks instead of hopes.

### Acceptance Criteria

- [ ] A story with `lifecycle-status: done` whose requirements are not all verified by active tests produces an error naming each unverified requirement.
- [ ] The lifecycle vocabulary is a controlled set; an unknown value is a frontmatter error.
- [ ] `retired` stories and requirements are excluded from the done check and from progress denominators.

### Notes

The vocabularies per document nature are decided in ADR-0010: stories walk draft → specified → in-implementation → done (terminal retired); requirements declare only active/retired (in force versus retired), everything else is computed from the trace graph; prose documents walk draft → final via `finalise`.
The v1 check counts test verification; the ontology's verification methods (inspection, analysis, demonstration) are the prepared hook for non-test evidence.
