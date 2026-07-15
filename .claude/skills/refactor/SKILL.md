---
name: refactor
description: Drive a behaviour-preserving or behaviour-visible refactor of the arqix Rust CLI through the four-phase Assess → Strengthen tests → Refactor → Tidy loop. Use when deduplicating a clone, extracting a shared helper, moving a hardcoded value to configuration, or executing a slice of the refactor program — especially when the target lives under src/checkers/ or src/parser.rs and the oracle-fidelity freeze may apply.
---

# Running a refactor on the arqix CLI

This skill drives one refactor — a single finding or a numbered slice of the refactor program — through the repository's four-phase loop.
It covers the mechanics only; the normative rules for when a refactor is allowed, what it must commit, and in what order live in the `## Refactoring` section of `AGENTS.md` — read that first and let it win over anything here.
The worked rationale and boundaries behind those rules are in `docs/en/processes/refactoring-methodology.md`; read it before touching code.

## Frame the change first

Decide which shape the change is before writing any code, because the two shapes take different paths:

- A **behaviour-preserving** refactor — extracting a helper, deduplicating a clone, renaming — changes structure without changing observable behaviour, and takes the characterization-first path below.
- A **behaviour-visible** change — a new diagnostic, a changed default, a moved contract — is a specification change, and takes the spec-first path below.

If a passing characterization test cannot pin the current behaviour before you start, the change is not ready: either the seam needs a test that does not yet exist, or the change is actually behaviour-visible and belongs on the spec-first path.

## The oracle-fidelity freeze is closed

The freeze that once blocked consolidation inside `src/checkers/` and `src/parser.rs` closed with the checker retirement on 2026-07-15 (task #78): every Python oracle passed conformance, its selftest fixtures are mirrored in the Rust test suite, and the scripts were removed — the Rust engine owns the contracts.

The previously frozen modules now refactor under the same characterization-first rules as everything else, with one obligation carried over: the mirrored oracle-fixture tests (`selftest_cases_match_the_oracle` and siblings in `src/checkers/` and `src/trace.rs`) are the behavioural pin — keep them green through any consolidation, and extend them first when the seam you touch is not pinned.

## Where a new shared helper goes

A helper shared across modules lands in a fresh, neutral `pub(crate)` module named for what it does — `src/markdown.rs` for markdown primitives, `src/util.rs` for path and directory-walk helpers, `src/date.rs` for calendar and ISO-date helpers.
Never host a general-purpose helper in `src/parser.rs` or under `src/checkers/`: those modules own the checker contracts, and a general-purpose helper belongs in a neutral home, not inside a contract owner.

## Behaviour-preserving: characterization tests first

1. **Assess.** Record the target locations, the ADR constraints that bind, and whether the change is gated (most often on the freeze above).
2. **Strengthen tests.** Scaffold characterization tests that pin the current observable behaviour of the seam — the exact outputs, orderings, and edge cases the extraction must preserve — and commit them green (a characterization pin has no red phase).
   Each new test carries an `// arqix:no-requirement` marker unless it verifies a REQ.
3. **Refactor code.** Make the smallest change that resolves the cost, keeping the characterization tests green; re-run them to prove the diff changed nothing observable.
4. **Tidy tests.** Fold scaffolding, remove now-redundant pins, and leave the suite expressing the intended contract rather than the path taken.

## Behaviour-visible: spec-first

1. A requirement or ADR records the new contract first (requirement text follows `docs/en/processes/requirements-style-guide.md`).
2. A test expresses that contract and is committed failing (red); include the red output as evidence.
3. The implementation turns it green.

The red commit precedes the green commit, mirroring the normative order in `AGENTS.md` `## Test-driven implementation`; reviewers check that order.

## Stay inside the closed dependency tree

Do not reach for a crate to remove a few lines.
Per `ADR-0014`, every dependency must clear a high bar, and the recurring example is `walkdir`: the directory walks are pinned to reproduce Python `sorted(dir.rglob('*.md'))` byte for byte — sorted order, no directory-symlink following, the `.tpl.md` exclusion — so a crate would still need manual sorting, skip-directory, and extension filtering wrapped around it and removes almost no code.
Consolidate the duplicated walks into one internal helper; do not adopt a crate to do it.

## Moving a hardcoded value to configuration

When a refactor turns a hardcoded value into configuration, obey the two `ADR-0011` rules: the default configuration reproduces today's value exactly so an unconfigured corpus stays byte-identical, and the value is resolved in exactly one place.
The substance of a check stays in code — the RFC 2119 and EARS keyword contracts, the lifecycle rung sets and their invariants (`ADR-0010`), and the `arqix:` marker prefix — because exposing them as free configuration would hollow the check.
A double-bookkept value that is also mirrored in a Python oracle inherits the freeze: land the one-source move in both at once or wait for the oracle retirement.
`docs/en/plans/refinement-2026-07-09/CONFIG-AUDIT.md` tracks the outstanding configuration rows.

## Close out

Run `python3 scripts/arqix verify` before every commit — it runs the checkers, the marker gate, `cargo test`, and the dogfooded `arqix verify` in one command.
Keep the refactor in focused commits per `AGENTS.md` `## Commits`, and commit the tests before or together with — never after — the code they pin.
