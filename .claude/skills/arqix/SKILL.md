---
name: arqix
description: Work with a documentation corpus governed by arqix — explore documents, run the verification gate, create documents from templates, and trace requirements to tests. Use when a repository carries an arqix.toml or a docs/ corpus with arqix frontmatter, or when asked to verify, search, or extend such a corpus.
---

# Working with an arqix corpus

arqix is a CLI for documentation-as-code: Markdown documents with YAML frontmatter, declared relations as triples, and a machine-checked trace graph from requirements to tests.
This skill covers the tool surface only; the repository's normative process rules live in its `AGENTS.md` — read that first and let it win over anything here.

## Explore before editing

- `arqix doc list` — catalog of every document (id, title, kind, file); `--kind requirement` filters.
- `arqix doc read <id>` — one document by id.
- `arqix doc search <query>` — full-text search with file and line.
- `--format json` on any command gives machine-readable output with a `schema_version`.

Long-running frameworks can hold the catalog open instead: `arqix mcp serve` exposes `search`, `read`, `list`, and `trace` as MCP tools over stdio — `search` takes optional `kind` and `path` filters, `list` an optional `lifecycle` filter, and `trace` answers coverage for a requirement or story id.

## The gate

`arqix verify` runs the repository's configured verification loop (format, lint, trace-scan, coverage, ratchet) and is the definition of "green" — run it before every commit.
Coverage findings are informational by default; ratchet findings mean a previously verified requirement lost its proof — fix the cause, never the checker.

## Create and mutate

- `arqix doc new <kind> --title "..."` — new documents come from the kind's template with a planned id and path; do not create corpus files by hand.
- `arqix fmt` — the only mechanical mutator of existing documents; run it instead of hand-formatting frontmatter.
- Never edit generated artefacts (assembled `pages/`, staged sites, committed report snapshots); regenerate them.

## Trace and export

- Claim a requirement from a test with a comment marker on the line above: `arqix:verifies <REQUIREMENT-ID>`; anchor code with `arqix:implements <REQUIREMENT-ID>`; declare a planned test with `arqix:plans <REQUIREMENT-ID>`.
- `arqix trace coverage` shows the picture; `arqix trace check <requirement>` answers for one id; `--results <junit.xml>` joins real test outcomes so verified means green.
- `arqix report bundle <ID>` exports audit evidence; `arqix report knowledge` exports the corpus as an agent-ready knowledge bundle.
