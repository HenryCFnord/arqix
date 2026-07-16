# Refactoring Methodology

This guide explains how refactoring is carried out in this repository and why the safeguards around it exist.
It is a descriptive companion to the process contract, not a source of process authority.

The normative rules — when a refactor is allowed, what it must commit, and in what order — live in the `## Refactoring` section of `AGENTS.md` and only there.
Where this guide and AGENTS.md appear to say the same thing, AGENTS.md governs; this document exists to give the rationale, the worked shape, and the boundaries that a rule sentence cannot carry.
It is modelled on the sibling [requirements style guide](requirements-style-guide.md) and [markdown style guide](markdown-style-guide.md): a human-readable contract behind a set of mechanically or reviewer-enforced rules.

## The loop

Refactoring in this repository runs a four-phase loop, and the order matters:

1. **Assess.** Identify the maintainability or testability cost — duplication, a hardcoded value that a repository owner could legitimately want different, an untested seam — and classify the change as either behaviour-preserving or behaviour-visible.
   The assessment records the target locations, the ADR constraints that bind, and whether the change is gated on anything.
2. **Strengthen tests.** Before changing production code, raise the test coverage around the seam so that the current behaviour is pinned.
   For a behaviour-preserving change this means characterization tests; for a behaviour-visible change it means a spec and a failing test that expresses the new contract.
3. **Refactor code.** Make the smallest change that resolves the assessed cost, keeping the strengthened tests green (for a behaviour-preserving change) or turning the new failing test green (for a behaviour-visible change).
4. **Tidy tests.** Once the refactor lands, fold any scaffolding, remove now-redundant pins, and leave the suite expressing the intended contract rather than the path taken to reach it.

The loop is deliberately test-first in both of its shapes, mirroring the red-before-green order that `AGENTS.md` already requires for story implementation.

## Behaviour-preserving refactors: characterization tests first

A pure internal refactor — extracting a helper, deduplicating a clone, renaming for clarity — changes structure without changing observable behaviour.
The safeguard is that it commits characterization tests pinning the current behaviour *before* the extraction, and those tests stay green across the change.
This is the same discipline as the red-before-green rule for new behaviour: the test is written and committed first, so the diff that follows is provably behaviour-preserving rather than merely believed to be.

A refactor that cannot be covered by a passing characterization test before it starts is not ready to start.
Either the seam needs a test that does not yet exist, or the change is not actually behaviour-preserving and belongs in the spec-first path below.

## Behaviour-visible change is spec-first

When a refactor changes what a user or a downstream tool can observe — a new diagnostic, a changed default, a moved contract — it stops being a refactor in the safe sense and becomes a specification change.
Those follow the spec-first order:

1. A requirement or an ADR records the new contract first.
2. A test expresses that contract and is committed failing (red).
3. The implementation turns it green.

The red commit precedes the green commit, and reviewers check that order, exactly as they do for story work.
Requirement text follows the [requirements style guide](requirements-style-guide.md); an architectural decision follows the ADR conventions under `docs/en/architecture/adr/`.

## Contract-owning modules

`src/checkers/`, `src/parser.rs`, and `src/trace.rs` own the checker, parsing, and trace contracts.
Their reference fixtures are mirrored case by case in their test modules (`selftest_cases_match_the_oracle` and its siblings); those pins are the executable specification of each contract.
Consolidation inside these modules follows the normal characterization-first rules of this methodology: extend the pin before touching an unpinned seam, keep it green through the change, and never weaken it to make a consolidation fit.
The provenance of each port lives in the refactor-program plan package and git history, not here.

## Where shared helpers live

When a refactor extracts a helper shared across modules, the helper goes into a fresh, neutral `pub(crate)` module named for what it does:

- Markdown primitives — heading detection, fence-aware scanning — belong in `src/markdown.rs`.
- Path and directory-walk helpers belong in `src/util.rs`.
- Calendar and ISO-date helpers belong in `src/date.rs`.

Shared non-oracle helpers never land in `src/parser.rs` or under `src/checkers/`.
Those modules are the oracle-mirrored surface; hosting general-purpose helpers there would pull neutral code across the fidelity boundary and back under the freeze.
A neutral home keeps the helper discoverable, keeps the mirrored modules minimal, and keeps the boundary between "faithful port" and "shared library" legible.

## Dependencies stay inside the closed tree

arqix keeps its dependency tree deliberately small, as recorded in [ADR-0014](../architecture/adr/ADR-0014-mcp-transport.md): every crate must clear a high bar of individual justification, and a crate is rejected unless it earns its place.

A refactor does not reach for a crate to remove a few lines.
The recurring example is `walkdir`: the repository's directory walks are pinned to reproduce Python `sorted(dir.rglob('*.md'))` byte for byte — sorted order, no directory-symlink following, the `.tpl.md` exclusion — so a crate would still need manual sorting, skip-directory, and extension filtering wrapped around it to match.
It would remove almost no code while adding a supply-chain entry and a silent ordering-or-symlink drift risk, so it does not clear the bar.
Consolidating the duplicated walks into one internal helper is the right move; adopting a crate to do it is not.

## Moving hardcoded values to configuration

Some refactors turn a hardcoded value into configuration because a repository owner could legitimately want it different — layout, naming, an id shape.
[ADR-0011](../architecture/adr/ADR-0011-configuration-boundary.md) draws the boundary and sets two rules that any such move obeys:

- **Defaults preserve present behaviour.** The default configuration reproduces today's hardcoded value exactly, so an unconfigured corpus stays byte-identical.
- **One source.** The value is resolved in exactly one place; a value maintained in more than one place today is the primary trigger for consolidating it, and leaving a second consumer reading its own copy recreates the drift the move was meant to remove.

The boundary is not "make everything configurable."
The substance of a check stays in code: the RFC 2119 and EARS keyword contracts, the lifecycle rung sets and their invariants ([ADR-0010](../architecture/adr/ADR-0010-lifecycle-vocabularies.md)), and the `arqix:` marker prefix that anchors the ontology vocabulary all stay hardcoded, because exposing them as free configuration would hollow the check.
The id *policy* model ([ADR-0012](../architecture/adr/ADR-0012-id-policy-model.md)) shows the line in practice: the id shape is configurable, while the identity scheme it belongs to is not.
When a hardcoded value is also double-bookkept between the Rust engine and a Python oracle, the one-source move must land in both simultaneously or wait for the oracle retirement, so it inherits the freeze constraint above.

## References

- `AGENTS.md` — the canonical process contract; its `## Refactoring` section holds the normative rules this guide explains.
- [ADR-0001 — Agent Instruction Document Layout](../architecture/adr/ADR-0001-agent-instruction-document-layout.md) — CLAUDE.md is a thin adapter, AGENTS.md is canonical, skills are non-normative.
- [ADR-0010 — Lifecycle Vocabularies](../architecture/adr/ADR-0010-lifecycle-vocabularies.md) — the rung sets and done-claim invariant that stay in code.
- [ADR-0011 — Configuration Boundary](../architecture/adr/ADR-0011-configuration-boundary.md) — convention versus configuration, defaults-preserve, and one-source.
- [ADR-0012 — ID Policy Model](../architecture/adr/ADR-0012-id-policy-model.md) — the configurable id shape versus the fixed identity scheme.
- [ADR-0014 — MCP Transport](../architecture/adr/ADR-0014-mcp-transport.md) — the deliberately small, individually justified dependency tree.
- [Requirements style guide](requirements-style-guide.md) — how the spec-first requirement is authored.
- [Markdown style guide](markdown-style-guide.md) — one sentence per line and the markdownlint rule set this document follows.
