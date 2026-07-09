---
id: unit-arc42-06
title: Runtime View
slug: runtime-view
iri: arqix:units/unit-arc42-06

rdf:
  type:
    - arqix:classes/unit

triples: []

properties:
  section-kind: arc42-chapter

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-03
  updated: 2026-07-09
  lang: en
  translation-of:
  generated: false
---

## Runtime View

Every scenario walks the shared spine — entrypoint → config resolution → feature component → diagnostics → exit code — so only the component-specific middle is described per scenario.
Scenarios are documented once their commands ship; the publish scenario follows with its story.

### `arqix verify` — the one-command loop

<!-- arqix:references-artefact arqix:adrs/adr-0003 -->
The Verification Orchestrator sequences the configured sub-steps as child invocations of the same binary — it never implements a check itself (ADR-0003, REQ-04-01-05-*).

1. Resolve the effective configuration (REQ-00-00-00-06).
2. Run `fmt --check`, `lint run`, `trace scan`, and `trace coverage` through the stable command interface; `--fail-fast` stops at the first failing step, the default aggregates all of them.
3. Collect each step's exit code into the per-step report (the `verify` wire schema in the ICD).
4. Exit with the worst channel observed: `0` clean, `1` findings, `2` when any sub-step ended outside the findings channel.

### `arqix assemble build` — deterministic assembly

1. Discover the corpus through the Document Store (skip-dirs applied, directory symlinks never followed).
2. Map every source document to its output under `pages/`; a second source claiming an already-owned output is a structural error (ASM-003).
3. Expand `<!-- arqix:include … -->` depth-first, with the DFS stack as cycle detector (ASM-001) and repository containment enforced on every target (ASM-004, REQ-00-00-00-13).
4. Write the pages plus one JSONL record per assembly step — doc, chapter_id, out, include, sha256, bytes, at_line (REQ-04-01-01-04/-05).

### `arqix doc new` — template instantiation

1. Validate the kind as a lowercase slug so it can never escape the configured root as a path component (REQ-00-00-00-13).
2. Scan the existing document IDs and derive the next counter deterministically (REQ-00-00-00-04, REQ-01-01-13-01).
3. Instantiate the kind's template into the configured location, never overwriting (TPL-001, REQ-00-00-00-08); dates stay `TODO` for `finalise` (ADR-0004: no ambient clock).
4. Report id, kind, and path as the creation result (REQ-01-01-13-02).

### Planned

- `arqix publish site --lang` — per-language root resolution and toolchain orchestration with error forwarding (REQ-04-01-07-*); documented with its story.
