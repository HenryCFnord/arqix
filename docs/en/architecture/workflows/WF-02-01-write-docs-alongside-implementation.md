---
id: WF-02-01
title: Write Docs Alongside Implementation
slug: write-docs-alongside-implementation
iri: arqix:workflows/wf-02-01

rdf:
  type:
    - arqix:classes/workflow

triples:
  - predicate: arqix:properties/has-primary-persona
    object: arqix:personas/per-02
  - predicate: arqix:properties/has-relevant-persona
    object:

properties:
  goal: Create or update documentation alongside implementation work, with stable links to code and tests.
  entry-state: A developer is implementing a feature or fix and needs to consult relevant docs, templates, and verification steps.
  end-state: Documentation, code, and tests are updated together with traceable links and local checks completed before commit.

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-25
  updated: 2026-07-08
  lang: en
  translation-of:
  generated: false
---

## Write Docs alongside Implementation

A developer is implementing a feature or fix.
Documentation must be produced in the same flow, without creating a separate “docs project”.

### Goal

Create or update documentation and link it to code/tests using stable IDs, while keeping local checks aligned with CI.

### Steps

1. Read relevant docs (requirements, ADRs, handbook) and identify the target scope.
2. Create new docs via templates (`doc new`) instead of copy-paste.
3. Implement code changes.
4. Write or update tests.
5. Add trace markers (`arqix:implements`, `arqix:verifies`) where appropriate.
6. Assemble preview pages if needed to validate narrative structure.
7. Run the local verification loop before committing.

### Outputs

- Updated docs and code/tests with traceable links
- Assembled pages for review (if applicable)
- Deterministic local checks prior to PR

### Failure Modes

- Missing templates lead to inconsistent structure.
- Includes or assembling break due to incorrect paths.
- Trace markers are missing or inconsistent.

### Related Commands

- `arqix doc new <kind>`
- `arqix assemble build`
- `arqix fmt`
- `arqix lint run`
- `arqix trace scan`
- `arqix trace coverage`

### Automation (recommended)

- just: `just verify`; CI runs the same gate on every PR
