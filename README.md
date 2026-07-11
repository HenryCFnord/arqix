# arqix

<!-- markdownlint-disable-next-line MD033 -->
<img src="assets/arqix-logo-transparent.png" alt="The arqix logo: an Archaeopteryx drawn as a trace graph of nodes and edges" width="240">

A Rust CLI for structured technical documentation and Architecture-as-Code workflows.

## What is arqix?

arqix is a text-first, Git-friendly tool that works with Markdown documents, YAML frontmatter, modular document units, and deterministic assembly into larger outputs.

arqix is not a wiki engine, a static site generator, or a note-taking app.
It is a tool for engineers who want their documentation to be part of the engineering system itself.

## Why does it exist?

arqix exists to keep technical Markdown documentation structured, machine-readable, and close to the code it describes.
For background and motivation, see [Why arqix had to exist](docs/en/blog/2026-03-why-arqix-had-to-exist.md).

## Current status

**Early development.
The core works; the publication surface does not exist yet.**

The Rust core is implemented and verifies this repository's own corpus daily: `config`, `doc init/new/list/read/search`, `unit new`, `fmt`, `finalise`, `lint run`, `assemble build`, `trace scan/check/coverage/matrix`, and `verify`.
`report`, `publish`, `render`, `policy`, and `mcp` are stubs that exit with code 70 until their stories ship.

See [docs/en/project/roadmap.md](docs/en/project/roadmap.md) for planned work.

## Building

Requires Rust (stable).
Install via [rustup.rs](https://rustup.rs).

```sh
cargo build
cargo run
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
