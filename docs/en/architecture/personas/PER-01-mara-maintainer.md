---
id: PER-01
title: Mara Maintainer
slug: mara-maintainer
iri: arqix:personas/per-01

rdf:
  type:
    - arqix:classes/persona

triples: []

properties:
  role: Documentation maintainer
  description: Maintains documentation standards, templates, schemas, and tooling to keep docs consistent, lintable, and safe for long-term automation.

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-03-05
  updated: 2026-07-08
  lang: en
  translation-of:
  generated: false
---

## Mara Maintainer

Mara owns the repository’s documentation standards.
She cares about consistency, long-term maintainability, and predictable tooling.
For Mara, arqix is not a Markdown generator.
It is a process enforcer for documentation-as-code.

### Goals

- Keep documentation structures consistent and lintable.
- Maintain templates, schemas, and conventions.
- Prevent drift in IDs, metadata, and references.
- Enable CI gating and agent-safe workflows.

### Success Looks Like

- New documents follow templates and schemas without manual cleanup.
- `fmt` and `lint` produce deterministic, actionable diagnostics.
- Duplicate IDs and broken references are caught early.
- The documentation system scales with team size.

### Pain Points

- Inconsistent frontmatter keys across documents.
- Broken includes and silent structure drift.
- “Just this once” exceptions turning into permanent debt.
- Large PRs that mix content changes with formatting churn.

### Typical Workflow with arqix

Mara defines the baseline configuration, sets up templates, and documents conventions in the handbook.
She regularly runs formatting and linting, reviews diagnostics, and adjusts rules when real-world writing patterns require it.

### Important arqix Capabilities and Commands

- `config show`, `config validate`
- `doc new`
- `fmt`
- `lint run`
- `trace scan` (as a quality baseline)
- `agent verify` (later) or the just recipes (`just verify`)

### artefacts They Care About

- `arqix.toml`
- Templates and schema definitions
- ADRs and handbook rules
- Lint and trace reports

### Boundaries

Mara does not need to author every technical detail.
She focuses on the system and standards that make documentation reliable for everyone.

### Open Needs

Mara benefits from a clear i18n strategy, strong diagnostics contracts, and safe automation guardrails for coding agents.
