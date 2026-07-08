---
id: ADR-0003
title: Verification Orchestrator
slug: verification-orchestrator
iri: arqix:adrs/adr-0003

rdf:
  type:
    - arqix:classes/adr

triples:
  - predicate: arqix:properties/guides-design-of
    object:
  - predicate: arqix:properties/guides-implementation-of
    object:
      - arqix:requirements/req-04-01-05-01
      - arqix:requirements/req-04-01-05-02
      - arqix:requirements/req-04-01-05-03
      - arqix:requirements/req-04-01-05-04
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

## Verification Orchestrator

### Context

`arqix verify` is the one-command quality loop that agents and CI depend on (US-04-01-05, US-08-01-13).
Its requirement cluster carries real domain logic: configured sub-step selection over format, lint, trace scan, and coverage (REQ-04-01-05-01), fail-fast versus aggregate result modes (REQ-04-01-05-02), a JSON mode with per-step results and diagnostic references (REQ-04-01-05-03), and the constraint that rendering is never part of the default loop (REQ-04-01-05-04).
The C4 model needs an owner for this cluster; until now it lived implicitly in the CLI entrypoint's routing.

### Decision

`verify` becomes its own component, the **Verification Orchestrator**, with two guardrails:

1. It invokes sub-steps exclusively through the same stable command interface the CLI entrypoint uses.
   It sequences and aggregates; it never implements a check itself.
2. It is treated as the fifth spine component (alongside entrypoint, config resolver, document parser, and diagnostics).
   Its edges to the sub-step components are orchestration edges at the command-API level and do not count against the low lateral coupling between feature components.

Policy checking is not a default sub-step (REQ-04-01-05-01 enumerates format, lint, trace scan, coverage), but the orchestrator is the designated place to add `policy check` as an optional configured sub-step later, letting CI gates collapse into one command.

### Alternatives Considered

- **Inside the CLI entrypoint (status quo):** rejected — the composition root must stay thin; fail-fast/aggregate semantics and the per-step result model are testable domain logic, and the requirement cluster would have no component to trace to.
- **Inside the Policy Checker:** rejected — the two answer different questions with different inputs.
  Policy checking judges *changes* (a changed-file list, process governance, optional and warn-only-first per REQ-01-01-07-03); verify judges repository *state* (the whole corpus, quality).
  Merging them would let an optional component swallow the mandatory core loop, and the component would orchestrate every gate except itself.
  The valid kernel of this option — both are gates — is honoured by the planned optional policy sub-step instead.
- **Inside Diagnostics & Exit Codes:** rejected — diagnostics is a passive contract layer that everything reports *to*; giving it invocation power inverts the dependency direction and creates cycles.
- **No component, external composition (Taskfile/CI):** rejected — REQ-04-01-05-01 requires arqix itself to execute the loop; taskfiles cannot guarantee the fail-fast/aggregate/JSON contract and would differ per repository.
  Taskfiles remain a convenience wrapper around `arqix verify`.

### Consequences

- The REQ-04-01-05 cluster has a home in the building block view; the entrypoint stays a thin router.
- The verify semantics are testable in isolation against stubbed sub-step results.
- Extending the loop (i18n profile, policy check) is configuration plus one orchestration edge — the component cut is stable.
- The boundary "sequencer only, never a checker" must be defended in review; an orchestrator that grows checks becomes a god component.
