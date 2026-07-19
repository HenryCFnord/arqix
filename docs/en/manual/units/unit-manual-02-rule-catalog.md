---
id: unit-manual-02
title: The Rule Catalog
slug: the-rule-catalog
iri: arqix:units/unit-manual-02

rdf:
  type:
    - arqix:classes/unit

triples: []

properties:
  section-kind: manual-chapter

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-19
  updated: 2026-07-19
  lang: en
  translation-of:
  generated: false
---

## The Rule Catalog

Every finding a gate emits carries a stable rule id (the diagnostics contract, REQ-00-00-00-03).
This chapter lists each id and its meaning, grouped by family; the substance of every rule is code, its activation and vocabulary binding is configuration (ADR-0017).

### FM — frontmatter contract (`lint frontmatter`)

| Rule | Meaning |
| --- | --- |
| FM-001 | A required `meta` key (the family's effective required-meta contract) is missing or empty. |
| FM-002 | The id does not carry the family's prefix, or the filename does not start with `<id>-`. |
| FM-003 | The iri does not match the family's namespace derived from the id. |
| FM-004 | The slug does not match the filename tail. |
| FM-005 | The first heading does not match the title. |
| FM-006 | Duplicate id or iri across the corpus. |
| FM-007 | `properties.section-kind` is outside the effective vocabulary (`[frontmatter].section-kinds` or built-in). |
| FM-008 | `meta.lifecycle-status` is outside the document nature's guarded vocabulary (ADR-0010). |
| FM-009 | A `properties` field value is outside the kind's declared vocabulary (`[kinds.<family>.vocab]`). |

### FMT — canonical formatting (`lint frontmatter`)

| Rule | Meaning |
| --- | --- |
| FMT-001 | The frontmatter is not canonically formatted (opening fence, closure, key shape). |
| FMT-002 | A blank line directly follows the opening `---`. |
| FMT-003 | An unknown top-level key for the family (the declared key order is the contract). |
| FMT-004 | Trailing whitespace on a line. |
| FMT-005 | A `meta` date is not ISO `YYYY-MM-DD`. |
| FMT-006 | `meta.lang` is not the source language. |

### ONT — ontology vocabulary (`lint frontmatter`)

| Rule | Meaning |
| --- | --- |
| ONT-001 | A triple predicate is not a defined ontology property. |
| ONT-002 | An `rdf.type` is neither a defined arqix class nor in the effective external-type vocabulary. |
| ONT-003 | A triple object in the arqix namespace resolves to no scanned document. |
| ONT-004 | An `rdfs` target (`sub-class-of`, `domain`, `range`) is not a defined class. |
| ONT-005 | An `owl.inverse-of` names a property without a property document. |
| ONT-006 | The ontology index lists a class or property that is not defined. |

### SRC — source provenance (`lint frontmatter`, keyed on `arqix:classes/source`)

| Rule | Meaning |
| --- | --- |
| SRC-001 | The source iri is not `arqix:sources/` plus its lowercased id. |
| SRC-002 | A finalised source misses `uri`/`accessed`, or `local-copy` and `sha256` are not given together. |
| SRC-003 | `accessed` is not a calendar date. |
| SRC-004 | `sha256` is not a 64-character lowercase hex digest. |
| SRC-005 | `local-copy` escapes the repository or lies inside a documentation root. |
| SRC-006 | The local copy is missing, or its bytes do not hash to the recorded digest. |

### EARS and REQ — requirement style and linkage (`lint requirements`)

| Rule | Meaning |
| --- | --- |
| EARS-001 | The Requirement section does not contain exactly one normative sentence. |
| EARS-002 | The sentence matches no EARS pattern. |
| EARS-003 | A forbidden RFC-2119 keyword (outside the SHALL/SHOULD/MAY subset). |
| EARS-004 | Not exactly one normative keyword in the sentence. |
| EARS-005 | The keyword is unusual for the requirement kind (warning, style-guide matrix). |
| EARS-006 | The sentence subject is not an allowed arqix subject form. |
| REQ-ID-001 | The requirement file has no frontmatter. |
| REQ-ID-002 | The id does not match `REQ-XX-YY-ZZ-NN`. |
| REQ-ID-003 | The iri does not match the id. |
| REQ-ID-004 | The slug does not match the filename tail. |
| REQ-ID-005 | Duplicate requirement id. |
| REQ-ID-006 | The `NN` sequence of a domain is not contiguous from 01. |
| REQ-KIND-001 | `rdf.type` is not exactly one requirement subclass. |
| REQ-META-001 | A key of the effective `[kinds.req].required-meta` contract is missing or empty. |
| REQ-LNK-001 | The requirement declares no `derived-from` story. |
| REQ-LNK-002 | `derived-from` references a missing story. |
| REQ-LNK-003 | A story's `has-requirement` has no matching `derived-from`. |
| REQ-LNK-004 | `has-requirement` references a missing requirement. |
| REQ-LNK-005 | `derived-from` has no matching `has-requirement` in the story. |
| REQ-LNK-006 | The story declares no `has-requirement` link at all. |
| US-WF-001 | The story id encodes one workflow but `is-part-of-workflow` names another (story-driven module). |
| US-PER-001 | The story's persona is not declared on its workflow; consolidation personas are exempt (story-driven module). |

### LNT — corpus linter (`lint run`)

| Rule | Meaning |
| --- | --- |
| LNT-001 | An include target does not exist. |
| LNT-002 | Duplicate document id. |
| LNT-003 | A body reference marker's target does not resolve. |
| LNT-004 | The lifecycle value is outside the nature's vocabulary. |
| LNT-005 | The done claim is violated: a `done` story has a requirement without an active verifying test. |
| LNT-006 | The id does not match the family's configured id-pattern. |
| LNT-007 | The id encodes one story but the declared owner is another. |
| LNT-010 | `translation-of` points to an unknown source document. |

### TRC — trace markers (`trace markers`)

| Rule | Meaning |
| --- | --- |
| TRC-001 | A marker references an unknown requirement. |
| TRC-002 | A test carries neither a `verifies`/`plans` marker nor `arqix:no-requirement`. |
| TRC-003 | An ignore reason is not `US-XX-YY-ZZ: <text>`. |
| TRC-004 | A marker payload is malformed (expected `REQ-XX-YY-ZZ-NN`). |
| TRC-005 | A test carries both a `verifies` marker and `arqix:no-requirement`. |
| TRC-006 | An `implements` marker has no `derived-from` counterpart in the spec. |

### TPL, ASM, CFG, POL — creation, assembly, configuration, policy

| Rule | Meaning |
| --- | --- |
| TPL-001 | `doc new` refuses to overwrite an existing file. |
| TPL-002 | A template uses an unknown placeholder, or the requested id is already taken. |
| TPL-003 | A `--set` key the template does not use. |
| ASM-001 | An include cycle. |
| ASM-002 | An include target cannot be read. |
| ASM-003 | An output collision: two sources generate the same page. |
| ASM-004 | An include target escapes the repository root. |
| ASM-005 | A heading shift overflows the outline depth. |
| ASM-006 | An include target escapes the configured roots. |
| CFG-001 | `arqix.toml` is not parseable, or a value has the wrong type. |
| CFG-002 | An unknown configuration key (ignored under schema v1, warning). |
| POL-001 | A changed file lies outside the declared change scope. |

TPL-002 covers two defects today; splitting the taken-id case into its own rule is open cleanup.
