---
id: US-04-01-03
title: Generate Publishing Outputs
slug: generate-publishing-outputs
iri: arqix:user-stories/us-04-01-03

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-04
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-06
      - arqix:requirements/req-04-01-03-01
      - arqix:requirements/req-04-01-03-02
      - arqix:requirements/req-04-01-03-03
      - arqix:requirements/req-04-01-03-04
      - arqix:requirements/req-04-01-03-05
      - arqix:requirements/req-04-01-03-06
      - arqix:requirements/req-04-01-03-07
      - arqix:requirements/req-04-01-03-08
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-04-01

properties:
  priority: high
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-30
  updated: 2026-07-02
  lang: en
  translation-of:
  generated: false
---

## Generate Publishing Outputs

As a DevOps engineer, I want to generate publishing outputs, so that documentation can be published as PDF and/or a website through deterministic automation.

### Acceptance Criteria

- [ ] `arqix publish` generates publishing outputs for PDF and/or website targets.
- [ ] Assembled pages are artefact-ready, and site build orchestration is supported when configured.
- [ ] `arqix render pdf` runs Pandoc on assembled pages or selected Markdown files.
- [ ] Pandoc `--defaults` is supported, and `--template eisvogel` is supported when configured.
- [ ] Artefacts are stored according to configured artefact mode and tool errors are forwarded cleanly.
- [ ] Per-doc-package render configuration and overrides are supported.

### Notes

Acceptance should cover both successful artefact generation and clean failure forwarding when Pandoc or site generation returns an error.
Add integration-style tests for defaults handling, per-package overrides, and artefact placement under the configured mode.
Defer deployment automation details unless they are needed to prove the documented GitHub Pages path end to end.
The main value for Daria is reproducible build and publish automation.
