---
id: ADR-0009
title: Documentation Production Policy
slug: documentation-production-policy
iri: arqix:adrs/adr-0009

rdf:
  type:
    - arqix:classes/adr

triples:
  - predicate: arqix:properties/guides-design-of
    object:
      - arqix:requirements/req-04-01-13-03
  - predicate: arqix:properties/guides-implementation-of
    object:
      - arqix:requirements/req-01-01-11-01
      - arqix:requirements/req-01-01-11-03
      - arqix:requirements/req-04-01-12-01
  - predicate: arqix:properties/guides-verification-of
    object:

properties:
  decision-status: accepted

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-06
  updated: 2026-07-06
  lang: en
  translation-of:
  generated: false
---

## Documentation Production Policy

### Context

arc42 documents the architecture, and the Rust core now carries `//!`
doc-strings and `arqix:implements`/`verifies` markers. What is missing is a
policy for the *rest* of the documentation a tool like this warrants —
classic systems-engineering artefacts (an Interface Control Document, a
Concept of Operations, a User Manual, a CI/Ops Runbook) — and, more
fundamentally, a way for any document to **link to the code it describes**.

A mapping of the corpus established four facts that shape the decision:

- The ontology already **splits authored documentation** (`unit`,
  `document-page`, both under `documentation`) from **generated evidence**
  (`report`, under `verification-artefact`). New prose belongs on the
  documentation side; generated tables are `report` fragments the prose
  includes.
- The assembler (`assemble build`) is the documentation pipeline: authored
  units plus `arqix:include` fragments compose into pages. "Authored vs
  generated" is therefore mostly **hybrid**.
- There is **no code→documentation link**. `documents-artefact` (unit →
  artefact) exists in the unit template but is unused, has no inverse, and
  code appears in the trace graph only as a path-keyed generic node, not a
  first-class `code-artefact`. Report question **Q-08** already registered
  this as the open decision and named `arqix:documented-by` as the
  candidate marker.
- Several classic documents would be **pure duplication**: an SRS is the
  requirement corpus plus stories plus the EARS/RFC-2119 style guide plus
  arc42; a Glossary is arc42 chapter 12; a Data Dictionary is the ontology
  plus the trace field vocabulary (which belongs inside the ICD).

### Decision

1. **Reuse the ontology, add no interface class.** New documents are
   `unit`s composed into a `document-page`, discriminated by
   `properties.section-kind` — exactly as arc42 reuses `arc42-chapter`. No
   dedicated ICD/ConOps/Manual/interface class is introduced.
2. **Code→documentation is a marker.** A code-side
   `// arqix:documented-by <unit-iri>` comment, parsed like
   `arqix:implements`, is the inverse of a unit's `documents-artefact`
   triple; the new ontology property `documented-by` is declared
   `owl:inverse-of documents-artefact`. This makes the link traversable
   from the code side and settles Q-08. `documents-artefact` stays
   `unit`-domain (links live on the included units; pages inherit).
3. **`section-kind` is a controlled vocabulary.** `check_frontmatter.py`
   enforces `properties.section-kind` against a registered set (FM-007), so
   the corpus stays machine-partitionable.
4. **One generator surface.** Generated documentation fragments enter the
   catalog-first `arqix report` / `QUESTIONS.md` machinery (ADR-0008) — a
   CLI Command Reference as **Q-11**, a Code Reference as **Q-12** — not a
   second `arqix doc gen` path.
5. **rustdoc is a gated layer.** `cargo doc` plus a doc-lint gate
   (`missing_docs`, broken intra-doc links) join the `verify` sub-steps,
   discharging the rustdoc layer REQ-01-01-11-03 mandates.
6. **`schema_version` is per-interface.** Each wire contract owns its own
   version rather than one global number, and the assembly log gains a
   `schema_version` field.

The **document set** this policy authorises, in priority order: **ICD**
(the machine interface contract — the highest-value new document, because
agents are the primary consumers and have no authored input contract
today), then the generated **CLI Command Reference** and **Code
Reference**, the hybrid **Diagnostics & Exit-Code Registry**, the **User
Manual**, the **ConOps**, and the **CI/Ops Runbook**. Each is built
story-first like every other feature.

### Alternatives Considered

- **A dedicated `interface`/`conops` class and a `specifies-interface-of`
  property:** rejected — it grows the vocabulary against the
  minimal-ontology ethos; `section-kind` + `documents-artefact` /
  `references-artefact` already express it.
- **`code-artefact` individuals per module instead of a marker:** viable,
  and may still back the marker as generated graph nodes, but as the
  *primary* convention it is boilerplate a human maintains; a code-side
  marker is agent-authorable and co-located with the code. (The two are
  not exclusive — the follow-up may mint individuals from the markers.)
- **A second `arqix doc gen` surface:** rejected — two generation paths
  drift; the catalog-first report machinery already exists.
- **One big handbook covering everything:** rejected for the same reason
  ADR-0008 rejected one big report — it converges on a dump no single
  consumer reads.

### Consequences

- This change ships the policy plus its first realisation: the `documented-by`
  ontology property, the `section-kind` enum + checker rule, and the **ICD**
  as the first `icd-*` unit family (six units composed into
  `page-icd-machine-interface`), linking to the governing ADRs/requirements
  via `references-artefact`.
- Follow-up slices, in order: (1) parse `arqix:documented-by` in the trace
  engine, promote code to first-class `code-artefact` nodes, and make Q-08
  (`doc-to-code.md`) live; (2) the Q-11/Q-12 generators and the rustdoc +
  doc-lint gate in `verify`; (3) the Diagnostics & Exit-Code Registry, then
  the User Manual, ConOps, and CI/Ops Runbook.
- Until the trace engine learns the marker, ICD units reference their
  governing specs (ADRs, requirements) rather than raw source; the code
  edges materialise with slice (1).
