---
id: unit-icd-03
title: Diagnostics
slug: diagnostics
iri: arqix:units/unit-icd-03

rdf:
  type:
    - arqix:classes/unit

triples: []

properties:
  section-kind: icd-diagnostics

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-06
  updated: 2026-07-06
  lang: en
  translation-of:
  generated: false
---

## Diagnostics

<!-- arqix:references-artefact arqix:adrs/adr-0006 -->
<!-- arqix:references-artefact arqix:requirements/req-00-00-00-03 -->
Every command that reports findings emits one shared JSON shape (REQ-00-00-00-03, ADR-0006 layer 2), so a consumer parses one format across the whole tool.
The `--format json` payload is:

```json
{
  "schema_version": 1,
  "diagnostics": [
    {"severity": "error|warning", "code": "LNT-001", "message": "…",
     "file": "docs/…md", "line": 12}
  ]
}
```

`file` and `line` are present only when a finding has a source location.
The `code` is a stable, greppable identifier owned by one component.

Stable codes carried today (the authored catalog; it becomes generated once the diagnostic call sites move to typed DTOs — ADR-0009): `CFG-001`, `CFG-002` (config resolver); `DOC-001` (document store); `LNT-001`, `LNT-002`, `LNT-003`, `LNT-010` (linter); `FMT-001` (formatter); `FIN-001` (finaliser); `TPL-001` (templates); `ASM-001`, `ASM-002`, `ASM-003` (output collision), `ASM-004` (include containment) (assembler); `TRC-COV-001`, `TRC-COV-002`, `TRC-KIND-001` (trace coverage).
The shared struct lives in `src/diag.rs`.
The full owner/severity/stability registry is the planned Diagnostics & Exit-Code Registry fragment.
