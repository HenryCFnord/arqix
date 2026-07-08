---
id: US-01-01-17
title: Configure Discovery Scope
slug: configure-discovery-scope
iri: arqix:user-stories/us-01-01-17

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-01
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-00-00-00-06
      - arqix:requirements/req-01-01-17-01
      - arqix:requirements/req-01-01-17-02
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-01-01

properties:
  priority: low
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-08
  updated: 2026-07-08
  lang: en
  translation-of:
  generated: false
---

## Configure Discovery Scope

As a maintainer, I want to configure which directories document discovery skips, so that repository layouts with vendored or generated directories under the roots stay clean without code changes.

### Acceptance Criteria

- [ ] A `skip-dirs` key in `arqix.toml` excludes the named directories from document discovery.
- [ ] Without an override, discovery excludes the established default set.
- [ ] `config show` renders the effective skip list.

### Notes

Scope is document discovery (the store: `doc list/read/search`, lint, fmt, assemble).
The trace corpus walk keeps its fixed skip set, which mirrors the Python oracle for conformance; widening the config to the trace family is a separate decision once the oracle is retired.
The main value for a maintainer is adapting discovery to a repository's layout without forking the tool.
