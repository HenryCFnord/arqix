---
id: US-08-01-27
title: Create Documents From the Contract
slug: create-documents-from-the-contract
iri: arqix:user-stories/us-08-01-27

rdf:
  type:
    - arqix:classes/user-story

triples:
  - predicate: arqix:properties/has-persona
    object: arqix:personas/per-08
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-08-01-27-01
      - arqix:requirements/req-08-01-27-02
      - arqix:requirements/req-08-01-27-03
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-08-01

properties:
  priority: medium
  edge-case: false

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-15
  updated: 2026-07-19
  lang: en
  translation-of:
  generated: false
---

## Create Documents From the Contract

As a coding agent, I want document creation to honour the declared contract and my explicit arguments, so that a correct document needs neither hand editing nor guesswork.

### Acceptance Criteria

- [ ] `doc new <kind> --dir <path>` creates (and `--dry-run` plans) the document under the given repository-relative directory.
- [ ] The explicit directory wins over the kind's declared `dir` and the `<first-root>/<kind>/` default.
- [ ] An absolute path or a path containing `..` is a usage error (exit 2) and nothing is written — filesystem containment holds (REQ-00-00-00-13).
- [ ] Repeatable `--set key=value` fills the template's own placeholders; an unused key is a TPL-003 finding naming it, and a malformed pair is a usage error.
- [ ] A requested id another document already carries — explicit or minted — is TPL-004 naming the id and its holder.

### Notes

Third slice of the authoring-ergonomics band from the knowledge-repository intake (`docs/en/plans/knowledge-repository-2026-07-15/`, gap G3, plan slice K2).
Builds on US-01-01-22 (declared kind directories); the explicit argument covers the per-document case a per-family contract cannot.
