---
id: ADR-0004
title: Finalise and the Mechanical Rewriter
slug: finalise-and-the-mechanical-rewriter
iri: arqix:adrs/adr-0004

rdf:
  type:
    - arqix:classes/adr

triples:
  - predicate: arqix:properties/guides-design-of
    object:
  - predicate: arqix:properties/guides-implementation-of
    object:
      - arqix:requirements/req-01-01-06-01
      - arqix:requirements/req-01-01-06-02
      - arqix:requirements/req-01-01-06-03
      - arqix:requirements/req-00-00-00-08
  - predicate: arqix:properties/guides-verification-of
    object:

properties:
  decision-status: accepted

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-04
  updated: 2026-07-04
  lang: en
  translation-of:
  generated: false
---

## Finalise and the Mechanical Rewriter

### Context

`arqix finalise` sets `updated` to an ISO date, rewrites nothing that is already current, fails clearly on unsupported frontmatter, and never touches body text (REQ-01-01-06-01..03, REQ-00-00-00-08). It needs a component owner. Since ADR-0002/PR #9 the lossless concrete-syntax layer lives in the Document Parser, so the machinery argument alone does not decide the question — several components could perform a targeted CST edit.

### Decision

`finalise` lives in the Formatter component, which is renamed **Formatter & Finaliser**. Its identity is: *the only component that mutates existing source documents, and it does so exclusively mechanically*.

- Constraint scoping: REQ-01-01-03-03 ("never change document meaning") binds the `fmt` command; REQ-00-00-00-08 ("mechanical metadata changes only, never body text") binds both commands. The component-level guarantee is the mechanical-only discipline, not meaning preservation.
- Mutator invariant: no other component modifies existing source documents. The Template Engine creates new files; the Assembler and Publisher write generated artefacts. Source mutation is therefore concentrated in one component — one enforcement and audit point for the containment and no-overwrite contracts.
- Clock injection: `finalise` is the only command whose correct output depends on the wall clock (`updated` = today). To keep REQ-00-00-00-01 meaningful, the date is an injected dependency (flag or clock interface), never an ambient system call; tests supply a fixed date.

### Alternatives Considered

- **Document Store & Catalog:** rejected — the store is strictly read-and-derive (discovery, IDs, catalog); making it write would break the read/write split that keeps determinism cheap, and `updated` is set by an operation, not managed store state.
- **Template Engine:** rejected — creating new files from templates and losslessly editing existing files are different machines; merging them would also break the single-mutator invariant.
- **Own "Metadata Finaliser" component:** rejected — three requirements and a one-field edit do not justify a sixteenth component, and a second source mutator would dilute the invariant. The clean-constraint benefit is achieved by command-level constraint scoping instead.

### Consequences

- One component to audit for source-document mutation; the write-path guards (idempotence, minimal diff, overwrite protection) are implemented once.
- `fmt` and `finalise` share the parser's CST read/serialize path; the finaliser is a thin targeted-edit layer.
- The injected clock becomes part of the component's test contract: identical corpus + identical injected date ⇒ byte-identical result.
- The component name must keep both commands visible; if a third mechanical rewrite command appears, it belongs here.
