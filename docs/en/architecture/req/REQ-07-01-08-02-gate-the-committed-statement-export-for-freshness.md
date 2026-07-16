---
id: REQ-07-01-08-02
title: Gate the Committed Statement Export for Freshness
slug: gate-the-committed-statement-export-for-freshness
iri: arqix:requirements/req-07-01-08-02

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-07-01-08
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: medium
  fit-criterion: With a committed normative-statements export that no longer matches the corpus (or is deleted), arqix report snapshot --check exits non-zero and names the file; with a fresh export it passes.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-16
  updated: 2026-07-16
  lang: en
  translation-of:
  generated: false
---

## Requirement

When `arqix report snapshot --check` runs, arqix SHALL report a committed normative-statement export whose content differs from the regenerated classification.

### Notes

The same freshness contract the report units and trace matrices carry (REQ-04-01-12-04): a missing committed export counts as stale, and the refresh rides the snapshot workflow on the default branch.
Derived from US-07-01-08 (knowledge-repository slice K4, proposal P5).
