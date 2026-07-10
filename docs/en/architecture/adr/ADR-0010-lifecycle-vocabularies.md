---
id: ADR-0010
title: Lifecycle Vocabularies
slug: lifecycle-vocabularies
iri: arqix:adrs/adr-0010

rdf:
  type:
    - arqix:classes/adr

triples:
  - predicate: arqix:properties/guides-design-of
    object:
      - arqix:requirements/req-03-01-09-02
  - predicate: arqix:properties/guides-implementation-of
    object:
      - arqix:requirements/req-03-01-09-01
      - arqix:requirements/req-04-01-14-03

properties:
  decision-status: accepted

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-10
  updated: 2026-07-10
  lang: en
  translation-of:
  generated: false
---

## Lifecycle Vocabularies

### Context

`meta.lifecycle-status` exists on every corpus document but carries an unregulated value (`draft` everywhere).
Three pressures make it load-bearing now: the verification-process strand needs "done" to be a machine-checked claim; the spec sweep produced superseded stories that need an honest terminal state; and the publish pipeline (phase-5 slice 1) needs a filter for what is ready to leave the repository.
The refinement discussion (plan package 2026-07-09, decision D1) settled the model recorded here.

### Decision

<!-- arqix:references-artefact arqix:requirements/req-03-01-09-01 -->
One rule governs everything: **declared states carry intent, computed states carry findings** — and the gate's job is comparing the two.
Progress within a state is a report number, never a state.

Three vocabularies, by document nature:

1. **Work items (stories):** `draft` → `specified` → `in-implementation` → `done`, terminal `retired`.
   The rungs map 1:1 to the trace graph's observable coverage states (uncovered, planned, verified), so every transition has a checkable invariant; the strictest is the done claim: `done` requires every requirement of the story to be verified by an active test.
2. **Requirements:** only `draft` and `retired` are declared.
   Everything between is computed from the trace graph; declaring it would be double bookkeeping against the story's intent (canonical-owner model).
   The v1 done check counts test verification; the ontology's verification methods (inspection, analysis, demonstration) are the prepared hook for non-test evidence.
3. **Prose documents (units, pages, personas, workflows):** `draft` → `final`, terminal `retired`.
   The transition to `final` is performed by `finalise` — the single mechanical mutator (ADR-0004) gains the single lifecycle transition.
   The publish pipeline takes only `final` documents; editing a final document returns it to `draft`.

ADRs keep their two orthogonal axes: the document text follows the prose model, while `properties.decision-status` (proposed, accepted, deprecated, superseded) stays the decision's own vocabulary — an accepted decision may live in a draft text.

`retired` documents are excluded from done checks and from progress denominators.
The vocabularies are controlled sets validated by the frontmatter contract (REQ-03-01-09-02).

### Alternatives Considered

- **A partial-progress state ("some tests green, some ignored"):** rejected — the boundary to `in-implementation` carries no decision value, the state would need re-stamping after every green test, and the gradient is already a computed number in the story-progress report.
- **A richer DOORS-style ladder (analyzed, approved, implemented, verified):** rejected — under the normative TDD rule, "implemented but unverified" cannot legally exist, and dead vocabulary values invite inconsistency.
- **Computed-only, no declaration:** rejected — a claim the gate can check needs both sides; intent ("deliberately parked", "retired") is not observable from coverage.
- **Two states (draft/done):** rejected — it loses exactly the distinction the coverage decision protects: healthy spec-first waiting versus unfinished authoring.

### Consequences

- The frontmatter checker gains per-nature vocabulary validation and the done-claim rule; both arrive with US-03-01-09 (strand 1 of the refinement).
- `finalise` gains the draft→final transition as its own later slice; until then `final` simply does not occur in the corpus.
- The existing corpus stays valid: every document currently declares `draft`.
- The spec sweep's superseded stories get an honest destination (`retired`) once the vocabulary ships.
