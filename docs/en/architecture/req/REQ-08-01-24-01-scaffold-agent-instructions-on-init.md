---
id: REQ-08-01-24-01
title: Scaffold Agent Instructions on Init
slug: scaffold-agent-instructions-on-init
iri: arqix:requirements/req-08-01-24-01

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-08-01-24
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: After doc init in a repository without AGENTS.md, an AGENTS.md exists at the repository root and names arqix verify and the corpus entry points.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-12
  updated: 2026-07-15
  lang: en
  translation-of:
  generated: false
---

## Requirement

When `arqix doc init` initialises a repository without an agent instruction document, arqix SHALL scaffold an `AGENTS.md` that names the verification loop and the corpus entry points.

### Notes

Derived from US-08-01-24 (agent-onboarding strand).
The scaffold is a starting point for the repository's own normative rules (REQ-01-01-09-01), not a finished process document.
