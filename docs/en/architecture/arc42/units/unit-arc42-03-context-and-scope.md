---
id: unit-arc42-03
title: Context and Scope
slug: context-and-scope
iri: arqix:units/unit-arc42-03

rdf:
  type:
    - arqix:classes/unit

triples: []

properties:
  section-kind: arc42-chapter

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-03
  updated: 2026-07-12
  lang: en
  translation-of:
  generated: false
---

## Context and Scope

arqix operates on a documentation corpus inside a git repository.
Humans and coding agents drive it through the CLI; CI runs the same commands as a gate; MCP clients consume the corpus through the built-in server.
Rendering is delegated to external tools that arqix orchestrates but never trusts with control flow.

<!-- derived from ../model/workspace.dsl (view: SystemContext) -->
```mermaid
C4Context
    title arqix — System Context
    Person(maintainer, "Documentation Maintainer", "Standards, templates, governance")
    Person(builder, "Builder", "Changes the corpus alongside code, pipelines, architecture")
    Person(assessor, "Assessor", "Consumes coverage, evidence, catalogue, site")
    Person(agent, "Coding Agent", "Deterministic story-by-story loops")
    System(arqix, "arqix CLI", "Deterministic documentation-as-code toolchain")
    System_Ext(gitRepo, "Git Repository", "Corpus, configuration, changed files")
    System_Ext(render, "Render Toolchain", "Configured renderers: Pandoc, a site command")
    System_Ext(ci, "CI & Pages", "Verification gate, site publishing")
    System_Ext(mcp, "MCP Client", "Agent or IDE consuming documentation")
    Rel(maintainer, arqix, "fmt, lint, standards")
    Rel(builder, arqix, "doc new, search, read")
    Rel(assessor, arqix, "coverage, evidence, published spec")
    Rel(agent, arqix, "verify, trace check")
    Rel(arqix, gitRepo, "reads and writes within configured roots")
    Rel(arqix, render, "invokes for PDF and sites")
    Rel(ci, arqix, "runs verify with stable exit codes")
    Rel(mcp, arqix, "search, read, list, trace over stdio")
```

External interfaces: the filesystem (bounded by REQ-00-00-00-13), the render toolchain contract (errors forwarded transparently, REQ-04-01-03-07), the exit-code contract towards CI (REQ-04-01-08-01), and MCP over stdio (REQ-05-01-12-*).
