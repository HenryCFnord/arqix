# arqix

A Rust CLI for structured technical documentation and Architecture-as-Code workflows.

## What is arqix?

arqix is a text-first, Git-friendly tool that works with Markdown documents, YAML frontmatter, modular document units, and deterministic assembly into larger outputs.

arqix is not a wiki engine, a static site generator, or a note-taking app.
It is a tool for engineers who want their documentation to be part of the engineering system itself.

## Why does it exist?

arqix exists to keep technical Markdown documentation structured, machine-readable, and close to the code it describes.
For background and motivation, see [Why arqix had to exist](docs/blog/2026-03-why-arqix-had-to-exist.md).

## Current status

**Early development.
Not usable yet.**

The repository is being initialized.
The CLI currently supports `--help` and `--version`; feature commands are not implemented yet.

See [docs/en/project/roadmap.md](docs/en/project/roadmap.md) for planned work.

## Building

Requires Rust (stable).
Install via [rustup.rs](https://rustup.rs).

```sh
cargo build
cargo run
```

## License

GPL-3.0-or-later.
See [LICENSE](LICENSE).
