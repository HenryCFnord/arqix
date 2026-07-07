---
id: unit-icd-02
title: Exit Codes
slug: exit-codes
iri: arqix:units/unit-icd-02

rdf:
  type:
    - arqix:classes/unit

triples: []

properties:
  section-kind: icd-exit-codes

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

## Exit Codes

<!-- arqix:references-artefact arqix:requirements/req-00-00-00-02 -->
<!-- arqix:references-artefact arqix:requirements/req-04-01-08-01 -->
The stable exit-code contract (REQ-00-00-00-02, REQ-04-01-08-01) is the first thing a CI gate or agent reads:

| Code | Meaning | Example |
| ---: | --- | --- |
| `0` | success | a command completed with no findings |
| `1` | findings / quality-gate failure | `lint run` found an error; `fmt --check` saw an unformatted file; `verify` had a failing sub-step |
| `2` | usage or system error | an unknown flag; a source-write or scaffold I/O failure |
| `70` | unimplemented stub | a Phase-5 command whose story has not shipped |

`70` sits deliberately outside the stable `0/1/2` range so a stub can never be mistaken for a real result.
Within `2`, arqix distinguishes two sub-cases by convention: a **usage** error (bad arguments, reported by clap) and a **system** I/O error (`fmt`/`finalise` cannot write a source file; `doc new`/`doc init` cannot create the target) — both are `2`, never the exit-`1` findings channel.
The governing code is the dispatcher in `src/main.rs` (`EXIT_UNIMPLEMENTED`) and the per-command handlers.
