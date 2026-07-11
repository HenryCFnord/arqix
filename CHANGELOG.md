# Changelog

All notable changes to arqix are documented in this file.
The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/); versioning follows the policy in [RELEASING.md](RELEASING.md).
Breaking changes carry a **Migration** note in their entry.

## [0.2.0] — unreleased

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
