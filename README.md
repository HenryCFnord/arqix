# arqix

A Rust CLI for structured technical documentation and Architecture-as-Code workflows.

## What is arqix?

arqix is a text-first, Git-friendly tool that works with Markdown documents, YAML frontmatter, modular document units, and deterministic assembly into larger outputs.


arqix is not a wiki engine, a static site generator, or a note-taking app. It is a tool for engineers who want their documentation to be part of the engineering system itself.

## Why does it exist?

arqix exists to keep technical Markdown documentation structured, machine-readable, and close to the code it describes.
For background and motivation, see [Why arqix had to exist](docs/blog/2026-03-why-arqix-had-to-exist.md).

## Current status

**Early development. Not usable yet.**

The repository is being initialized. The CLI currently supports `--help` and `--version`; feature commands are not implemented yet.

See [docs/project/roadmap.md](docs/project/roadmap.md) for planned work.

## Building

Requires Rust (stable). Install via [rustup.rs](https://rustup.rs).

```sh
cargo build
cargo run
```
## OpenClaw skill integration

Project-specific OpenClaw skills are maintained in `.agents/skills/`.

Selected skills are linked into the OpenClaw host skill directory via symlinks so they can be loaded by the running OpenClaw installation.

The current OpenClaw split is `arqix-repo-readonly` for inspection, `arqix-plan-intake` for branch-local planning intake, and `arqix-delivery` for plan validation before Codex and draft PR steps. The intake side now includes a thin free-text wrapper at `tools/openclaw/plan_from_idea.sh`.
See [OpenClaw mobile-first planning flow](docs/project/openclaw-mobile-first-planning-flow.md) for the workflow and [OpenClaw Plan Intake Wrapper Contract](docs/project/openclaw-plan-intake-wrapper.md) for the repository-side intake interface.

## License

GPL-3.0-or-later. See [LICENSE](LICENSE).
