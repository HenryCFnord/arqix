---
title: "Knowledge-repository intake: user feedback from the standards use case"
date: 2026-07-15
status: draft
category: docs
branch: claude/knowledge-repository-backlog
---

# Intake

User feedback from real arqix usage on the "standards and technical knowledge bases" use case, captured 2026-07-15.
The feedback endorses the current direction (documents as knowledge objects, YAML frontmatter, verification, agent-readiness) and proposes the step from a document repository to a knowledge repository.
This package records the proposals and the observed 0.2.0 gaps as backlog, triaged against what arqix already has.
Nothing here is decided; the deferred decisions are listed at the end.

## The vision

The proposed target shape layers four graphs over the Markdown corpus.

```text
Markdown -> Frontmatter -> Entity Graph -> Evidence Graph -> Reasoning Graph
```

Plus a fourth graph the feedback singles out as the most arqix-specific opportunity: a provenance graph after the W3C PROV model (entity, activity, agent, evidence), so every statement in an agent-built knowledge base answers where it came from, whether it was inferred or extracted, whether a human reviewed it, and which agent version produced it.
The agent then works against a semantic net whose nodes are documented by Markdown, instead of against Markdown files directly.

## The eight proposals, triaged

Each proposal is recorded with what arqix has today, what is genuinely new, and the candidate home for the decision.

### P1 — Explicit ontology: entity vs document vs source

Proposed: distinguish the Markdown document, the domain entity it describes, and the evidence for it, so several documents can describe one entity.

Today: `docs/ontology/` already is an explicit, checkable ontology (classes, properties, individuals; ONT-001..006), and every document carries an `iri` plus `rdf.type`.
What is missing is the entity-as-first-class-node: today the document IS the node (ADR-0007 identity), so two documents cannot declare themselves as descriptions of the same entity.

New: an `entity` layer (or an `arqix:properties/describes` convention) separating document identity from entity identity.
Candidate home: the ontology-as-config ADR (refactor slice 8) is the natural decision point — this proposal widens exactly the question that ADR already owns.

### P2 — Semantic relations instead of free strings

Proposed: relations as `predicate`/`target` pairs so an RDF/property graph falls out.

Today: this exists — frontmatter `triples` are exactly `predicate`/`object` pairs, validated against the ontology (ONT-001/003) and projected into the trace graph.
New: user-defined predicates and external vocabularies (`conformsTo`, `supersedes` against non-arqix namespaces) without hollowing the vocabulary check.
Candidate home: slice 8/9 (ontology-vocabulary derivation) plus the IRI-namespace question (slice 10).

### P3 — Sources and claims as first-class citizens

Proposed: `evidence` records with typed sources, and `claims` whose statements point at the evidence supporting them.

Today: `external-references` carries typed source records (type, label, uri) at document level; there is no claim-level granularity and no supportedBy edge.
New: an evidence model below document granularity — the biggest genuinely new surface in this intake, and the one the feedback calls most important.
Candidate home: its own ADR (evidence model), designed together with P9 (provenance) because both annotate statements rather than documents.

### P4 — Confidence values

Proposed: every statement carries `confidence: high | inferred | estimated` to keep hallucinations out of the graph.

Today: nothing equivalent; the closest analogue is the lifecycle vocabulary (declared, checked).
New: a confidence vocabulary; small in surface, but only meaningful once P3 gives statements an identity to attach it to.
Candidate home: rides the P3/P9 ADR.

### P5 — Normative statements as structured data

Proposed: model `shall`/`should`/`may` statements as modality/subject/action/object so requirements become comparable.

Today: the requirements checker already parses exactly this — RFC 2119 keyword subsets and EARS patterns per sentence, with subject extraction — but only internally, for validation.
New: exporting that classification as data (a projection, e.g. `report` or `trace` surface), not new parsing.
Candidate home: a story on the report/export strand; low design risk because the parser exists.

### P6 — Mappings / crosswalks between standards

Proposed: a document maps one concept onto several standards (TMForum SID, DCAT, ISO 19115, OGC API), yielding a crosswalk database.

Today: expressible as triples once external namespaces are allowed (P2); no dedicated mapping semantics or crosswalk projection.
New: the external-namespace decision (slice 8/10) plus a crosswalk report unit.
Candidate home: after slice 8; the report unit is then a small ADR-0008-style addition.

### P7 — Entity versioning

Proposed: `validFrom`, `supersedes`, `deprecated` per entity, because standards live.

