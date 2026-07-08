# Report question catalog

The living catalog behind the question-driven report units (ADR-0008): every human-facing report artefact answers exactly one of these questions.
The catalog is deliberately non-exhaustive — new questions are added here first, and each question graduates into a user story for the report family before it becomes Rust command surface.

| ID | Question | Data source | Unit | Status |
| --- | --- | --- | --- | --- |
| Q-01 | How far along is each user story? | coverage × `derived-from` | [units/story-progress.md](units/story-progress.md) | built |
| Q-02 | Which tests verify which requirements? | `verifies` marker edges | [units/test-to-requirement.md](units/test-to-requirement.md) | built |
| Q-03 | What share of the requirements is verifiably implemented? | coverage summary | [units/scoreboard.md](units/scoreboard.md) | built |
| Q-04 | Which code implements which requirement? | `implements` marker edges | [units/code-to-requirement.md](units/code-to-requirement.md) | built (empty until the Rust phase) |
| Q-05 | Which user story belongs to which integration test? | join test → requirement → story | [units/test-to-story.md](units/test-to-story.md) | built |
| Q-06 | Which workflow belongs to which integration test? | join test → story → workflow | [units/test-to-workflow.md](units/test-to-workflow.md) | built |
| Q-07 | Which ADRs are linked to which requirements? | `guides-implementation-of` edges | [units/adr-to-requirement.md](units/adr-to-requirement.md) | built |
| Q-08 | Where is the documentation for a given piece of code? | needs a code→doc convention | [units/doc-to-code.md](units/doc-to-code.md) | partial — convention is an open design decision |
| Q-09 | How large is the codebase (lines of code, by component)? | external (e.g. tokei) joined against the graph | — | needs external data |
| Q-10 | What is the code coverage of the test suite? | external (e.g. tarpaulin) joined against `verifies` edges | — | needs external data |

Regenerate all built units with:

```text
python3 scripts/arqix_report.py --snapshot "<sha>, <date>"
```
