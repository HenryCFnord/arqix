---
title: "Why arqix?"
description: "How arqix compares to StrictDoc, Doorstop, Sphinx-needs, and Structurizr — and when to pick which"
date: 2026-07-11
status: active
---

# Why arqix?

Requirements-as-code is not a new idea.
StrictDoc, Doorstop, and Sphinx-needs are mature tools in this space, and Structurizr owns architecture-as-code.
This page states what arqix bets on that they do not — and, just as plainly, when one of them is the better choice today.

## The bets

**Markdown as data, not a new format.**
An arqix unit is a plain Markdown file whose YAML frontmatter carries identity and typed relations as subject–predicate–object triples against a small ontology.
GitHub, editors, and diff tools render every document as-is; there is no grammar to learn and no export step between the source and something readable.
The formatter is built on a lossless concrete syntax tree with a single-mutator discipline: `arqix fmt` is byte-identical and idempotent on the whole corpus, so tooling never fights your prose.

**Traceability that reaches into code.**
`arqix:verifies` and `arqix:implements` comment markers in test and source files are edges in the same graph as the document relations.
"Which active test proves this requirement" is a query (`arqix trace coverage`), not a spreadsheet — and a marker on an `#[ignore]`d test counts as planned, not verified.

**Verification as the centre, not an add-on.**
`arqix verify` runs a configured pipeline over the corpus; coverage is informational by default while the ratchet gates regressions.
A story may only claim `done` when every requirement it carries is verified by an active test — the claim is machine-checked, not asserted.

**Agents as first-class users.**
arqix assumes coding agents work on the corpus: deterministic outputs, stable diagnostic codes, JSON everywhere, and gates that define "done" externally instead of relying on anyone's attention.
The engine itself is developed that way — a Python reference oracle is cross-checked against the Rust implementation in CI on every change.

**Publishing is orchestration, not rendering.**
`arqix publish site` stages artefact-ready pages and drives the site toolchain you configure (Zensical, MkDocs, …), exactly as Pandoc will render the PDFs.
There is no built-in renderer to outgrow.

## Tool by tool

| Tool | Source format | Traceability reach | Coupling |
| --- | --- | --- | --- |
| arqix | Markdown + YAML frontmatter | documents ↔ code ↔ tests | standalone CLI, toolchain-neutral publishing |
| [StrictDoc](https://strictdoc.readthedocs.io/) | own `.sdoc` grammar | documents ↔ source ranges | standalone CLI, own web/export pipeline |
| [Doorstop](https://doorstop.readthedocs.io/) | YAML items in Git | document items, review fingerprints | standalone CLI, Python ecosystem |
| [Sphinx-needs](https://sphinx-needs.readthedocs.io/) | rST/MyST directives | needs inside the doc build | bound to Sphinx |
| [Structurizr](https://structurizr.com/) | DSL for C4 models | architecture model, not requirements | own tooling and renderers |

**StrictDoc** is the most complete dedicated requirements tool in open source, with ReqIF interchange for regulated processes.
Its price is a dedicated grammar: requirements live in `.sdoc` files, apart from the rest of your documentation.
Pick StrictDoc if you need formal requirements documents and ReqIF exchange today; pick arqix if requirements, ADRs, arc42 chapters, and prose should be one corpus in one format.

**Doorstop** is the closest relative: requirement items as YAML files in Git, linked into a tree.
Doorstop is item-first — the prose sits in a `text:` field of a data file — and review state is tracked by content fingerprints.
arqix is document-first — the prose *is* the file, with a structured header — and done-ness is tied to tests via markers rather than to reviews.

**Sphinx-needs** embeds needs, links, and dashboards as directives in a Sphinx build, and within that world it is extremely capable.
The data model exists inside the doc build; querying it means running Sphinx, and authoring means rST or MyST.
arqix keeps the graph in a standalone CLI that works without any site build, and the publishing toolchain stays replaceable configuration.

**Structurizr** solves a different problem — C4 architecture models as code — and arqix treats it as a neighbour, not a competitor.
This corpus itself uses Structurizr for its C4 model, referenced from the arc42 document (ADR-0002).

## The honest caveat

arqix is early, and its specification is deliberately ahead of its implementation.
The [scoreboard](reports/units/scoreboard.md) on the landing page is the exact, generated answer to "how much of this is real" — that it is generated, included, and republished by arqix itself is the point.
