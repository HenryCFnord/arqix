# Agent instructions

This repository is an arqix-governed documentation corpus.
This file is the normative process document for coding agents: replace the starter rules below with your repository's own, and keep them here — agent-specific extension points stay thin and carry no process rules.

## The verification loop

Run `arqix verify` before every commit and treat its findings as gating.
The loop runs the configured sub-steps (format, lint, trace-scan, coverage, ratchet); see `arqix.toml` for the policy.

## Corpus entry points

- `arqix doc list` — the document catalog (id, title, kind, file).
- `arqix doc read <id>` — one document by id.
- `arqix doc search <query>` — full-text search over the corpus.
- `arqix doc new <kind> --title "..."` — create a document from the kind's template; never create corpus documents by hand.
- `arqix fmt` — the only mechanical mutator of existing documents; run it instead of hand-formatting.
- `arqix mcp serve` — the same catalog as MCP tools (`search`, `read`, `list`) over stdio, for agent frameworks.

## Traceability

Requirements live in the corpus; tests claim them with a comment marker on the line above the test: `arqix:verifies <REQUIREMENT-ID>`.
`arqix trace coverage` shows what is verified; the ratchet in `arqix verify` fails a change that silently un-verifies a requirement.

## Starter rules

- Work one change at a time and keep the diff within the declared scope.
- Never edit generated artefacts (`pages/`, staged sites, report snapshots); regenerate them.
- When a check fails, fix the cause — never the checker.
