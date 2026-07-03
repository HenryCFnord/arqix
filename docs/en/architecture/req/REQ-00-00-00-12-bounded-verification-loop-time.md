---
id: REQ-00-00-00-12
title: Bounded Verification Loop Time
slug: bounded-verification-loop-time
iri: arqix:requirements/req-00-00-00-12

rdf:
  type:
    - arqix:classes/quality-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-05
      - arqix:user-stories/us-08-01-13
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: On a 1000-document repository, the default verification loop finishes within ten seconds (initial budget, to be calibrated).

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-03
  updated: 2026-07-03
  lang: en
  translation-of:
  generated: false
---

## Requirement

The verification loop SHOULD complete within ten seconds on a repository of one thousand documents.

### Notes

Performance requirement from the NFR pass; a slow verification loop would undermine the one-command CI/agent loop the stories demand.
