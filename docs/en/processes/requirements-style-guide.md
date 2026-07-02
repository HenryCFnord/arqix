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

## Subject conventions

The `<system>` subject of the core clause follows these rules:

- Tool behaviour, ubiquitous pattern: the subject is exactly `The arqix CLI`.
  - `The arqix CLI SHALL NOT overwrite existing files without explicit approval.`
- Tool behaviour, triggered patterns (event, state, unwanted, optional): name the command in backticks in the trigger clause and use the bare `arqix` as the core-clause subject.
  - `When \`arqix fmt\` runs, arqix SHALL sort frontmatter keys according to the configured \`key_order\`.`
- A backticked command may itself be the subject when the requirement binds exactly that command.
  - `Where warn-only mode is configured, \`arqix policy check\` SHALL report violations without failing.`
- Artefact and output subjects are allowed when the requirement constrains the artefact itself rather than tool behaviour (documents, reports, diagnostics, processes, coding agents).
  - `The agent instruction document SHALL define scope rules for story-by-story execution.`
- Invented subsystem nouns are forbidden — do not write `the arqix formatter`, `the linter`, or `the exporter`; use `The arqix CLI` or the command-trigger form instead. The checker reports arqix-containing subjects outside the allowed forms (`EARS-006`, warning).

## Atomicity

One requirement binds exactly one verifiable contract: one behaviour or one artefact, one trigger, one verification method.

- Enumerations inside the normative sentence are allowed when they completely specify that single contract — for example the result set of one command invocation, or the mandatory content of one document.
- When the sentence enumerates, the `fit-criterion` MUST make every enumerated item independently checkable: either by itemising them ("Each violation class (missing, extra, type-invalid) produces a distinct finding.") or by a set quantifier ("produces exactly the standard scaffold").
- Split into separate requirements when the enumerated items are independently implementable features, could ship separately, or need different verification methods.
- Partial-failure localisation lives in the trace model, not in the requirement count: `verifies-requirement` is many-to-one, so several test cases may verify one requirement — one per enumerated aspect. A requirement counts as satisfied only when all linked verification evidence passes; the failing test identifies the missing aspect.
- Atomicity is a review criterion, not a checker rule: enumeration detection over natural sentences produces too many false positives ("identical inputs and configuration" is one condition, not two features) to be enforced mechanically.

Canonical owner: every behaviour is specified exactly once. When several stories demand the same behaviour, the requirement is owned by the story with the lowest ID that demands it (the requirement ID stays `REQ-<owner>-NN`, and the owner is the first `derived-from` object); all further demanding stories are added to `derived-from` and link the requirement in their `has-requirement` instead of duplicating it. The `00-00-00` domain remains reserved for system-wide contracts, not for shared feature behaviour.

Good — one contract, itemisable fit criterion:

> The `unit new` command help SHOULD explain where units are created, which metadata is optional, and how IDs are supplied.

Bad — two independent features hiding in one sentence (split them):

> The arqix CLI SHALL export trace matrices as CSV and serve documentation over MCP.

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
