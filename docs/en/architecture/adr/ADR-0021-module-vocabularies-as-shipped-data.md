---
id: ADR-0021
title: Module Vocabularies as Shipped Data
slug: module-vocabularies-as-shipped-data
iri: arqix:adrs/adr-0021

rdf:
  type:
    - arqix:classes/adr

triples: []

properties:
  decision-status: accepted

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-19
  updated: 2026-07-19
  lang: en
  translation-of:
  generated: false
---

## Module Vocabularies as Shipped Data

### Context

The layered ontology (ADR-0017) promises that each process module brings its classes and properties, yet the vocabulary exists only as one corpus's ontology documents.
A project that selects `story-driven` has no definition of `arqix:classes/user-story` in its own tree: the vocabulary check rejects the type, and a corpus without a `docs/ontology` directory cannot run the frontmatter gate at all.
The promise needs a distribution mechanism, a precedence rule for overlapping definitions, and an answer to whether rule bindings travel with it.

### Decision

**Modules ship their vocabulary inside the binary; the effective ontology layers corpus over modules over core; bindings stay fixed by the module.**

1. **Distribution: embedded at compile time.**
   The module catalog embeds the ontology documents of the authoring corpus directly (compile-time inclusion) — the documents stay the single source of truth, and a consuming project needs no files, no sync step, and no freshness gate.
   Updating a module vocabulary is an ordinary corpus change here, effective everywhere with the next binary.
2. **The effective ontology.**
   For every check, the ontology is resolved as three layers in rising precedence: the reserved core, the vocabularies of the effective process modules (`[process].modules`; an unconfigured corpus has every shipped module, preserving present behaviour), and the corpus's own ontology documents.
   A corpus without a `docs/ontology` directory is valid — the layers below carry the vocabulary, and the former hard error falls.
3. **Precedence and the reserved core.**
   A corpus definition of a module IRI overrides the embedded one — which also keeps the authoring corpus, whose documents are the embedded layer's source, valid without a special case.
   Reserved-core IRIs are different: a corpus may re-declare one only with identical semantics (type, subclass parents, domain, range); a divergent redefinition is a finding (ONT-009).
   Shadowing means changing — re-stating the same definition is authorship, not shadowing.
4. **Bindings stay module-fixed.**
   The rule families a module activates keep their class and property bindings as shipped; `arqix.toml` selects modules but never rebinds a rule to different vocabulary.
   A module remains a curated bundle — rules, vocabulary, and binding travel together (the ADR-0017 refusal of a construction kit).

The shipped partition: the reserved core carries the artefact taxonomy, documentation structure (document-page, unit, report, adr, ontology-definition), and the marker, translation, and supersession properties; `story-driven` carries persona, workflow, user story, the requirement family, verification methods, and their properties; `knowledge-base` carries source, claim, and `supported-by`.

### Alternatives Considered

- **Materialized copies in the consuming project** (generated `docs/ontology/<module>/` with a freshness gate): rejected — every consumer inherits gate machinery and a drift surface for data it does not author.
- **Copy-on-init, project-owned afterwards:** rejected — the module vocabulary degrades to a template; every project forks it and shared semantics dissolve.
- **A TOML vocabulary:** rejected by ADR-0017 — a parallel source of truth beside the ontology documents.
- **Per-project rebinding of rule vocabulary:** rejected for now — it multiplies the test surface toward the rule construction kit ADR-0017 refused; rebinding can return when a real corpus needs it.

### Consequences

- Embedded documents parse through the same reader as corpus documents — one grammar, no second code path.
- The ONT family gains the shadowing rule (ONT-009) over the reserved core; module-IRI overrides stay silent by design.
- A fresh corpus validates immediately after selecting modules; deselecting a module really removes its vocabulary ("not every project uses user stories" is now configuration, all the way down to the checker).
- Packaging the crate must include the ontology documents the binary embeds.
