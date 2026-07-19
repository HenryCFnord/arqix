---
id: US-08-01-39
title: Name Each Creation Defect Distinctly
slug: name-each-creation-defect-distinctly
iri: arqix:user-stories/us-08-01-39

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-08-01-27-03
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-08-01

properties:
  priority: low
  edge-case: false

external-references: []

meta:
  lifecycle-status: retired
  owner: hcf
  created: 2026-07-19
  updated: 2026-07-19
  lang: en
  translation-of:
  generated: false
---

## Name Each Creation Defect Distinctly

As a coding agent, I want each creation defect under its own rule id, so that automation can branch on the id instead of parsing the message.

### Acceptance Criteria

- [ ] A requested id that is already taken — explicit or minted from an id-template — is TPL-004 naming the id and its holder.
- [ ] TPL-002 covers only unknown template placeholders.
- [ ] The rule catalog reflects the split.

### Notes

TPL-002 covered two defects (an unknown placeholder and a taken id); one id, one meaning restores the diagnostics contract's branching value.
