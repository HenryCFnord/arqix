---
id: ADR-0017
title: Process Profiles and the Layered Ontology
slug: process-profiles-and-the-layered-ontology
iri: arqix:adrs/adr-0017

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
  created: 2026-07-17
  updated: 2026-07-19
  lang: en
  translation-of:
  generated: false
---

## Process Profiles and the Layered Ontology

### Context

arqix has two proven application areas — clean development and knowledge-base management — but ships exactly one process: the user-story → requirement → test flowdown, hardwired together with its vocabulary.
Not every project uses user stories; not every project uses requirements; personas and workflows are already an extra layer on top; and real projects know shapes arqix cannot express today, such as hierarchical requirements whose top level is a prose wishlist rather than stories or requirements.

Every question this raises is one question: which vocabulary and which rules are arqix, and which belong to a process a project chooses?
How far the ontology becomes configuration, and whether a project may declare its own document states, are both answered by where that line runs.

A second corpus governed by arqix sharpens the same line from outside: the PSI terminology pipeline (its needs are recorded as the second intake, `docs/en/plans/knowledge-repository-2026-07-15/FEEDBACK-2026-07-17-psi.md`).
That corpus needs project-defined controlled vocabularies for named frontmatter fields (FR-C1), typed relation predicates whose targets resolve (FR-C2), bounded contexts as enforced namespaces (FR-C3), a provenance contract over its own source-record fields (FR-A2), a finding for every dangling declared reference (FR-A1), and a configured `verify` that expresses its whole project gate without a wrapper script (FR-E2).

Half of the answer already exists.
`[policies.verify].steps` selects gates per project; `[kinds.<family>]` declares artefact kinds with directory, template, id pattern, and required meta; the ontology is corpus data under `docs/ontology/` with its own checker rules.
What is hardwired is the vocabulary inside the checkers: the coupling lint knows `is-part-of-workflow` and the persona properties as Rust constants, the SRC provenance family triggers on the literal class `arqix:classes/source`, and the trace gate knows requirement → test.
ADR-0011 drew the configuration boundary — the substance of a check stays convention — but left open where a check's *vocabulary binding* falls.

### Decision

Three decisions, one line: **rules stay code; their activation and binding become configuration; vocabulary becomes layered corpus data.**

#### 1. Process profiles

arqix ships a catalog of named process modules.
A module declares the artefact kinds it requires, the vocabulary it brings (see layer 2 below), and the rule families it activates together with their bindings to classes and properties.
A project selects and parameterizes modules in `arqix.toml`; the rule implementations themselves remain hardwired Rust — there is no rule DSL, and the substance of every check keeps the ADR-0011 keep status.

Two modules ship first, because this repository dogfoods both: **story-driven development** (personas, workflows, user stories, requirements, tests; the coupling lint, the flowdown gates, the EARS/RFC-2119 style rules, the trace and coverage machinery) and **knowledge base** (source records with the SRC provenance family, and the evidence layers as they land).
Selecting no module is legal and leaves the base that is always on: frontmatter shape, markdown style, marker mechanics, link resolution, assembly, publish, and the report snapshot gate.
Further modules (hierarchical requirements with a prose needs level, ADR-only governance, verification methods as evidence) enter the catalog only when a real corpus wants them.

#### 2. Three ontology layers

1. **Reserved core** — the `arqix:` namespace: artefact, document, documentation, the marker and trace mechanics, the lifecycle axes, the ontology-definition vocabulary itself.
   Reserved names cannot be redefined or shadowed; the core is the stable ground that skills, templates, and documentation point at.
2. **Module vocabularies** — each process module brings its classes and properties.
   Persona, workflow, user story, and requirement belong to the story-driven module, not to the core; source and the provenance properties belong to the knowledge-base module.
   A project that does not select the module does not have the vocabulary — "not every project uses user stories" becomes configuration.
