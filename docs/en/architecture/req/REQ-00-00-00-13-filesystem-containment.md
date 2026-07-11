---
id: REQ-00-00-00-13
title: Filesystem Containment
slug: filesystem-containment
iri: arqix:requirements/req-00-00-00-13

rdf:
  type:
    - arqix:classes/constraint

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-07
      - arqix:user-stories/us-02-01-09
      - arqix:user-stories/us-04-01-02
      - arqix:user-stories/us-05-01-04
      - arqix:user-stories/us-06-01-04
      - arqix:user-stories/us-08-01-08
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: No command opens paths outside the repository root or the configured allowed roots; traversal attempts are rejected with a diagnostic.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-03
  updated: 2026-07-10
  lang: en
  translation-of:
  generated: false
---

## Requirement

The arqix CLI SHALL NOT access files outside the repository root and the configured allowed roots.

### Notes

Security requirement from the NFR pass; generalises the include-root restriction and the change-scope guardrails to all filesystem access.
