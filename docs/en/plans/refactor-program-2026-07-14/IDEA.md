---
title: "Pre-0.2.0 refactor program intake"
date: 2026-07-14
status: draft
category: docs
branch: claude/refactor-prep
---

# Intake

Free-text intake from the owner for the pre-0.2.0 maintainability pass, condensed:

- Before 0.2.0, run a deliberate refactor program over the Rust core rather than more ad-hoc cleanups.
- Do it as a disciplined loop, not a big-bang rewrite: ASSESS the code, STRENGTHEN the tests that pin the affected behaviour, REFACTOR the code, then TIDY the tests.
- The loop is behaviour-preserving by construction: a pure internal refactor gets characterization tests committed *before* the extraction, mirroring the repository's existing red-before-green rule, so the change is provably behaviour-preserving.
- Anything that changes observable behaviour stays spec-first: a REQ or ADR precedes the test, then the red commit, then the green commit.

Look across four audit dimensions:

- Crate audit: is any hand-rolled logic a place where the closed dependency tree (ADR-0014) should reconsider a crate, or conversely where a crate would earn nothing?
  Concrete example: ISO calendar-date validation (leap year plus days-in-month) is implemented by hand more than once — is that a shared helper, a crate, or fine as-is?
- Hardcoded to configuration: where are conventions baked into code that a repository owner could legitimately want different (ADR-0011)?
  This includes generalizing the ontology so the `docs/ontology/` corpus is the single runtime vocabulary source, and treating the full IVVQ verification-method set — inspection, analysis, demonstration, test — as a first-class vocabulary rather than an implicit idea.
- Deduplication into a shared library: the same primitive re-implemented across modules (markdown scanning, path normalization, directory walking, frontmatter splitting, the small checker helpers) — pull the genuinely-identical copies into shared homes.
- Test placement: are tests in the right tree (inline `#[cfg(test)]` for pure unit logic versus `tests/` for repo-level and command-contract assertions), and are they hygienic (unique temp dirs, no cross-run contamination)?

Framing constraints the owner set going in:

- Respect the oracle-fidelity freeze: while the Python checker scripts are still the active behavioural oracle, do not consolidate inside the checker modules or into the oracle-mirrored parser in a way that could perturb conformance.
- Keep the closed dependency tree honest: a crate has to clear a high bar (ADR-0014); rejecting a crate is a legitimate, recordable outcome.
- Any convention that moves to configuration keeps its present behaviour as the default and lives in exactly one source (ADR-0011).
- No correctness fishing expedition: the goal is maintainability and testability, and the program should say plainly where a finding turned out to be low value.