3. **Project ontology** — a project namespace whose classes and properties may subclass core or module vocabulary and add new properties.
   "Separate or extending" is resolved as: separately named (own namespace, never shadowing reserved names), attached through subclass and domain/range declarations.

The ontology's source of truth stays corpus documents (the existing `docs/ontology/` pattern); `arqix.toml` selects and parameterizes, documents define.

#### 3. Lifecycle stays core; domain status becomes declared vocabulary

The guarded lifecycle of ADR-0010 — declared intent versus computed findings, the three per-nature vocabularies, `finalise`, the publish filter, the done claim — is core machinery and stays unconfigurable; a module kind declares which of the three natures applies to it (work item, requirement-like, prose).
Domain states such as `extracted` → `proposed` → `decided` are a second, orthogonal axis: named `properties.*` fields with controlled vocabularies declared by a module or project and validated by the frontmatter contract, exactly the two-axes pattern `decision-status` already models on ADRs.
Projects get their own state vocabularies (FR-C1) without touching an ADR-0010 invariant.

#### 4. The checker validates the configured ontology

The existing ONT rule family (ONT-001..006, which already validates this repository's ontology documents) extends into a meta-rule family over the layered ontology and the corpus:

- every class and property a bound rule references must be defined in some layer;
- properties declare domain and range, and every declared edge in the corpus is checked against them;
- every reference target resolves — a dangling object of any declared triple is a finding (FR-A1), and orphan reports (defined but unused vocabulary, nodes without required edges) are informational;
- reserved names cannot be shadowed, and subclass chains are acyclic.

Per the ADR-0011 rules, defaults preserve the present: a corpus without profile configuration behaves exactly as this repository does today, and the shipped profiles reproduce the current gates byte for byte before anything becomes switchable.

### Alternatives Considered

- **A rule DSL (project-defined checks):** rejected — it trades sharp, tested rule semantics for an interpreted rule zoo, moves the substance of checks across the ADR-0011 boundary, and creates an unbounded compatibility surface.
  The catalog offers curated named processes instead of a construction kit.
- **One flat configurable ontology (everything replaceable, including the core):** rejected — it orphans the marker verb vocabulary and the tool's identity (the explicit ADR-0011 keep), and leaves nothing stable for skills, templates, and documentation to reference.
- **Two layers only (core plus project, no module vocabularies):** rejected — it forces persona, workflow, and story into either the core (imposing them on every project) or the project layer (making every project re-declare them); the module layer is precisely what makes process choice configurable.
- **TOML as the vocabulary's source of truth:** rejected — the ontology documents already carry definition, documentation, and checkable data in one reviewable, publishable place; a parallel TOML vocabulary would split the one source (REQ-01-01-19-02).
- **External ontology formats (OWL/SKOS files) as the configuration surface:** rejected for now — corpus documents keep the ontology inside the existing lint, publish, and review machinery; semantic serializations remain a candidate *projection* (FR-D1), not the source.

### Consequences

- Implementation starts where the smallest hardwired vocabulary sits: the frontmatter vocabularies (`section-kinds`, `allowed-external-types`) move to configuration with byte-identical defaults; the coupling lint's property bindings move into the story-module profile; the SRC family generalizes into a module provenance contract with on-disk digest verification (FR-A2).
- The extended ONT rules and the reference-target resolution land as `verify` sub-steps, closing the currently invisible dangling-edge error class (FR-A1) for every declared triple, not only body markers (LNT-003).
- `[kinds.<family>]` grows module membership; kind-declared path and id patterns (FR-B2) ride along as a compatible follow-up.
- Migration for this repository is configuration only: its `arqix.toml` declares both shipped profiles, and until the switch every default reproduces today's behaviour.
- Non-goals, deliberately parked: entity-versus-document identity stays with the coming evidence-and-provenance decision, where claims force the question; the claims model and its statement anchors; semantic projections (FR-D1); a declarative query surface.
