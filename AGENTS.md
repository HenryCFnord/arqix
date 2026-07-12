# AGENTS.md

## Purpose

This repository uses a lightweight, planning-driven workflow for solo development.

The goal is to keep work:

- structured enough for reliable AI-assisted implementation,
- lightweight enough to stay practical,
- and strict enough to protect `main`.

Agents working in this repository must optimize for:

- clarity,
- small reviewable changes,
- stable mainline history,
- and explicit scope control.

## Core principles

- Keep `main` stable.
- Prefer small, reviewable changes.
- Use the lightest process that still preserves clarity.
- Do not silently expand scope.
- Do not treat arbitrary notes as implementation-ready tasks.
- When AI is involved, make review easier, not harder.

## Canonical workflow

The preferred workflow is:

1. A free-text idea may enter through any clearly scoped instruction source.
2. An intake step may classify the work and create a dedicated planning branch using the standard branch prefixes.
3. Branch-local planning artefacts may be created under `docs/en/plans/<slug>/`.
4. The human reviews and refines the plan on that branch before implementation starts.
5. The coding agent implements from the reviewed planning artefacts, not from the raw idea alone.
6. Non-trivial AI-assisted work should go through a pull request.
7. Documentation and tests are updated when relevant.
8. The human reviewer remains the final decision point before merge.

## Task sources

Preferred task sources are:

- reviewed planning packages under `docs/en/plans/<slug>/`
- approved handoff documents where useful
- scoped GitHub issues
- explicit direct instructions for trivial work

Do not assume that every note, draft, or loose idea is ready for implementation.

If the input is ambiguous, summarize the ambiguity and propose a narrow interpretation before editing.

## Planning-driven implementation

For non-trivial mobile-first work, prefer using a reviewed planning package if one exists.

The standard planning package lives under:

- `docs/en/plans/<slug>/IDEA.md`
- `docs/en/plans/<slug>/PLANS.md`
- `docs/en/plans/<slug>/STATUS.md`

Reviewed `PLANS.md` artefacts are the preferred basis for later implementation work by a coding agent.

During implementation, agents record progress in `STATUS.md` and do not rewrite the reviewed `PLANS.md`; the verification loop `python3 scripts/arqix verify` must pass before every commit.

The initial planning branch and draft planning artefacts may be created during intake before implementation starts.

Handoffs remain supported artefacts where useful, but they are no longer the mandatory first step.

A handoff should define:

- goal
- scope
- out of scope
- context
- constraints
- acceptance criteria
- test expectations
- relevant files
- risks and unknowns

When a handoff exists:

- read it before changing files
- map it to the current branch-local plan when appropriate
- stay within its scope
- explicitly mention important ambiguities
- do not silently broaden the task

## Branching rules

Never work directly on `main`, except for trivial, low-risk edits explicitly suitable for direct push.

Use branch prefixes that match the type of work:

- `feat/` for new functionality
- `fix/` for bug fixes
- `refactor/` for internal restructuring without intended behavior changes
- `docs/` for technical documentation
- `blog/` for blog posts
- `report/` for experiment reports and evaluations
- `chore/` for repository maintenance and low-level upkeep

Branch names should be:

- short
- descriptive
- lowercase
- slug-like

Examples:

- `feat/handoff-parser`
- `fix/yaml-validation`
- `refactor/config-loading`
- `docs/contributing-workflow`
- `blog/why-arqix-had-to-exist`
- `report/agent-tooling-evaluation`

## Process selection

Use the lightest process that still preserves clarity.

### Direct push to `main`

Allowed only for trivial, low-risk changes such as:

- typo fixes
- wording changes
- markdown formatting
- comments without behavior changes
- tiny metadata fixes

Do not directly push to `main` if the change affects:

- code behavior
- tests
- architecture
- CI/CD
- public interfaces
- repository structure in a meaningful way

### Branch without issue or PR

Appropriate for small real changes where:

- the scope is small,
- the intent is obvious,
- no formal review checkpoint is needed,
- and future traceability requirements are low.

Typical examples:

- small docs updates
- blog posts
- experiment reports
- minor low-risk cleanup

### Issue plus branch

Use an issue when the work benefits from:

- explicit scope tracking,
- acceptance criteria,
- contextual references,
- or likely follow-up work.

Typical examples:

- medium-sized tasks
- structured imports or normalization work
- tasks derived from reviewed plans or handoffs
- non-trivial fixes or improvements

### Issue plus branch plus PR

Preferred for:

- features
- non-trivial bug fixes
- meaningful refactorings
- architecture changes
- public behavior changes
- work derived from reviewed planning packages or handoffs
- AI-assisted implementation
- AI-assisted semantic restructuring
- any change that deserves an explicit review checkpoint

## Content versus implementation

Treat content artefacts differently from implementation work.

### Content artefacts

Blog posts, reports, and standalone documentation pieces are often the work item itself.

Typical flow:

- branch only
- optional PR if the change is large, structural, or AI-assisted

Examples:

- blog article
- experiment report
- workflow documentation

### Implementation work

Features, bug fixes, refactorings, architectural changes, and structured requirements work typically need:

