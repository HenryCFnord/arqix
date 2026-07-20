---
id: ADR-0023
title: The Query Surface
slug: the-query-surface
iri: arqix:adrs/adr-0023

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

## The Query Surface

### Context

Agents can retrieve along built-in axes — full-text search, the kind-filtered catalog, the trace graph as JSON — but cannot ask the graph a structured question: every document of one kind carrying one edge, say a mapping onto a given external namespace, has no single query.
The pieces the question needs all exist: declared triples, kinds, lifecycle states, and two retrieval surfaces (CLI and MCP) that answer identically by contract.
What is missing is the composition, and the temptation to close the gap with a query language is real — and expensive: a parser, an error surface, and a compatibility promise for syntax nobody asked for.

### Decision

**A query is a small structured filter set — conjunctive, no language — answered identically by `doc query` and the MCP `query` tool.**

1. **The filter set.**
   A query names any combination of a kind, a lifecycle status, and edge patterns; a document matches when it satisfies every given filter (conjunction).
   An edge pattern is a predicate plus a target: the predicate accepts the full IRI or the bare arqix property name, the target matches exactly or as a prefix with a trailing `*` (`exact-match=dcat:*`).
   Edge matching reads the declared triples from the raw frontmatter, so external targets — the crosswalk case — are first-class query material.
2. **The home.**
   `doc query` joins the Document Store surface beside `doc list`, `doc read`, and `doc search` (ADR-0005: one analysis, one command); the result is the catalog shape plus each document's matching edges, versioned like every JSON payload (ADR-0006).
3. **The MCP mirror.**
   The `query` tool joins search, read, and list, backed by the same function as the CLI — the transport-separation contract (REQ-05-01-12-03) extends to it unchanged.
4. **Deliberately absent.**
   Disjunction, negation, and multi-hop traversal wait for a real corpus question that needs them; each returns as a filter-set extension, not as a language.

### Alternatives Considered

- **A query mini-language** (`type:entity maps-to:dcat:*`): rejected — a parser with its own error and versioning surface for what structured arguments express directly; SPARQL-class engines fail the ADR-0014 bar outright.
- **Named query templates:** rejected — every new question would need a code change, which is the prompting-instead-of-navigating problem the query exists to remove.
- **A `trace query` home:** rejected — the trace noun owns the V&V machinery; knowledge retrieval lives with the store, whose CLI/MCP twin surfaces already exist.

### Consequences

- The store gains a generic declared-edge reader over raw frontmatter; the arqix-scoped triple extraction of the shared parser stays untouched.
- Agents compose retrieval from one primitive: filter, then read — no prompt engineering over prose output.
- Growth is additive: new filter axes extend the object, never a grammar.
