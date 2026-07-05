# Trace report snapshots

Manually generated snapshots of the human-facing trace projections, taken
so the outputs can be reviewed exactly as the tool produces them. Snapshot
commit `a6a277b`, 2026-07-05, produced by the Python oracle
(`scripts/arqix`, ADR-0006).

| File | Layer (ADR-0006) | Regenerate with |
| --- | --- | --- |
| `coverage.md` | 2 — diagnostics projection | `python3 scripts/arqix trace coverage > docs/en/reports/trace/coverage.md` (plus header block) |
| `matrix.csv` | 3 — audit product | `python3 scripts/arqix trace matrix > docs/en/reports/trace/matrix.csv` |

These files go stale with every change to requirements, tests, or markers;
they are refreshed manually until a CI workflow regenerates them (arc42
chapter 11: "implement with the first CI workflow PR"). Treat the committed
copies as review artefacts, not as a live source of truth — the live
answer is always the command.

## Third human view: `trace check`

The per-requirement report is interactive rather than a file; verbatim
output for the one requirement currently verified by a live test:

```
$ python3 scripts/arqix trace check REQ-00-00-00-02
REQ-00-00-00-02: verifies: tests/cli.rs:30
REQ-00-00-00-02: implements: none
```

## How to read `coverage.md`

- Diagnostic lines come first: `TRC-COV-001` (error) = uncovered functional
  requirement, `TRC-COV-002` (warning) = only planned (all verifies markers
  sit on `#[ignore]`d tests), `TRC-KIND-001` (warning) = no declared kind.
- The table lists every requirement with its verified/planned/implemented
  locations; `—` means none.
- The per-kind summary at the bottom is the project's progress gauge:
  requirements move uncovered → planned (red skeleton) → verified
  (story implemented test-first).
