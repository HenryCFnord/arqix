# Reports

Human-facing reporting follows ADR-0008: every artefact answers exactly one named question from the [question catalog](QUESTIONS.md), and a report is an assembly of such units.
Three layers live here:

| Layer | What | Where |
| --- | --- | --- |
| Question units (human) | One generated Markdown file per question — story progress, scoreboard, test traceability, ADR links, code views | [`units/`](units/) |
| Audit products (machine/audit) | Stable CSV matrices per ADR-0006 layer 3 | [`trace/`](trace/) |
| Live answers | The commands themselves — always current, never stale | `python3 scripts/arqix trace …` |

## Units

- [How far along is each user story?](units/story-progress.md) (Q-01)
- [What share of the requirements is verifiably implemented?](units/scoreboard.md) (Q-03)
- [Which tests verify which requirements?](units/test-to-requirement.md) (Q-02)
- [Which user story belongs to which integration test?](units/test-to-story.md) (Q-05)
- [Which workflow belongs to which integration test?](units/test-to-workflow.md) (Q-06)
- [Which ADRs are linked to which requirements?](units/adr-to-requirement.md) (Q-07)
- [Which code implements which requirement?](units/code-to-requirement.md) (Q-04)
- [Where is the documentation for a given piece of code?](units/doc-to-code.md) (Q-08)

## Regeneration and staleness

All committed files here are snapshots (each carries its commit + date in a generated header) and go stale with every change to requirements, tests, or markers.
Refresh with:

```text
arqix report snapshot --stamp "<sha>, <date>"
arqix trace matrix > docs/en/reports/trace/matrix.csv
arqix trace matrix --type us-req > docs/en/reports/trace/matrix-us-req.csv
```

Regeneration stays manual (`just reports`), but staleness is gated: `arqix report snapshot --check` runs inside `arqix verify` (and therefore in CI) and fails when any committed snapshot no longer matches the corpus.
The live answer is always the command.
