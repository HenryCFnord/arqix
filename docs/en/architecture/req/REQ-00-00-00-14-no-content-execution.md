---
id: REQ-00-00-00-14
title: No Content Execution
slug: no-content-execution
iri: arqix:requirements/req-00-00-00-14

rdf:
  type:
    - arqix:classes/constraint

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-01-01-07
      - arqix:user-stories/us-04-01-02
      - arqix:user-stories/us-08-01-08
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: Document content never triggers process execution; external tools run only when explicitly invoked and configured.

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

The arqix CLI SHALL NOT execute code or shell commands embedded in processed documents.

### Notes

Security requirement from the NFR pass; document-triggered execution would break the agent-safe containment the guardrail stories demand.
