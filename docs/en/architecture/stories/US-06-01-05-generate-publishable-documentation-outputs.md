---



id: US-06-01-05
title: Generate Publishable Documentation Outputs
slug: generate-publishable-documentation-outputs
iri: arqix:user-stories/us-06-01-05

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-06
  - predicate: arqix:properties/has-requirement
    object:
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-06-01

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


## Generate Publishable Documentation Outputs

As an architect, I want to generate publishing outputs, so that architecture and handbook documentation can be reviewed in publishable forms such as PDF and website output.

### Acceptance Criteria

- [ ] `arqix publish` generates publishing outputs for PDF and/or website targets.
- [ ] Assembled pages are artefact-ready, and site build orchestration is supported when configured.
- [ ] `arqix render pdf` runs Pandoc on assembled pages or selected Markdown files.
- [ ] Pandoc `--defaults` is supported, and `--template eisvogel` is supported when configured.
- [ ] Artefacts are stored according to configured artefact mode and tool errors are forwarded cleanly.
- [ ] Per-doc-package render configuration and overrides are supported.

### Notes

Acceptance should cover both successful artefact generation and clean failure forwarding when Pandoc or site generation returns an error. Add integration-style tests for defaults handling, per-package overrides, and artefact placement under the configured mode. Defer deployment automation details unless they are needed to prove the documented GitHub Pages path end to end. The main value for Aria is reviewable and navigable architecture documentation outputs.
