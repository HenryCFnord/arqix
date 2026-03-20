# arqix

`arqix` is an early Rust CLI for structured technical documentation and Architecture-as-Code workflows.

The project starts from a simple observation: Markdown with YAML frontmatter is already good enough for a large amount of technical work. It is readable, portable, versionable, and fits naturally into Git-based engineering practice. The problem is not Markdown itself. The problem is what usually happens around it: documents grow into large monoliths, metadata becomes inconsistent, links drift, and traceability across decisions, requirements, implementation, and tests becomes expensive to recover.

`arqix` explores a small, practical alternative:

- write in normal Markdown
- keep metadata explicit and machine-meaningful
- break large documents into smaller units
- assemble larger outputs deterministically
- validate structure without turning documentation into ceremony

The intent is not to build a documentation empire. The intent is to make technical documents more usable as part of the engineering system itself.

## Why It Exists

Several useful practices already exist: ADRs, user stories, arc42, DDD, TDD, architecture notes, requirements baselines, and Docs-as-Code workflows. In many teams they remain adjacent rather than connected. The result is familiar:

- office-document copies diverge from the repository
- technical rationale lives in chat or memory
- architecture descriptions age faster than code
- traceability is rebuilt manually when someone needs it
- AI tooling is pointed at large, messy text blobs instead of clean document units

`arqix` exists to test whether a text-first, Git-friendly, machine-structured documentation workflow can stay lightweight while still improving coherence and future traceability.

## Current Status

This repository is intentionally minimal.

- one binary crate
- early project documentation
- no plugin model
- no UI
- no graph backend
- no fake production architecture

The immediate goal is to establish a coherent base that can evolve through real use.

## Rough Roadmap

Near-term work:

1. Define the first document-unit and metadata conventions.
2. Implement basic parsing and validation for Markdown with YAML frontmatter.
3. Add deterministic assembly for small sets of units into stable outputs.
4. Start using the repository's own `docs/` tree as the first real corpus.
5. Add focused checks for broken references and structural drift.

Longer-term directions:

- traceability views across documents and code references
- opinionated formatting and normalization
- project-level configuration for schemas and assembly rules
- export paths for larger deliverables without losing source granularity

## Repository Map

- [`docs/index.md`](docs/index.md): main documentation entry point
- [`docs/project/roadmap.md`](docs/project/roadmap.md): minimal roadmap
- [`docs/project/ai-transparency.md`](docs/project/ai-transparency.md): project AI usage policy
- [`docs/blog/2026-03-20-why-arqix-had-to-exist.md`](docs/blog/2026-03-20-why-arqix-had-to-exist.md): initial project blog post

## License

The final license is not settled yet. See [`LICENSE`](LICENSE) for the current placeholder note before publishing a stable first release.
