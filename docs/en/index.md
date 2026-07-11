---
title: "arqix"
description: "Markdown as data: structured documentation, deterministic assembly, and traceability as a graph"
date: 2026-07-11
status: active
---

# arqix

arqix is a Rust CLI that treats Markdown documents as structured, verifiable units instead of loose text files.
Frontmatter carries identity and typed relations, the body stays plain prose, and traceability from requirement to test is a graph the tool checks — not a spreadsheet somebody maintains.

Everything on this site is the working corpus of the project itself: arqix verifies it, assembles it, and publishes it.

## Install

```bash
cargo install --git https://github.com/HenryCFnord/arqix
```

You need a current stable Rust toolchain; a crates.io release is planned for 0.1.0.

## Markdown as data

A document is a unit: the frontmatter is data, the body is prose.
This is an actual requirement from this corpus (abridged):

```markdown
---
id: REQ-04-01-03-02
title: Keep Assembled Pages Artefact-Ready
iri: arqix:requirements/req-04-01-03-02
rdf:
  type:
    - arqix:classes/functional-requirement
triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-04-01-03
meta:
  lifecycle-status: active
---

## Requirement

The assembled pages SHALL be artefact-ready for downstream publishing.
```

A marker in the test suite closes the loop from specification to proof — this is the actual test that verifies the requirement above:

```rust
// arqix:verifies REQ-04-01-03-02
#[test]
fn publish_site_stages_artefact_ready_inputs() {
```

One command checks the whole corpus — formatting, structural lint, the trace graph, coverage, and coverage regressions:

```text
$ arqix verify
ok   format (exit 0)
ok   lint (exit 0)
ok   trace-scan (exit 0)
ok   coverage (exit 0)
ok   ratchet (exit 0)
verify: ok
```

## Start here

- [Quick Start](getting-started.md) — from an empty repository to a verified, published corpus
- [Why arqix?](why-arqix.md) — how it compares to StrictDoc, Doorstop, Sphinx-needs, and Structurizr
- [Why arqix had to exist](blog/2026-03-why-arqix-had-to-exist.md) — the reasoning behind the project
- [Roadmap](project/roadmap.md) — where the project is going, measured by one number
- [Architecture](architecture/arc42/page-arc42-arqix-architecture.md) — the arc42 document with the C4 model, workflows, and ADRs
- [AI transparency](project/ai-transparency.md) — how AI tooling is used in this project

## How much of this is real?

arqix is early, and the specification is deliberately ahead of the implementation.
The scoreboard below is not written by hand: it is the generated [scoreboard report unit](reports/units/scoreboard.md), pulled in by an `arqix:include` directive and refreshed with every published build.

<!-- arqix:include reports/units/scoreboard.md -->
