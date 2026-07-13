# Changelog

All notable changes to arqix are documented in this file.
The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/); versioning follows the policy in [RELEASING.md](RELEASING.md).
Breaking changes carry a **Migration** note in their entry.

## [0.2.0] — unreleased

### Added

- Creation aliases `req new`, `us new`, and `adr new` — the same code path as `doc new <kind>`, with `--title`, `--id`, and `--dry-run`.
- Include containment tightened (`ASM-006`): an include target inside the repository but outside every configured root is refused — corpus composition only ever reads the corpus.
- `arqix:plans <REQUIREMENT-ID>`: the language-neutral planned claim — counts as planned in coverage without framework skip syntax, satisfies the test-marker duty like `verifies`; Rust's `#[ignore]` detection stays as a convenience.
- `trace coverage --results <junit.xml>`: joined test outcomes by test name — a failing or skipped test demotes its verifying claim to planned, unjoined claims stay untouched, and coverage rows carry the outcomes for downstream evidence exports.
- `mcp serve`: the corpus as MCP tools — `search`, `read`, and `list` over stdio (JSON-RPC 2.0, one message per line), answering with the same JSON as the CLI surface; the protocol subset is implemented directly per ADR-0014, no SDK dependency.
- `publish site` stages a generated specification catalogue when enabled (`[policies.publish] specification-catalogue`): one page per workflow group bundling stories and their derived requirements, an HTML anchor per ID, live coverage status from the trace graph — the excluded spec sources return to the site in bundled form.
- `doc init` scaffolds an `AGENTS.md` agent-instructions starting point at the repository root — the verification loop and the corpus entry points named from the first commit, never overwriting an authored one.
- `arqix init`: a top-level alias for `doc init` — the discoverable entry point for repository initialisation, the same code path and arguments as `doc init` (mirrors the `req new`/`us new`/`adr new` aliases).
- `render pdf [<file>...] [--lang <lang>] [--out <path>]`: PDF artefacts via the configured renderer (Pandoc by default) — staged artefact-ready pages or selected files as input, `--defaults` and `--template eisvogel` passed through when configured, per-package overrides, artefact storage per the configured mode, tool errors forwarded transparently.
- `report bundle <ID>... [--out <dir>] [--stamp <text>]`: scoped evidence bundles — a story ID stands for the requirements derived from it; the bundle directory carries `bundle.json`, `evidence.md`, and the scoped `matrix.csv` with stable schemas and caller-provided generation metadata.
- `report knowledge [--out <dir>]`: the corpus as an [Open Knowledge Format](https://github.com/GoogleCloudPlatform/knowledge-catalog) bundle — one artefact-ready concept document per living corpus document (includes expanded, directives stripped), OKF fields mapped from declared metadata and never fabricated, publish scope and lifecycle honoured.
- `policy check <file>...`: changed files evaluated against the change scope declared in `[policies.change]` — an `allow` list of path prefixes (trailing slash for a subtree, exact entry for one file), violations as `POL-001` diagnostics, gate mode exiting 1 and warn-only mode reporting without failing; no declared policy means nothing to enforce.

## [0.1.0] — 2026-07-11

The first release: arqix verifies and publishes its own corpus.

### Added

- Command surface per ADR-0005: `config validate/show`, `doc init/new/list/read/search`, `unit new`, `fmt`, `finalise`, `lint run`, `assemble build`, `trace scan/check/coverage/matrix/ratchet`, `verify`, `publish site`; the remaining commands (`report bundle`, `policy check`, `mcp serve`, `render pdf`) are stubs with exit 70.
- Document parser, store, and catalog: frontmatter, classes and triples, full-text search, JSON output with per-interface `schema_version`.
- Linter contract checks: duplicate ids, include targets, reference markers, translation sources, lifecycle vocabulary per document nature, and the machine-checked done claim (LNT-001..005, LNT-010).
- Formatter and finaliser as the single mechanical mutator (ADR-0004), byte-identical and idempotent on the corpus.
- Trace engine as the Rust port of the Python oracle, JSON-value-equal on the corpus; coverage by requirement kind and matrices as CSV.
- Verification orchestrator with a configured sub-step policy: informational coverage by default, the coverage ratchet gating regressions against the committed matrix snapshot.
- Template engine: package scaffold, `doc new` with `--title`, `--id`, and `--dry-run`.
- Assembler with include expansion, cycle detection, and a JSONL assembly log.
- Publisher: per-language staging of artefact-ready inputs and orchestration of a configured site toolchain — arqix stages, the toolchain renders.
- Configuration schema v1 (`arqix.toml`): roots, skip-dirs, verify policy, publish policy, i18n default language.

### Changed

- License: relicensed from GPL-3.0-or-later to MIT OR Apache-2.0 (dual), the Rust ecosystem convention, ahead of the first crates.io release; sole-author relicensing, no external contributions affected.

### Security

- Containment discipline: no writes outside configured roots, no content execution, directives never reach published outputs.
