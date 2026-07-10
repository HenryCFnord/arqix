---
title: "Spec sweep: unverified stories against the phase-4 reality"
date: 2026-07-09
status: draft
category: docs
branch: docs/refinement-2026-07
---

# Spec sweep

Every story with zero verified requirements (n = 17), classified against the shipped phase-4 surface (`src/main.rs` clap tree, arc42 chapter-5 command ownership, ADR-0005/0009), plus three partial-coverage stories whose one open requirement is the known `doc new` flag discrepancy.
Decision D4 of [PLANS.md](PLANS.md): confirm or overrule per story; SUPERSEDED and UNDERSPECIFIED items then get their own correction change before implementation.

## Method

The candidate set is every story the generated story-progress report (kept current by the report-freshness gate) lists with zero verified requirements, plus partial-coverage stories whose only open requirement is a known surface discrepancy.
A read-only agent (mapper pattern, `docs/en/processes/agent-orchestration-patterns.md`) compared both sides per story: the specification side (the full story text, its requirements sampled via the `has-requirement` triples) against the shipped reality (the clap tree in `src/main.rs`, the assembler's directive handling, the shipped `trace check`, the stubs, and the normative anchors ADR-0005, ADR-0009, and the arc42 chapter-5 ownership table).

The categories are operational, not impressions:

- **SUPERSEDED** requires a concrete, citable contradiction between an acceptance criterion and shipped code.
- **UNDERSPECIFIED** means no precise red test can be written — the bar is the TDD flow itself (a red skeleton that cannot be sharpened is not implementable).
- **DEFERRED** means internally sound but dependent on something ADR-0009 or the roadmap schedules later, with the dependency named.
- **FITS** is the residual class: no contradiction found, trigger/output/error case sufficiently determined.

Honest limitation: this was one finder without an adversarial verification pass (unlike the PR-#20 review) — acceptable because the output is a decision list whose skeptic is the owner (decision D4), not auto-applied fixes.
The error characteristics are asymmetric: SUPERSEDED/UNDERSPECIFIED entries rest on positive, cited contradictions; **FITS is the weakest claim** (absence of contradiction, with requirements only sampled) and each FITS story still gets the normal story-start check (sharpening its red skeleton) before implementation.

## FITS — implementable as specified (10)

| Story | Title | Note |
| --- | --- | --- |
| US-02-01-10 | Scaffold Translations During Implementation | greenfield extension of the Template Engine; i18n layout and `translation-of` metadata exist |
| US-06-01-06 | Create Linked Translation Documents for Architecture Content | same translation-scaffold flow, architect persona |
| US-08-01-14 | Scaffold Translations Deterministically from Source IDs | third translation clone (agent persona); trigger, output, and error case fully specified |
| US-05-01-12 | Expose Arqix via MCP over STDIO | `mcp serve` is a stub, but the store operations it adapts are shipped |
| US-08-01-12 | Use MCP Tools Deterministically in Agent Workflows | same requirements as US-05-01-12, agent persona |
| US-01-01-09 | Govern Agent Workflow Document Standards | pure governance documentation; no code conflict |
| US-08-01-18 | Standardize Agent Workflow Documents | clone of US-01-01-09, agent persona |
| US-01-01-15 | Operationalise the Release Process with SemVer | CHANGELOG/RELEASING are authoring work; align the schema_version wording with ADR-0009 (per-interface) |
| US-04-01-09 | Run Governed Release Preparation Workflows | same release-process acceptance, DevOps persona |
| US-08-01-17 | Prepare Releases within Explicit Automation Boundaries | third release clone, agent persona |

## SUPERSEDED — contradicted by shipped code (1 + 3 partial)

| Story | Title | Contradiction |
| --- | --- | --- |
| US-08-01-19 | Detect Missing Trace Markers for a Requirement | acceptance names `trace check --req REQ-xxxx`; shipped surface is positional `trace check <requirement>` and already reports implements/verifies with locations and JSON |
| US-01-01-13 (80%) | Govern Deterministic Document Creation via Templates | open requirement expects `doc new <kind> --title/--id/--dry-run`; shipped `doc new` takes only the positional kind |
| US-02-01-07 (80%) | Create Conforming Documents Quickly via Templates | same `--title/--id/--dry-run` acceptance against the flagless shipped command |
| US-08-01-23 (80%) | Create Documents without Ambiguity via Templates | same acceptance, agent persona |

Correction options for the `doc new` trio: (a) reword acceptance to the shipped surface and move the flags to a follow-up story, or (b) keep the acceptance and implement the flags as the next Template Engine slice.
The REQ-01-01-13-01/-02 sentences were already made surface-neutral in PR #20; option (b) needs no further spec change.

## UNDERSPECIFIED — no precise test writable yet (2)

| Story | Title | Gap |
| --- | --- | --- |
| US-04-01-12 | Publish Stable Report Exports for Automation | demands Markdown+CSV+JSON exports, but the global `--format` enum is text/json (ADR-0005) and format/flag selection inside `report bundle` is undefined |
| US-07-01-07 | Publish Stable Compliance-Ready Report Exports | same gap, auditor persona |

Needs a small design decision (export selection surface for `report bundle`) before the red skeleton can be sharpened — natural companion to roadmap slice 4.

## DEFERRED — sound, but dependency-blocked (4)

| Story | Title | Blocked on |
| --- | --- | --- |
| US-01-01-11 | Govern Architecture Documentation Standards | ADR-0009 follow-up slices (Q-11 generator, rustdoc + doc-lint gate in verify) |
| US-06-01-07 | Maintain Architecture and Governance Documentation Consistently | same ADR-0009 slices |
| US-04-01-13 | Publish Navigable Architecture Views Deterministically | Publish & Render orchestrator plus a cross-link/navigation validator that does not exist yet |
| US-06-01-11 | Assemble Architecture Narratives into Navigable Outputs | same publish-pipeline dependency |

## Side observation

`<!-- arqix:chapter N -->` directives in the arc42 page are currently decorative: the assembler expands only include directives, and chapter identity comes from frontmatter ids.
REQ-02-01-09-01 ("parse chapter and include directives") is therefore half-implemented — already recorded as a follow-up in the PR-#20 notes; slice 1 (publish site) is the natural home for deciding whether chapter directives gain semantics or are retired from the grammar.
