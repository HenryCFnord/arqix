---
title: "Quick Start"
description: "From an empty repository to a verified, published documentation corpus"
date: 2026-07-11
status: active
---

# Quick Start

This page takes you from an empty repository to a corpus that arqix scaffolds, traces, verifies, and publishes.
Every command and every output block below is real: the sequence was run as-is against the current build.

## 1. Install

```bash
cargo install arqix
```

You need a current stable Rust toolchain; the [crate](https://crates.io/crates/arqix) installs the latest release, `--git https://github.com/HenryCFnord/arqix` the development state.

## 2. Scaffold a documentation package

```bash
git init my-docs && cd my-docs
arqix doc init
```

```text
initialised documentation package
```

This creates `docs/index.md`, the entry point of the package.
All arqix commands resolve their configuration from `arqix.toml` in the working directory; no file means defaults, and `docs/` is the default root.

## 3. Create your first requirement

```bash
arqix doc new requirement --title "Parse frontmatter"
```

```text
created REQUIREMENT-0001 at docs/requirement/REQUIREMENT-0001.md
```

The scaffold carries the full unit structure: identity (`id`, `iri`), typing (`rdf.type`), a `triples` list for relations, and lifecycle metadata.
Now author it — write the actual obligation into the body, declare its kind, and set it active (a requirement's lifecycle vocabulary is `active` or `retired`; there is no draft requirement, because an unfinished obligation is not yet a requirement):

```yaml
rdf:
  type:
    - arqix:classes/functional-requirement
meta:
  lifecycle-status: active
```

```markdown
## Parse frontmatter

When a document is read, the parser shall expose its frontmatter as structured data.
```

## 4. Close the loop with a marker

Traceability in arqix does not stop at documents: a comment marker attaches a test to the requirement it proves.
Create a test — any code tree works, the marker is just a comment:

```rust
// tests/parser.rs
// arqix:verifies REQUIREMENT-0001
#[test]
fn frontmatter_is_read() {
    // your actual assertion here
}
```

Then ask arqix for the coverage picture:

```bash
arqix trace coverage
```

```text
| requirement | kind | verified by | planned by | implemented by |
| --- | --- | --- | --- | --- |
| REQUIREMENT-0001 | functional | tests/parser.rs:2 | — | — |
functional: 1 verified, 0 planned, 0 uncovered (of 1)
coverage: 0 error(s), 0 warning(s)
```

An `arqix:implements` marker in source code adds the third column the same way.
Markers on `#[ignore]`d tests count as *planned*, not verified — a red skeleton is a promise, not proof.

## 5. Verify the corpus

```bash
arqix verify
```

```text
ok   format (exit 0)
ok   lint (exit 0)
ok   trace-scan (exit 0)
ok   coverage (exit 0)
ok   ratchet (exit 0)
verify: ok
```

One command runs the configured sub-steps: formatting, structural lint, the trace-graph scan, coverage, and the coverage ratchet.
Coverage is informational by default — it measures progress and never gates a change — while the ratchet gates *regressions*: a requirement that was verified must stay verified unless it is retired.
The steps and their treatment are configuration, not convention; see the [verify policy](processes/configuration.md#the-verify-policy).

## 6. Publish the site

arqix stages artefact-ready pages and orchestrates a site toolchain — it never renders HTML itself.
Configure the toolchain (Zensical is the recommended default) and point it at the staging directory:

```toml
# arqix.toml
[policies.publish]
site-command = "zensical build"
```

```toml
# zensical.toml
[project]
site_name = "my-docs"
docs_dir = "site-src"
site_dir = "site-build"
```

```bash
pip install zensical==0.0.50
arqix publish site
```

```text
Build started
No issues found
Build finished in 0.29s
staged 2 page(s) to site-src; toolchain 'zensical build' ok
```

Staged pages are artefact-ready: include directives expanded, marker comments stripped, and the frontmatter reduced to what the toolchain consumes.
Your rendered site is now in `site-build/`; add `site-src/` and `site-build/` to `.gitignore`.

## Where to go next

- The [configuration schema](processes/configuration.md) documents every policy shown above.
- The [Markdown style guide](processes/markdown-style-guide.md) and the [requirements style guide](processes/requirements-style-guide.md) describe the authoring conventions.
- [Why arqix?](why-arqix.md) positions the tool against its neighbours.
- This site is the live demo: [arqix.dev](https://arqix.dev) is published by arqix from the corpus you are reading, and the [scoreboard](reports/units/scoreboard.md) shows exactly how much of the specification is implemented.
