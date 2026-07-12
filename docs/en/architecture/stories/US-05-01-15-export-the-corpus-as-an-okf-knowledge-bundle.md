---
id: US-05-01-15
title: Export the Corpus as an OKF Knowledge Bundle
slug: export-the-corpus-as-an-okf-knowledge-bundle
iri: arqix:user-stories/us-05-01-15

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-10
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-05-01-15-01
      - arqix:requirements/req-05-01-15-02
      - arqix:requirements/req-05-01-15-03
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-05-01

external-references:
  - type: specification
    label: Open Knowledge Format v0.1
    uri: https://github.com/GoogleCloudPlatform/knowledge-catalog/tree/main/okf

meta:
  lifecycle-status: done
  owner: hcf
  created: 2026-07-12
  updated: 2026-07-12
  lang: en
  translation-of:
  generated: false
---

## Export the Corpus as an OKF Knowledge Bundle

As an assessor feeding documentation into agentic tooling, I want the corpus exported as an Open Knowledge Format bundle, so that any OKF-consuming agent or catalog can use it without a translation layer.

### Acceptance Criteria

- [x] `arqix report knowledge [--out <dir>]` exports the corpus as an OKF bundle: one Markdown concept document per corpus document, includes expanded, directives stripped.
- [x] The OKF frontmatter is mapped, not invented: `type` from the declared class, `title` verbatim, `timestamp` from the declared update date where present — and nothing else is fabricated.
- [x] The bundle honours the publish scope and the lifecycle: excluded subtrees and retired documents never reach it, and identical inputs produce identical bundles.

### Notes

Owner decision 2026-07-12 after the OKF publication: arqix's units are already Markdown with YAML frontmatter, so OKF (whose only required field is `type`) is a down-mapping, exported — never adopted as the internal format, which would abandon the triples, the ontology, and the lifecycle.
The spec is v0.1 and Google-driven; the mapping is deliberately minimal so spec churn stays cheap.
This story makes WF-05-01 concrete: the knowledge bundle is the artefact agents and retrieval pipelines consume.
