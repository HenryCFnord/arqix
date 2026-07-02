# Requirements Style Guide

This guide defines the normative authoring rules for requirement documents under `docs/en/architecture/req/`. It combines a strict subset of RFC 2119 keywords with the EARS sentence patterns so that every requirement is deterministic to parse, classify, and verify.

`scripts/check_requirements.py` enforces these rules mechanically. This guide is the human-readable contract behind that checker and the later arqix implementation.

## References

- RFC 2119: Key words for use in RFCs to Indicate Requirement Levels — <https://datatracker.ietf.org/doc/html/rfc2119>
- EARS: Easy Approach to Requirements Syntax — <https://alistairmavin.com/ears/>

## Requirement kinds

Every requirement document declares exactly one of three kinds via its `rdf.type`:

| Kind | `rdf.type` | Verification |
| --- | --- | --- |
| Functional requirement | `arqix:classes/functional-requirement` | Directly testable; tests link to it via `verifies-requirement`. |
| Quality requirement | `arqix:classes/quality-requirement` | Verified through acceptance criteria and review, not a directly linked test. |
| Constraint | `arqix:classes/constraint` | Not directly testable; frames the solution space for other requirements. |

## Normative vocabulary (RFC 2119 subset)

Only the following keywords are allowed in the normative requirement sentence, always uppercase:

- `SHALL` / `SHALL NOT` — mandatory behavior
- `SHOULD` / `SHOULD NOT` — recommended behavior; deviations need justification
- `MAY` — optional behavior

Rules:

- Exactly one primary keyword per requirement sentence.
- `MUST`, `MUST NOT`, `REQUIRED`, `RECOMMENDED`, `NOT RECOMMENDED`, and `OPTIONAL` are forbidden; use the `SHALL`/`SHOULD`/`MAY` forms instead.
- Lowercase `shall`, `should`, and `may` are forbidden inside the normative sentence.

## Sentence patterns (EARS)

The `## Requirement` section contains exactly one normative sentence matching one of these patterns:

| Pattern | Canonical form |
| --- | --- |
| Ubiquitous | `The <system> SHALL <response>.` |
| Event-driven | `When <trigger>, the <system> SHALL <response>.` |
| State-driven | `While <state>, the <system> SHALL <response>.` |
| Unwanted behaviour | `If <condition>, then the <system> SHALL <response>.` |
| Optional feature | `Where <feature>, the <system> SHALL <response>.` |
| Complex | Combinations of `While`/`When`/`If … then` clauses before the `the <system> SHALL <response>.` core. |

Rules:

- The sentence ends with a period and contains the core clause `the <system> <KEYWORD> <response>`.
- `<system>` names the acting system or component (for arqix typically `arqix`, a command, or a subsystem such as `the linter`).
- Clause keywords `When`, `While`, `If`/`then`, and `Where` are capitalized exactly as shown at the start of their clause.
- Explanatory context, scope notes, and rationale belong in the `### Notes` section, never in the normative sentence.

## Kind ↔ keyword matrix

| Kind | Expected keywords |
| --- | --- |
| Functional requirement | `SHALL`, `SHALL NOT` |
| Quality requirement | `SHOULD`, `SHOULD NOT` (use `SHALL` only where the quality is CI-gated) |
| Constraint | `SHALL`, `SHALL NOT`, preferably in the ubiquitous pattern |
| Optional behavior (any kind) | `MAY` |

The checker reports matrix violations as warnings, not errors: the matrix is guidance for authors, while the vocabulary and pattern rules above are hard requirements.

## Examples

Functional (event-driven):

> When `arqix doc new <kind>` is invoked without `--id`, arqix SHALL generate a unique document ID from the configured policy.

Quality (ubiquitous):

> The lint diagnostics SHOULD be actionable enough to resolve a finding without reading arqix source code.

Constraint (ubiquitous):

> The arqix CLI SHALL NOT write outside the declared change scope.

Unwanted behaviour:

> If an include target does not exist, then the assembler SHALL fail with a non-zero exit code.
