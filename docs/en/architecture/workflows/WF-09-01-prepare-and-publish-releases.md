---
id: WF-09-01
title: Prepare and Publish Releases
slug: prepare-and-publish-releases
iri: arqix:workflows/wf-09-01

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
  goal: Turn a releasable tree into an immutable, validated release with the human decision kept where it belongs.
  entry-state: The gate is green on the default branch and the changelog's top section describes the unreleased state.
  end-state: A tagged release exists with validated artefacts (GitHub release, crates.io package), and the next cycle is open with the version and changelog in step.

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

## Prepare and Publish Releases

Releases repeat, and every repeated process deserves a described workflow.
The division of labour is fixed by the process contract: agents prepare releases, a human publishes them (REQ-01-01-15-03).

### Goal

Turn a releasable tree into an immutable, validated release — changelog, tag, published artefacts — with the human decision kept where it belongs.

### Steps

1. Verify the tree is releasable: the full gate, the conformance cross-check, and fresh report snapshots.
2. Confirm the release documents agree: the changelog's top section matches the crate version (machine-checked by the test suite).
3. Stamp the release: replace the unreleased marker with the ISO date (agent work, lands via review).
4. Tag the release on the default branch and push the tag; the release pipeline re-validates the exact tagged state.
5. Publish: the GitHub release with the changelog section as body, then the package registry from the tag (human work, behind the owner's credentials).
6. Open the next cycle: bump the version and open its unreleased changelog section in one commit, keeping the machine-checked consistency rule true at every commit.

### Outputs

- A validated tag and its GitHub release
- The published package (immutable; a broken version is yanked, never replaced)
- The opened next cycle (version bump + fresh unreleased section)

### Failure Modes

- The tagged state differs from the validated state (validation must run on the tag, not the branch).
- The changelog and the crate version drift (the consistency test gates this).
- Publishing from a moved branch head instead of the tag.

### Related Commands

- `arqix verify`
- `cargo publish`
