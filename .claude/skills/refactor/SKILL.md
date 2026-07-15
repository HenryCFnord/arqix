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

## Check the oracle-fidelity freeze before anything else

This is the gating question for every arqix refactor.
`scripts/check_frontmatter.py` and `scripts/check_requirements.py` are the active behavioural oracle, and the Rust checkers under `src/checkers/` and the reading layer in `src/parser.rs` are deliberately faithful ports held to identical findings on identical inputs.

Before touching a target, answer:

- Does the target live under `src/checkers/` or in `src/parser.rs`?
  If yes, it is under the freeze — consolidation is blocked even when a clone is byte-identical and the merge looks trivial.
- Are the Python checkers still the active oracle, or has the checker-retirement work (task #78) landed?
  While the oracle is active, a freeze-covered consolidation either rides task #78 or lands in both the Rust and Python sources at once from a single origin — never Rust-only.
- Is the target genuinely oracle-neutral — internal to `assembler.rs`, `publisher.rs`, `rewriter.rs`, or a similar non-mirrored module?
  Those are not covered by the freeze and can be consolidated now.

When in doubt, treat the target as frozen and defer it rather than risk silent engine-versus-oracle drift during the grace period.

## Where a new shared helper goes

A helper shared across modules lands in a fresh, neutral `pub(crate)` module named for what it does — `src/markdown.rs` for markdown primitives, `src/util.rs` for path and directory-walk helpers, `src/date.rs` for calendar and ISO-date helpers.
Never host a general-purpose helper in `src/parser.rs` or under `src/checkers/`: those are the oracle-mirrored surface, and putting neutral code there pulls it back under the freeze.

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
