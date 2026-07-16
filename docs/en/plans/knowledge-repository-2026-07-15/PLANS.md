---
title: "Knowledge-repository program plan"
date: 2026-07-15
status: draft
category: docs
branch: claude/knowledge-repository-plan
---

# Plan

## Summary

This plan sequences the knowledge-repository intake (IDEA.md) together with the remaining pre-0.2.0 refactor slices into one working order.
The structure is three bands: a pre-release band of independently shippable slices (owner directive 2026-07-15: pull the updates before v0.2.0 as far as possible), a decision gate where the owner settles the questions the intake deferred, and a post-release band that builds the entity, evidence, and query layers on those decisions.
Every code slice follows the AGENTS.md rules: spec-first for behaviour-visible change, characterization-first for pure refactors, `just verify` before every commit, one reviewable PR per slice.

## Band 1 — pre-v0.2.0 (schedulable now, no open decisions)

Ordered so each slice builds on the previous one's surface; K-slices come from the intake, R-slices from the refactor program.

| # | Slice | Source | Goal | Shape | Size |
| --- | --- | --- | --- | --- | --- |
| K0 | doc-new-kind-dir | gap G2 | `doc new` creates in the declared `[kinds.<family>].dir` | spec-first | done (US-08-01-25, PR #90) |
| K1 | kind-template-key | gap G1 | `[kinds.<family>].template` names the template file; the placeholder vocabulary is documented and validated (unknown placeholders are findings, not silent literals) | spec-first | S/M |
| K2 | doc-new-target-path | gap G3 | an explicit `--dir` (or context) argument places a document at `contexts/<context>/terms/<term>.md`-style paths, containment-guarded | spec-first | S |
| K3 | source-record-kind | gap G5 | a `source` document kind for URL-plus-local-copy provenance (uri, access date, local-copy path, sha-256, licence, anchor); binaries stay outside the tracked corpus | spec-first | M |
| K4 | normative-statement-export | proposal P5 | export the requirements checker's existing EARS/RFC-2119 sentence classification as data (modality, subject, pattern per requirement) — a projection, no new parsing | spec-first | M |
| R5 | checker-internal-dedup | refactor slice 5 | hoist byte-identical checker helpers, route the deferred `to_posix` sites, consolidate date validation into `src/date.rs` | characterization-first | S/M |
| R6 | required-meta-one-source | refactor slice 6 | REQ-META-001 resolves the effective `[kinds.<family>].required-meta` contract instead of its hardcoded const — the program's one high-value correctness item | spec-first | M |
| R7 | frontmatter-vocab-config | refactor slice 7 | `[frontmatter] section-kinds` and `allowed-external-types` become configuration with byte-identical defaults | spec-first | S/M, may slip past the release |

Release v0.2.0 closes the band: RELEASING.md steps, CHANGELOG stamped, tag by the owner.
K3 feeds the evidence model (band 3) its source vocabulary, so landing it pre-release keeps band 3 additive.

## Band 2 — the decision gate (owner review, no code)

The intake deferred these; band 3 starts once they are settled.

- D1 — slice-8 ADR scope: does the ontology-as-config ADR absorb the entity/relation questions (P1 entity-vs-document identity, P2 external vocabularies, P6 mapping semantics, P7 versioning properties), or stay narrow with a follow-up ADR?
- D2 — G7: configurable lifecycle vocabularies (ADR-0010 amendment) versus a separate, unguarded status namespace next to the guarded lifecycle.
- D3 — PR #87 (splitter-contract docs): rewrite the note for the post-oracle world, or close it and schedule the splitter consolidation as a refactor slice.
- D4 — evidence granularity for band 3: statement-level from the start, or section-level first.

## Band 3 — post-v0.2.0 (builds on the decisions)

| # | Slice | Source | Goal | Depends on |
| --- | --- | --- | --- | --- |
| A1 | ontology-as-config ADR | refactor slice 8 + P1/P2/P6/P7 | the one structural spec: what derives from the ontology, entity identity, external vocabularies, versioning properties | D1 |
| A2 | ontology-vocabulary-derivation | refactor slice 9 | derive requirement-subclass membership and the IVVQ vocabulary from the ontology | A1 |
| A3 | iri-namespace-config | refactor slice 10 | shared namespace constants (part a); `[kinds.<family>].iri-namespace` (part b, if D1 allows) | A1 |
| A4 | mapping-resolution lint | gap G6 | generalize target resolution (LNT-003/ONT-003) to configured predicates and namespaces | A1 |
| B1 | evidence-and-provenance ADR pair | P3 + P9 (+ P4 rider) | the evidence/claims model and the W3C-PROV provenance layer, one design space | D4, A1 |
| B2 | evidence implementation slices | B1 | source records gain claim/supportedBy edges, confidence vocabulary, provenance blocks; G4 field validation rides here | B1, K3 |
| C5 | lifecycle per D2 | gap G7 + refactor slice 11 | configurable vocabularies or the status namespace, plus the lifecycle-model selector | D2 |
| D6 | crosswalk report unit | proposal P6 | an ADR-0008-style report unit projecting the mapping edges | A1/A4 |
| D7 | query surface | proposal P8 | declarative queries over the entity/triple graph, MCP as the transport — deliberately last | A1, B1 |
| X8 | splitter consolidation | PR #87 / D3 | one splitter contract now that the oracle constraint is gone | D3 |

## Process

1. Work band 1 top to bottom, one PR per slice, each `just verify`-green and owner-reviewed.
2. Release v0.2.0 when K1-K4 and R5-R6 are merged (R7 ships if it is ready, otherwise it opens band 3).
3. Hold the band-2 review with the owner; record the decisions in this package.
4. Work band 3 in the dependency order above; A1 and B1 are owner-review slices before their implementation slices start.

## Progress

Progress is tracked in STATUS.md next to this plan; agents do not rewrite this reviewed plan.
