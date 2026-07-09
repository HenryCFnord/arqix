# Agent Orchestration Patterns

This document describes the multi-agent patterns used while building arqix, and what they mean for arqix in both directions: how agents working in arqix-governed projects profit from them, and how arqix itself can use them.
It is a non-normative knowledge document; process rules live in AGENTS.md.
Every pattern below was observed in real development runs of this repository, not designed on paper.

## Building blocks

Three mechanics make the patterns work; all three have direct counterparts in arqix's own design.

- **Deterministic orchestration.**
  A plain script — loops, conditionals, fan-out — decides which agents run and how results flow.
  No model interprets intermediate results; the orchestration itself is reviewable code.
- **Schema-forced results.**
  Every agent must return its result as JSON validated against a declared schema.
  The orchestrator routes results mechanically, exactly as arqix consumers parse the tool-wide diagnostics shape instead of prose.
- **A journal.**
  Each run writes an append-only JSONL log: one record per agent start and per validated result, keyed by a hash of the agent's instructions.
  Interrupted or revised runs resume from the journal; unchanged steps replay from it instead of re-running.
  This is the same discipline as `pages/assembly.jsonl` — one record per step, deterministic, resumable.

## The patterns

### Finder–Skeptic (adversarial verification)

One **finder** per dimension searches for defects with a narrow lens and must return evidence (commands run, output observed), with an empty list as an explicitly valid result.
Every finding then goes to a **skeptic** whose instruction is inverted: default to *refuted*, reproduce independently, confirm only what is real, reproducible, and in scope.

The value is the interest conflict by design: finders are tuned to find, so they also produce plausible-but-wrong findings; the skeptic does not judge the finder's argument but rebuilds the reproduction against reality.
What survives both filters is robust.
In the phase-4 review of this repository, 9 raw findings became 7 confirmed — and all 7 were real defects, including a panic on contract-permitted input and a losslessness break.

### Mapper (read-only fact sweep)

One agent per subsystem gathers **cited facts** (claims with file references) plus explicitly named gaps, before any design or judgement happens.
Mappers produce raw material, not opinions; downstream agents argue only from mapped facts.
The ADR-0009 documentation strategy was grounded this way: six mappers over ontology, unit model, generated reports, code self-documentation, manual surface, and interface surface.

### Panel + Critic (perspective-diverse design)

Several designers receive the **same question** from deliberately different angles — for ADR-0009: the systems-engineering canon, generation-first, and consumer-driven views — each blind to the others.
A single **completeness critic** then reconciles the drafts: overlaps, gaps, model violations, and one synthesized recommendation.
Diversity here catches what redundancy cannot: three agents with the same brief converge on the same blind spots; three briefs do not.

### Patterns not yet used here

- **Judge panel:** n independent solution attempts, scored by independent judges, synthesis from the winner with the best ideas of the runners-up grafted in.
  Fits wide solution spaces where one-attempt-iterated gets stuck in a local optimum.
- **Loop-until-dry:** finder rounds repeat until k consecutive rounds surface nothing new.
  Fits completeness-critical sweeps ("find *all* the broken links"), where a fixed finder count silently truncates the tail.
- **Perspective-diverse verification:** three skeptics with distinct lenses (correctness, security, reproducibility) instead of one, when a finding can fail in more than one way.

## a) How agents in arqix projects profit

The patterns need mechanical ground truth to verify against — and that is precisely what arqix provides to a consuming project.

- **Skeptics become cheap.**
  "Reproduce independently" collapses to running `arqix verify`, `trace scan`, or `lint run` and comparing byte-stable output.
  Determinism (REQ-00-00-00-01) removes flakiness ambiguity from every verdict; stable diagnostic codes and exit codes make verdicts machine-checkable instead of interpretive.
- **Dimensions come for free.**
  The command-ownership table, the requirement families, and the story groups are ready-made finder briefs: one finder per component, per persona group, or per requirement kind, instead of hand-inventing a review taxonomy per run.
- **Evidence is a first-class artefact.**
  Finders cite trace-graph edges, coverage entries, and report units (file, line, stable code) rather than prose impressions; a reviewer can follow every citation.
- **The authoring contract is explicit.**
  The ICD's input grammars define what agents write (markers, directives, triples), and the gate validates it (TRC, LNT, ONT rules) — an agent fleet can split authoring and validation across roles without inventing its own conventions.

## b) How arqix can use the patterns

The deeper observation runs the other way: **arqix's gate already is a skeptic — a mechanical one.**
An author (human or agent) proposes; the checkers refute with stable rules; only what passes reaches `main`.
The finder–skeptic pattern is the LLM generalisation of the same asymmetry, for properties no checker can decide.

That yields a design principle:

> Push every decidable property into the mechanical gate; spend LLM skeptics only on undecidable ones.

A rule that can be a checker (marker grammar, link symmetry, frontmatter order) should never stay an agent judgement — the mechanical skeptic is cheaper, deterministic, and runs on every commit.
What remains for agent review are the undecidable properties: does the documentation still describe the behaviour, does the ICD match the wire output, is a requirement still meaningful.

Concrete uses on the roadmap's horizon, in rough order of leverage:

1. **Drift review as a repeatable practice.**
   The dimensions used to review this repository (ICD accuracy against the running binary, arc42 claims against reality, corpus semantics against the command surface) generalise to any arqix-governed project, because they only rely on arqix's own contracts.
   A documented review recipe — dimensions, skeptic instruction, scope rules — could ship alongside the handbook's agent chapter.
2. **`mcp serve` as the fleet substrate.**
   Once arqix speaks MCP, finders and skeptics consume `doc search/read/list` and the trace graph natively instead of shelling out — the review patterns become first-class arqix clients.
3. **Journals as evidence.**
   `report bundle` packages deterministic evidence; an agent-review journal (one validated record per finding and verdict) is the same shape of artefact and could join a bundle as review evidence.
4. **A verification-method question.**
   arqix's ontology models verification methods (analysis, demonstration, inspection, test).
   Whether adversarially-verified agent review becomes a recorded method alongside them is an open design question — it would need the same honesty rules as everything else (a verdict without a reproducible journal is not evidence).

## Pointers

- The gate and its rules: AGENTS.md, `scripts/arqix verify`, the checker scripts under `scripts/`.
- The authoring contract for agents: the ICD (`docs/en/architecture/icd/`), especially the input-grammars unit.
- The documentation production policy the panel pattern produced: ADR-0009.