- scoped task context, ideally from reviewed `PLANS.md`
- dedicated branch
- PR for meaningful AI-assisted work

## Planning behavior

For non-trivial tasks:

- inspect relevant files first
- understand the local conventions
- propose a short plan before large edits
- follow the reviewed planning artefacts when they exist
- call out assumptions
- identify likely risks
- keep the plan aligned with the stated scope

Do not start broad rewrites when a local change is sufficient.

Prefer evolutionary changes over premature abstraction.

## Editing behavior

When editing:

- preserve existing style unless there is a good reason to improve it
- avoid unrelated cleanup
- keep changes focused
- avoid hidden side effects
- maintain readability
- favor explicitness over cleverness

Do not mix unrelated changes into the same task unless explicitly asked.

When implementing stories, work on one story at a time and keep opportunistic refactors out of the change.

## Testing

For behavioral changes:

- add or update tests
- prefer targeted tests close to the changed behavior
- keep tests readable
- mention important untested edge cases explicitly

If a task changes behavior and no tests are added, explain why.

Do not claim that tests passed unless they were actually run.

## Test-driven implementation

Rust implementation work on the arqix CLI is test-driven.
The command skeleton under `tests/cli_*.rs` mirrors the command-ownership table in arc42 chapter 5; tests for unimplemented stories are `#[ignore]`d with the owning story ID as the reason.

Implementing a story follows this order, and the order is normative:

1. Remove the `#[ignore]` attributes of the story's tests (and refine the test bodies and fixtures where the skeleton was only a contract sketch).
2. Run the tests and show they fail (red).
   Include the red output in the PR description as evidence.
3. Implement until the tests pass (green), then refactor.
4. Commit the test changes before or together with — never after — the implementation they drive.
   Reviewers check the commit order.

Additional rules:

- Every test that verifies a requirement carries an `// arqix:verifies REQ-…` marker comment directly above the test function; a test that deliberately verifies no requirement (an implementation-detail or oracle-conformance pin) carries `// arqix:no-requirement` instead — exactly one of the two, on every test function including unit tests under `src/`. `scripts/check_trace_markers.py` validates markers against the requirements corpus and must pass before every commit.
- Every ignored test must name its owning story in the ignore reason (`#[ignore = "US-XX-YY-ZZ: not implemented"]`).
- Tests must be deterministic: no wall clock (dates are injected, see ADR-0004), no network, no dependence on test execution order.
  Mutating commands run on `scratch_copy` fixtures, never on the shared fixture.
- Do not delete or weaken a skeleton test to get to green; if a contract turns out wrong, change the requirement first and reference the change in the PR.

Conformance runs: the test helpers honour an `ARQIX_BIN` override, so the same skeleton tests double as the conformance suite for the Python oracle (arc42 chapter 8).
Command families implemented in Python are exercised with

ARQIX_BIN=$PWD/scripts/arqix cargo test --test cli_trace -- --ignored

The tests stay `#[ignore]`d — the default `cargo test` run measures the Rust implementation only.
A story counts as ported when its suite is green without the override.

## Documentation

Update documentation when:

- behavior changes
- interfaces change
- workflows change
- examples become outdated
- new constraints or caveats become relevant

Keep documentation concise, practical, and aligned with actual behavior.

Markdown follows the markdownlint rule set configured in `.markdownlint.jsonc` plus two project conventions: write one sentence per line (never wrap a sentence for column width, never put two sentences on one line; separate paragraphs with a blank line), and place any `<!-- arqix:… -->` marker or directive on its own line directly above the block it annotates.
The full rules and rationale are in `docs/en/processes/markdown-style-guide.md`; run `npx markdownlint-cli2` on touched Markdown.

## Commits

Make focused commits.

Avoid mixing:

- implementation changes
- large documentation rewrites
- formatting-only cleanup
- unrelated refactors

A good commit should make sense when read in isolation.

## Pull requests

When a PR is expected:

- summarize the change clearly
- state why it was made
- call out risks and open questions
- mention validation performed
- identify files or areas needing attention

Optimize for reviewability.

## GitHub and review context

When working from an issue, reviewed plan, handoff, or PR context:

- preserve references where possible
- keep branch names aligned with the task type
- keep acceptance criteria visible in the final result
- make it easy for the reviewer to compare intent and implementation

## Safety and escalation rules

Do not:

- push directly to `main` for non-trivial work
- tag or publish a release (git tag, GitHub release, crates.io publish) without explicit approval from the repository owner
- invent requirements not present in the task
- silently introduce new runtime dependencies
- change public behavior unless explicitly required
- rewrite broad areas of the repository without clear justification

When uncertain:

- pause
- summarize the uncertainty
- propose a narrow next step
- prefer a plan over a guess

## Decision defaults

When in doubt:

- prefer a branch over direct work on `main`
- prefer a PR over a silent merge
- prefer a small scoped change over a broad rewrite
- prefer explicit review notes over implicit assumptions

## Repository-specific note

Intake and planning may be orchestrated by any tool or arrive as direct instructions, a coding agent implements inside the repository, the human reviews and approves, and GitHub remains the source of truth.

The current project name is `arqix`.

Do not use the obsolete project name `darcy`.
