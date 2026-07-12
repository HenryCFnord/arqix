---
id: WF-10-01
title: Refine and Consolidate the Corpus
slug: refine-and-consolidate-the-corpus
iri: arqix:workflows/wf-10-01

rdf:
  type:
    - arqix:classes/workflow

triples:
  - predicate: arqix:properties/has-primary-persona
    object: arqix:personas/per-01
  - predicate: arqix:properties/has-relevant-persona
    object:
      - arqix:personas/per-08

properties:
  goal: Keep the specification honest over time by retiring, superseding, and merging corpus documents with provenance intact.
  entry-state: The corpus has accumulated redundancy or drift — clone documents, superseded decisions, or viewpoints that no longer earn their upkeep.
  end-state: The redundant documents are retired with successor notes, the progress denominators reflect only living work, and no relation or provenance was lost.

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-12
  updated: 2026-07-12
  lang: en
  translation-of:
  generated: false
---

## Refine and Consolidate the Corpus

A specification that only ever grows stops being honest: clone stories inflate progress denominators, superseded decisions mislead readers, and personas outlive their usefulness.
The lifecycle vocabulary (ADR-0010) gives consolidation its mechanics — `retired` is an honest destination, not deletion.

### Goal

Keep the specification honest over time by retiring, superseding, and merging corpus documents with provenance intact.

### Steps

1. Identify candidates with a mechanical, auditable rule wherever possible (for example: a story retires when no requirement carries its ID prefix and everything it co-derives is owned by an active canonical story).
2. Where the rule ends and judgement begins, stop and present the cut to the owner — consolidation of viewpoints is curation, not mechanics.
3. Retire each document by declaring `lifecycle-status: retired` and appending a note that names its successor; never delete, never rewrite history.
4. Keep relations intact: declared triples (derived-from, has-requirement) stay on record as provenance, and no active document may lose its last active source.
5. Regenerate the report snapshots; retired documents leave the progress denominators (ADR-0010), and the coverage ratchet confirms nothing verified was lost.
6. Let the gate arbitrate: lint vocabulary checks, reference resolution, and the freshness gate must all stay green through the sweep.

### Outputs

- Retired documents with successor notes
- Progress reports whose denominators count only living work
- An auditable trail: the mechanical rule, the exceptions, and the owner decisions

### Failure Modes

- A retirement orphans an active document's last source.
- Numbers change without a note explaining the rule that changed them.
- Judgement calls executed silently as if they were mechanics.

### Related Commands

- `arqix lint run`
- `arqix trace ratchet`
- `arqix verify`
