---
id: ADR-0007
title: Graph Node Identity
slug: graph-node-identity
iri: arqix:adrs/adr-0007

rdf:
  type:
    - arqix:classes/adr

triples:
  - predicate: arqix:properties/guides-design-of
    object:
  - predicate: arqix:properties/guides-implementation-of
    object:
      - arqix:requirements/req-03-01-05-04
      - arqix:requirements/req-03-01-05-05
  - predicate: arqix:properties/guides-verification-of
    object:

properties:
  decision-status: accepted

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-05
  updated: 2026-07-05
  lang: en
  translation-of:
  generated: false
---

## Graph Node Identity

### Context

The canonical trace graph (ADR-0006 layer 1) grew two node sources: corpus documents, whose frontmatter-link edges originate from declared document IDs, and code/test artefacts, whose marker edges originate from file paths. Two identity schemes in one graph leave the join rule implicit — consumers must know which kind of node they are matching — and the question must be settled before the Rust Trace Engine freezes the contract.

### Decision

Every node has exactly one `id`, chosen by one rule: **the declared document ID when the source declares one, otherwise the repository-relative path.** Every node carries `file` as an attribute (for artefacts, `file` equals `id`). Edges always reference node `id`s.

Artefact identity is deliberately path-based and therefore rename-unstable: the graph contains only facts declared in the corpus, and a Rust source file declares no identity beyond its location. Stability for artefacts comes from version control, not from invented names.

### Alternatives Considered

- **Status quo without a rule:** rejected — the mixed model works today, but the join semantics stay implicit and every consumer re-derives them; an undocumented convention is the worst kind of contract.
- **Synthetic artefact IDs (hashes, module paths):** rejected — invented identities that appear nowhere in the corpus contradict the graph's lossless-facts principle and add collision and maintenance questions for marginal rename-stability.
- **Two separate graphs (ontology links vs marker links):** rejected — it breaks the single node-and-edge contract (REQ-03-01-05-04), coverage needs both halves together anyway, and it reintroduces the two-format problem ADR-0006 eliminated.

### Consequences

- One join rule for all consumers: match on `id`, read `file` for location.
- The Rust Trace Engine inherits the rule; the conformance suite exercises it through the skeleton tests.
- Test functions can later become declared identities (child nodes of their file's artefact node) without breaking the rule — a declared name is a declared fact.
- Marker edges carry the attached test function's name as an attribute (`test`), which is location context, not identity.
