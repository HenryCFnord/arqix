---
name: test-audit
description: Audit and tidy the arqix Rust test suite — coverage of pure helpers, unit-versus-integration placement, filesystem hygiene, and trace-marker correctness. Use when strengthening tests before a refactor extraction or tidying tests after it, or when running the test-baseline-hygiene pass of the refactor program.
---

# Auditing the arqix test suite

arqix is test-driven: every behavioural change ships tests, and every test carries exactly one trace marker.
This skill covers the mechanics of auditing and tidying those tests; the repository's normative rules — the red-before-green order, the marker contract, determinism, and commit discipline — live in its `AGENTS.md` (see "## Testing", "## Test-driven implementation", "## Commits"), and `AGENTS.md` wins over anything here.

## When this runs

This skill drives the two test-facing phases of the refactor loop, never the code-changing one.

- Strengthen tests: before a refactor, add the characterization tests that pin current behaviour so the extraction is provably behaviour-preserving — a characterization pin asserts today's output and so lands green with no red phase, committed before the code moves.
- Tidy tests: after the refactor, remove redundancy the extraction made obsolete and re-home any test that now sits in the wrong layer, without changing what is asserted.

Keep the two apart from the code edit: strengthening lands first, tidying last, and neither is mixed into the extraction commit.

## Coverage of pure helpers

- A helper that moves into a fresh neutral module (`src/markdown.rs`, `src/util.rs`, `src/date.rs`) earns direct unit tests at its new home; a helper only reachable through an integration test today is under-pinned for an extraction.
- Characterization tests assert the current output for the inputs that distinguish this helper from a naive rewrite — the edge cases the assessment flags (fence toggling, exotic line separators in `py_splitlines` index space, leap-year and days-in-month boundaries, POSIX path normalization) — not just the happy path.
- Coverage inside `src/checkers/` and `src/parser.rs` is open since the checker retirement (task #78, 2026-07-15) closed the oracle-fidelity freeze; the mirrored oracle-fixture tests (`selftest_cases_match_the_oracle` and siblings) are the behavioural pin to extend.

## Placement: unit versus integration

- Pure, module-local logic is unit-tested inline under `#[cfg(test)]` in the module that owns it; a `tests/*.rs` integration test that only exercises one pure helper belongs inline instead.
- Command and end-to-end behaviour stays in the `tests/cli_*.rs` skeleton that mirrors the arc42 command-ownership table; do not smear CLI contracts into unit tests.
- When an extraction gives a helper a new home, its characterization tests move with it — the test's layer should match where the code now lives.

## Filesystem hygiene

- Every test that writes to disk uses its own unique temp directory; no two tests share a fixed path, and nothing writes under a hardcoded `/tmp` or repo-relative scratch name that can collide across a parallel `cargo test` run.
- Mutating commands run on `scratch_copy` fixtures, never on the shared read-only fixture.
- Temp state is cleaned up (or scoped to an auto-removed guard) so a failed run leaves no residue that taints the next.

## Marker correctness

- Every test function — inline unit tests under `src/` included — carries exactly one marker directly above it: `// arqix:verifies REQ-…` when it proves a requirement, or `// arqix:no-requirement` for an implementation-detail or oracle-conformance pin.
- A behaviour-preserving characterization test that pins internal behaviour with no requirement of its own is `// arqix:no-requirement`; a test driven by a new REQ or ADR (a spec-first, behaviour-visible change) is `// arqix:verifies`.
- `arqix trace markers` validates the markers and must pass; run it, `cargo test`, and `python3 scripts/arqix verify` before the phase's commit.

## Determinism

Tests stay deterministic per `AGENTS.md`: no wall clock (dates are injected, see ADR-0004), no network, and no dependence on execution order.
An audit that surfaces a flaky or order-dependent test reports it; it does not paper over it by pinning the incidental order.
