---
id: unit-manual-01
title: Working in an arqix-Governed Repository
slug: working-in-an-arqix-governed-repository
iri: arqix:units/unit-manual-01

rdf:
  type:
    - arqix:classes/unit

triples: []

properties:
  section-kind: manual-chapter

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-12
  updated: 2026-07-12
  lang: en
  translation-of:
  generated: false
---

## Working in an arqix-Governed Repository

This chapter is for anyone — human or agent — who lands in a repository whose documentation is governed by arqix and needs to work productively from the first commit.

### Recognising the corpus

An arqix repository declares itself through `arqix.toml` at the root; a missing file simply means defaults, with `docs/` as the corpus root.
The corpus is Markdown with YAML frontmatter: every document carries its identity (`id`, `iri`), its classes, and its relations as declared triples.
`arqix config show` renders the effective configuration every command acts on.

### Reading before writing

Explore the corpus through the catalog, never by guessing paths:

- `arqix doc list` — every document with id, title, kind, and file.
- `arqix doc read <id>` — one document by id.
- `arqix doc search <query>` — full-text search with file and line.

Agent frameworks get the same three operations as MCP tools: `arqix mcp serve` speaks the Model Context Protocol over stdio, and the tools answer with the same JSON as the CLI.
Over MCP, `search` additionally takes optional `kind` and `path` filters, `list` an optional `lifecycle` filter, and a fourth tool `trace` answers coverage from the trace graph for a requirement or story id.

### The verification loop

`arqix verify` is the one-command gate: it runs the configured sub-steps — format, lint, trace-scan, coverage, ratchet — and its policy lives in `arqix.toml`, not in convention.
Coverage is informational by default (it measures progress, never gates a change); the ratchet gates regressions — a requirement that was verified must stay verified unless it is retired.
Run the loop before every commit and fix causes, not checkers.

### Writing

New documents come from templates: `arqix doc new <kind> --title "..."` plans the id, the slug, and the target path from the corpus state — creating corpus files by hand forks the conventions the templates encode.
Existing documents are reformatted only by `arqix fmt`, the single mechanical mutator; hand-formatting frontmatter invites drift the linter will flag.
Generated artefacts — assembled pages, staged sites, report snapshots — are never edited, only regenerated.

### Traceability

Requirements connect to proof through comment markers: `arqix:verifies <REQUIREMENT-ID>` above a test claims it, `arqix:implements <REQUIREMENT-ID>` above code anchors the implementation.
`arqix trace coverage` shows the current picture; `arqix trace check <requirement>` answers for one requirement.
Evidence leaves the repository as exports: `arqix report bundle <ID>` for audit-ready evidence, `arqix report knowledge` for an agent-ready knowledge bundle.

### Naming a new corpus

For a fresh corpus, the default ID policy is the recommended naming scheme: semantic IDs like `REQ-01-01-07-02` keep ownership relations readable in place, and the declared triples stay the source of truth — the consistency checks between ID shape and declared relations already exist (see the ID policy in the configuration schema).
Repositories with their own conventions configure an `id-pattern` per family instead; the checks adapt to the declared shape.

### Instructions and extension points

Normative process rules — scope, branching, testing, commit discipline — live in the repository's agent instruction document (`AGENTS.md`); `arqix doc init` scaffolds a starting point that names the loop and the entry points above.
Agent-specific extension points (skills, adapters, editor integrations) stay thin: they point at the instruction document and the tool surface, and carry no process rules of their own.
