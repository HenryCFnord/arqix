# Trace audit matrices

Machine/audit artefacts per ADR-0006 layer 3 — stable CSV snapshots of
the traceability matrices, commit `c352e4c`, 2026-07-05. Human-readable
answers live in the [question units](../units/) (ADR-0008); the live
answer is always the command.

| File | Content | Regenerate with |
| --- | --- | --- |
| `matrix.csv` | requirement ↔ marker matrix (verified/planned/implements) | `python3 scripts/arqix trace matrix > docs/en/reports/trace/matrix.csv` |
| `matrix-us-req.csv` | story ↔ requirement matrix from the ontology (`derived-from`, 482 pairs) | `python3 scripts/arqix trace matrix --type us-req > docs/en/reports/trace/matrix-us-req.csv` |
