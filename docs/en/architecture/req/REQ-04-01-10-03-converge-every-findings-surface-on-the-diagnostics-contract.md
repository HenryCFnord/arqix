---
id: REQ-04-01-10-03
title: Converge Every Findings Surface on the Diagnostics Contract
slug: converge-every-findings-surface-on-the-diagnostics-contract
iri: arqix:requirements/req-04-01-10-03

rdf:
  type:
    - arqix:classes/functional-requirement

triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-10
  - predicate: arqix:properties/has-verification-method
    object:

properties:
  priority: high
  fit-criterion: lint frontmatter, lint requirements, and trace markers answer --format json with the shared diagnostics payload (schema_version, diagnostics with severity, code, message, and source location); trace markers keeps its coverage counters as additional keys; text output and exit codes are byte-identical to before.

external-references: []

meta:
  lifecycle-status: active
  owner: hcf
  created: 2026-07-19
  updated: 2026-07-19
  lang: en
  translation-of:
  generated: false
---

## Requirement

When `arqix lint frontmatter`, `arqix lint requirements`, or `arqix trace markers` runs with `--format json`, arqix SHALL emit its findings as the shared diagnostics payload — `schema_version` and `diagnostics` entries carrying severity, code, message, and the source location where available.

### Notes

The three commands are the last surfaces speaking a command-specific findings shape (an unversioned `findings`/`summary` form inherited from the retired reference checkers); one consumer-facing shape across the whole tool is the point of the diagnostics contract (REQ-00-00-00-03, ADR-0006 layer 2, ICD Diagnostics).
Supplementary payloads stay additive keys beside `diagnostics` — `trace markers` keeps its coverage counters — and the human-oriented text output is untouched.
Derived from US-04-01-10.
