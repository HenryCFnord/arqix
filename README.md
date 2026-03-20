# arqix

A Rust CLI for structured technical documentation and Architecture-as-Code workflows.

---

## What is arqix?

arqix is a text-first, Git-friendly tool that works with Markdown documents, YAML frontmatter, modular document units, and deterministic assembly into larger outputs.

The name is a compact blend of *architecture* and *index*: the idea that structured documentation can act as a real index into a software project — not just narrative, but navigable and machine-readable.

arqix is not a wiki engine, a static site generator, or a note-taking app. It is a tool for engineers who want their documentation to be part of the engineering system itself.

## Why does it exist?

Good documentation practices already exist — ADRs, arc42, user stories, DDD, TDD. But they often live next to the project instead of inside it. The result is familiar: documentation that drifts, traceability that breaks, artifacts that exist in parallel rather than in relation.

Plain Markdown with YAML frontmatter is already close to the right answer: portable, Git-friendly, human-readable, and usable with tools like Obsidian. The problem is not the format — it is the lack of tooling that treats Markdown documents as structured units rather than loose text files.

arqix explores whether a minimal, pragmatic CLI can close that gap, without overengineering it.

## Current status

**Early development. Not usable yet.**

The repository is being initialized. The CLI skeleton compiles and prints a version string. Nothing more is implemented.

See [docs/project/roadmap.md](docs/project/roadmap.md) for planned work.

## Repository layout

```
Cargo.toml          Rust project manifest
src/                CLI source code
docs/               Project documentation (also the future self-hosted corpus)
  index.md          Main entry point for docs
  project/          Stable project-level documents (roadmap, AI transparency)
  blog/             Reflective project posts
  notes/            Informal working notes
  experiments/      Documented experiments
README.md           This file
LICENSE             GPL-3.0
.gitignore          Standard Rust gitignore
```

## Building

Requires Rust (stable). Install via [rustup.rs](https://rustup.rs).

```sh
cargo build
cargo run
```

## License

GPL-3.0. See [LICENSE](LICENSE).