Today: documents have lifecycle status (ADR-0010) and ADRs have `decision-status`; there is no supersedes chain and no validity interval.
New: a versioning vocabulary — cleanly expressible as ontology properties (a `supersedes` predicate already appears in the feedback's own examples), so it rides P1/P2 rather than needing its own mechanism.
Candidate home: slice 8 ADR (vocabulary), plus a linter rule for dangling supersedes targets (see G6).

### P8 — Declarative queries

Proposed: declarative queries (`find all Concepts where conformsTo: ISO19115`) so agents navigate without prompting.

Today: `doc search` (full text), `doc list --kind`, the trace graph as JSON, and the MCP tools; agents can already filter, but only along built-in axes.
New: a query surface over the entity/triple graph.
Candidate home: deliberately last — a query language is only as good as the graph under it, so this waits until P1-P3 exist; the MCP server is the natural transport.

### P9 — Provenance graph (W3C PROV)

Proposed: per-statement provenance — activity, generating agent, reviewing human, derived-from sources, review state and date.

Today: `meta.owner`, `meta.generated`, and git history carry coarse provenance at document level; nothing at statement level, and no PROV vocabulary.
New: the PROV layer; pairs with P3 (evidence) as one design space.
Candidate home: the evidence/provenance ADR; the feedback positions this as the differentiator for agent-built knowledge bases, which matches arqix's agent-first positioning.

## Observed 0.2.0 gaps

Concrete authoring gaps from the same usage session, smaller than the proposals and mostly config-surface work.

| # | Gap | Today | Direction | Size |
| --- | --- | --- | --- | --- |
| G1 | Repository-configured custom templates for `doc new`, incl. template path and a controlled placeholder vocabulary | `[templates] dir` and per-kind `<kind>.tpl.md` files exist; the placeholder set is implicit and unvalidated, and a kind cannot point at an arbitrary template path | Extend `[kinds.<family>]` with a `template` key; document and validate the placeholder vocabulary | S/M |
| G2 | A kind declares its target root or bounded-context path | `[kinds.<family>].dir` exists for validation only; `doc new` always writes `<first-root>/<kind>/` | Make creation honour the declared `dir` (one source with the validation contract, ADR-0011) | S |
| G3 | Explicit namespace/context arguments for `doc new` (e.g. `contexts/<context>/terms/<term>.md`) | No placement argument exists | A `--path`-like argument or context-aware kinds, on top of G2 | S/M |
| G4 | Validate user-defined provenance fields (source URL, access date, local-copy path, SHA-256, licence, section anchor) | `external-references` validates only the basic shape | Extend the FM contract for source records; overlaps with P3/P9 | M |
| G5 | First-class source-record kind for URL-plus-local-copy provenance, binaries outside the tracked corpus | No source kind; binaries would land in the corpus | A `source` document kind + a policy for untracked local copies | M |
| G6 | Graph validation for unresolved `definedIn` and mapping targets | LNT-003 resolves `references-artefact` body markers; ONT-003 resolves triple objects against arqix IRIs only | Generalize target resolution to configured predicates/namespaces (depends on P2/slice 8) | M |
| G7 | Project-defined lifecycle vocabularies (`extracted`, `proposed`, `decided`) or a separate status namespace | Lifecycle rungs are deliberately code-resident (ADR-0010); slice 11 adds only a model selector | Either an ADR-0010 amendment for configurable vocabularies or a distinct, freely-configurable status field next to the guarded lifecycle | M, decision-heavy |

## Convergence with the refactor program

The intake does not compete with the running refactor program; it converges with it.

- Slice 8 (ontology-as-config ADR) is the decision point for P1, P2, P6, and P7 — the ADR's scope grows from "derive the vocabulary from the ontology" to "what the entity/relation model is", which is worth deciding once, not twice.
- Slice 10 (IRI-namespace config) gates the external-vocabulary half of P2/P6.
- Slice 11 (lifecycle-model selector) is the same design space as G7.
- G1-G3 are template-engine/config work, independent of the gate and small enough for the next authoring-ergonomics pass.
- P3/P4/P9 (evidence, confidence, provenance) form one new design space with its own ADR pair, after slice 8.

## Suggested strands (not decided)

- Strand A — entity and relation model: fold P1/P2/P6/P7 questions into the slice 8 ADR review.
- Strand B — evidence and provenance: one ADR pair for P3 + P9 (+ P4 as a rider), after strand A settles entity identity.
- Strand C — authoring ergonomics: G1 + G2 + G3 (+ G5) as a small config/template story band, schedulable now.
- Strand D — projections and queries: P5 (normative-statement export), P6 crosswalk unit, then P8, in that order.
- G4 rides strand B (its fields are evidence fields); G6 rides strand A; G7 is its own owner decision.

## Deferred owner decisions

- Whether the slice 8 ADR absorbs the entity/relation questions (strand A) or stays narrow with a follow-up ADR.
- Whether entity identity becomes a new frontmatter layer (`entity:`) or a convention over existing IRIs and triples.
- Whether evidence/provenance (strand B) targets statement granularity from the start or begins at section granularity.
- G7: configurable lifecycle vocabularies versus a separate unguarded status namespace.
- Priority of strand C relative to the remaining refactor slices 5-7.
