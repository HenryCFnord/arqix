---



id: US-05-01-13
title: Expose Language-Aware Site Outputs Deterministically
slug: expose-language-aware-site-outputs-deterministically
iri: arqix:user-stories/us-05-01-13

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-05
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-05-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-30
  updated: 2026-04-04
  lang: en
  translation-of:
  generated: false
---


## Expose Language-Aware Site Outputs Deterministically

As an AIOps engineer, I want language-aware site publishing to produce deterministic outputs per language, so that downstream indexing and agent-facing retrieval can rely on a stable bilingual site structure.

### Acceptance Criteria

- [ ] `arqix publish site --lang en` builds a site from the EN root and writes outputs to the EN artefact target.
- [ ] `arqix publish site --lang de` builds a site from the DE root and writes outputs to the DE artefact target.
- [ ] Resolved roots come from `arqix.toml` i18n configuration and are visible in effective config.
- [ ] If Zensical fails, arqix returns exit code 2 and diagnostics that identify the failing tool invocation context.
- [ ] Machine-readable diagnostics are available through `--format json`.

### Notes

In scope are language-root selection from i18n configuration, Zensical as the first site builder integration, deterministic artefact locations, and machine-readable diagnostics. Out of scope are HTML generation via Pandoc and additional site builders beyond Zensical. This story validates that the chosen i18n layout is practical for CI and automation. The main value for Alex is deterministic bilingual output structure for downstream systems.
