---
title: "Pre-0.2.0 refactor program plan"
date: 2026-07-14
status: draft
category: docs
branch: claude/refactor-prep
---

# Plan

## Summary

arqix is a healthy, deliberately-minimal single-crate Rust CLI (17 modules, ~11.5k source / ~5.3k test lines) that dogfoods its own doc-as-code checks and is governed by an explicit ADR set.
The audit surfaced no correctness bugs — every finding is a maintainability or testability improvement.
Adversarial verification then downgraded most of them: 10 of 17 findings dropped to low value, and several "safe now" claims turned out to have hidden couplings.
High-value work is scarce and concentrated: making the requirements checker read the same effective `required-meta` contract as the frontmatter checker (the one-source completion the requirements checker was left out of) is the program's single high-value item.

This plan is written before any code moves.
It records the reviewed sequencing, the two-band structure, the confirmed module homes, the crate rejection, and the owner decisions that still gate specific slices.

## Headline

- No correctness bugs were found; the whole program is maintainability and testability.
- Exactly one high-value item: required-meta one-source (slice 6), a literal instance of ADR-0011's own C6 divergence anti-pattern.
- Most dedup findings are small (helpers of a dozen or so lines); they are worth doing to stop the copies drifting, not because the line count is large.
- Where a finding did not survive scrutiny it is still carried, but scoped down honestly — see the walker consolidation (really two safe-to-merge functions, not eight) and the frontmatter-split close-out (documentation, not a merge).

## The oracle-freeze sequencing insight

The single dominant sequencing fact is the oracle-fidelity freeze.
`scripts/check_frontmatter.py` and `scripts/check_requirements.py` are still the active behavioural oracle; their Rust ports are Phase 5 work, and only `arqix_trace.py` has entered the cross-check phase.
Any change to the two checker modules, or to any oracle-coupled configured value, must either land in Rust and Python from one source (ADR-0011 rule 1) or wait for the checker retirement milestone (task #78).

That milestone therefore gates roughly two-thirds of the program, which inverts the naive order.
The intuitive plan would start with the "obvious" checker-internal dedup; the correct plan defers all of it behind the gate and front-loads only the work that touches genuinely non-oracle modules.
So the program splits into a pre-gate band of independent, low-risk helper dedup and test hygiene, a single gate slice, and a post-gate band that completes the ADR-0011 config backlog and the one large structural spec.

## Two-band structure

Pre-gate band (independent of the gate, can proceed immediately):

- Slice 1 — test-baseline-hygiene.
- Slice 2 — markdown-scan-helpers (new `src/markdown.rs`).
- Slice 3 — path-and-walker-helpers (new `src/util.rs`).
- Slice 12 — splitter-contract-docs (documentation close-out; no code, no gate dependency).

The gate:

- Slice 4 — oracle-retirement-gate (owned by the self-hosting strand, task #78; tracked here as the program gate).

Post-gate band (unblocked once the Python checkers retire, or done dual-source before then):

- Slice 5 — checker-internal-dedup.
- Slice 6 — required-meta-one-source (the one high-value item).
- Slice 7 — frontmatter-vocab-config.
- Slice 8 — ontology-as-config-adr (the one large structural spec).
- Slice 9 — ontology-vocabulary-derivation.
- Slice 10 — iri-namespace-config.
- Slice 11 — lifecycle-model-selector (conditional / low priority).

## The twelve slices

The table below transcribes the reviewed slice set.
"Depends-on" names the gate or the prerequisite slice; "—" means independent.

| # | Name | Goal | Dimension | Spec changes | Test strategy | Risk | Depends-on |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | test-baseline-hygiene | Fold `tests/render_config.rs` into `tests/corpus_guards.rs` (drop the cloned repo-reader, adjust the module doc for the two `arqix:no-requirement` wiring guards); add a `src/config.rs` `#[cfg(test)]` block over the pure helpers `toml_to_json` / `string_array` / `json_string_array`; give the four inline publisher staging tests unique per-test temp dirs with cleanup (interim hygiene only). | test-placement | None (test-only; touched code is `arqix:no-requirement` or already-specified pure helpers). The `resolve()` precedence sub-proposal is dropped — `resolve(dir)` reads `arqix.toml` from disk, so its precedence stays in `tests/cli_config.rs`. | This slice is the STRENGTHEN / TIDY phase applied standalone. No production behaviour changes, so no red-then-green on src; new inline tests land green, each carrying an `arqix:no-requirement` marker. | low | — |
| 2 | markdown-scan-helpers | New neutral `src/markdown.rs` (`pub(crate)`): promote the byte-identical `heading_level`, add a `headings_outside_fences(text)` iterator that `first_heading_level` (`.next`) and `pdf_staging_plan` (`.take(2)`) consume, and extract `included_target_set(files: &[PathBuf])` for both `included_targets` and `drop_included_fragments`. Do NOT fold in the line-emitting expand/stage loops or the directive-scanning loops, and do NOT touch `checkers/frontmatter.rs` `first_heading` (oracle-mirrored, deliberately non-fence-aware). | dedup-shared-lib | None — pure behaviour-preserving internal refactor. The module-home choice (`src/markdown.rs`, NOT `parser.rs` which mirrors `check_frontmatter.py`) is a one-line arc42 ch5 note, not a spec. | Pure refactor, so characterization tests FIRST: pin `heading_level` over the `1..=6`/space cases, pin `first_heading_level` and `pdf_staging_plan` `(level, text)` pairs across fenced/unfenced inputs, and set-equality of `included_targets` / `drop_included_fragments` on a fixture. Commit green, extract, re-run, then tidy. | low | — |
| 3 | path-and-walker-helpers | New neutral `src/util.rs` (`pub(crate)`): (1) add `to_posix(&Path)` / `&str` for the ~13 `to_string_lossy().replace('\\', "/")` sites, rewiring the NON-checker sites only (assembler `rel`, store, reporter, publisher, linter, rewriter, trace); (2) collapse ONLY `store::walk` and `publisher::collect_markdown` (the byte-alike skip-dirs + symlink-skip + sort + `.tpl.md` pattern). Explicitly REJECT `walkdir`. Do NOT merge `trace.rs`'s oracle-pinned walkers, the flat gate scans, `copy_asset`, or `discover_documents`. | dedup-shared-lib | None — internal refactor, no config surface added. Record the `walkdir` rejection as an ADR-0014 follow-up note (a recurring "why not a crate", like the rmcp rejection). | Characterization-first, heavier than slice 2: before merging the walker, pin traversal ORDER (`paths.sort`), directory-symlink non-following, and `.md`/`.tpl.md` filtering on a fixture tree. `to_posix` rewrites are mechanical; snapshot affected command outputs before/after. Keep the merged walker OUT of `trace.rs` and the checkers. | low | slice 2 |
| 4 | oracle-retirement-gate | Enabling milestone (task #78): complete conformance of the Rust frontmatter/requirements checkers against the two Python scripts, demote the Python checkers to CI cross-checks, and after the grace period retire them so the Rust engine owns the checker contract. | self-hosting | Update arc42 ch8 checker-oracle policy from "active oracle / Phase 5" to retired; record the retirement in the risks / technical-debt unit (the ADR-0010-style lifecycle already applied to `arqix_trace.py`). | The conformance suite (identical JSON findings on the real corpus plus selftest fixtures, driven via the `ARQIX_BIN` override) is the acceptance gate. No refactor here — it flips the freeze. | medium | — |
| 5 | checker-internal-dedup | Now that the checkers own their contract, remove internal duplication: hoist byte-identical `json_string` and `collect_md` into `src/checkers/mod.rs` as `pub(super)`; route the two deferred checker `to_posix` sites through `util::to_posix`; consolidate `is_leap` / `days_in_month` / `is_calendar_date` into a new `src/date.rs`, wiring both the frontmatter checker and `rewriter::valid_iso_date`. The rewriter half may land pre-gate (no oracle constraint); the checker half completes here. | dedup-shared-lib | None — behaviour-preserving internal moves. The reason to gate was checker auditability under the live oracle, not any spec. | Characterization-first per module: the conformance fixtures already pin checker JSON byte-for-byte, so run them before and after; add a `src/date.rs` unit block over leap / days-in-month / `1..=9999` boundaries (mirroring `datetime.date.fromisoformat`) before consolidating. Tidy the dead per-module copies. | low | gate (slice 4) |
| 6 | required-meta-one-source | Complete the C6 consolidation the requirements checker was left out of: make REQ-META-001 resolve the effective `[kinds.<family>].required-meta` contract instead of its hardcoded const, so FM-001 and the requirements checker agree on the same file under a non-default `required-meta`. The program's one high-value item. | hardcoded-to-config | Confirm/extend REQ-01-01-19-03 to bind the requirements checker as a second consumer; no new config surface (`[kinds.<family>].required-meta` already exists and is documented). Lift a shared `required-meta` resolver into `config.rs` — `KindContract` carries family/dir/key-order/id-pattern but NOT required-meta, so effort is S/M not pure S. | Spec-first then red/green: after the REQ update, add a failing test with a non-default `[kinds.req].required-meta` showing FM-001 and REQ-META-001 disagree on a req-family file (red), then wire the resolver so they agree (green), committing the test before/with the impl. | medium | gate (slice 4) |
| 7 | frontmatter-vocab-config | Clear the two small frontmatter-constant CONFIG-AUDIT ride-alongs: make `SECTION_KINDS` config-loaded via `[frontmatter] section-kinds` (row C9, option a) and surface allowed external `rdf.type` values via `[frontmatter] allowed-external-types` (default `rdfs:Class`, `rdf:Property`). Defaults reproduce the present nine section-kinds and two external types exactly, so an unconfigured corpus stays byte-identical. | hardcoded-to-config | Author the C9 story/REQ for `[frontmatter] section-kinds` under ADR-0011 (no new ADR; C9 is an authorized ride-along with `[frontmatter]` as its stated home). Non-arqix types ride the same strand as an additional row. Prefer option (a) `[frontmatter]` over ontology-backed individuals to avoid coupling to the ontology-as-config ADR. | Spec-first REQ, then an ADR-0011 rule-2 test that the DEFAULT config reproduces the current nine values / two types, plus a non-default case proving FM-007 honours it (red before the loader, green after). Post-gate, one source = the Rust engine only. | low | gate (slice 4) |
| 8 | ontology-as-config-adr | Author the one large structural spec: a new ontology-as-config ADR stating that the `docs/ontology/` corpus is the single source of truth for the class/property/individual vocabulary AND the full IVVQ verification-method vocabulary, loaded at runtime by the engine (the `check_vocabulary` path already does this for classes/properties). The ADR records the boundary and resolves the contested questions. No code in this slice. | hardcoded-to-config | New `docs/en/architecture/adr/ADR-00NN-ontology-as-config.md`, plus an ADR-0011/ADR-0012 amendment note deciding whether the IRI namespace SEGMENT after `arqix:` is "naming" (config) or protected ontology identity (genuinely contestable; blocks slice 10). The largest, most decision-heavy spec piece — its own owner-review slice. | Spec-only; the deliverable is the ADR plus amendment reviewed and accepted by the owner. It defines the acceptance criteria (what derives versus stays code, IVVQ scope) that slices 9 and 10 implement test-first. | medium | gate (slice 4) |
| 9 | ontology-vocabulary-derivation | Implement the ADR: derive the requirement-subclass membership used by `trace.rs` `GATE_KIND_RE` from the ontology's sub-class-of `arqix:classes/requirement` individuals rather than the hardcoded regex alternation, and generalize the runtime vocabulary loader to cover the full IVVQ verification-method vocabulary as ontology individuals. Keep the IRI-to-coarse-label maps (`KIND_CLASSES`, `kind_short`) and the kind-to-EARS-keyword contract in code unless slice 8 decided to add the coarse label to the ontology. | hardcoded-to-config | None beyond the slice-8 ADR; this is its implementation. Introducing ontology loading into the deliberately-isolated `requirements.rs` / `trace.rs` is conformance-sensitive — post-gate it is Rust-only, which is why it waits for the gate. | Behaviour-visible, so spec-first (the slice-8 ADR) then red/green: a test that adds a requirement-subclass individual to a fixture ontology and asserts `GATE_KIND_RE` / the vocabulary check recognises it with no code edit (red against the hardcoded set, green after derivation). Characterization-pin the current three-subclass behaviour first. | medium | slice 8, gate |
| 10 | iri-namespace-config | Give families a configurable IRI namespace so non-default `[kinds.<family>]` families get an iri contract (today they fall through to the generic branch with no iri validation). (a) Unblocked internal dedup: collapse the three literal `arqix:` namespace copies (frontmatter `arch_ns`/`ont_ns` and `requirements.rs` inline `format!` literals) into shared constants, no config. (b) Extend `[kinds.<family>]` with an `iri-namespace` key resolved through `config.rs` `KindContract`, defaults reproducing `arch_ns`/`ont_ns` exactly. The `arqix:` SCHEME stays fixed; only the segment after it becomes config. | hardcoded-to-config | Part (b) is blocked on the slice-8 ADR-0011/0012 amendment authorizing `[kinds].iri-namespace` (neither ADR currently scopes it; ADR-0012 pointedly left the namespace alone). Audit row C4. If the owner rules the segment is protected ontology identity, this slice reduces to part (a) only. | Part (a) is a pure-refactor characterization dedup (pin current iri output, extract constants, prove identical) and could be pulled forward into slice 3 if desired. Part (b) is spec-first (the amendment) then red/green with a non-default family declaring an iri-namespace and FM-002 validating against it — pin the load-bearing REQ-LNK-001 owner path (`requirements.rs` story-iri) before routing it through config. | medium | slice 8, gate |
| 11 | lifecycle-model-selector | CONDITIONAL / low priority: let a family DECLARE which of the three code-resident lifecycle models it follows via `[kinds.<family>].lifecycle-model = work-item\|binary\|prose`, defaulting to the built-in family mapping, so a custom family can opt into the story/req model instead of silently falling into the prose default. The rung sets and the done-claim invariant stay in code (ADR-0010/ADR-0011 substance) — do NOT expose rung values as free text. | hardcoded-to-config | A small REQ for the lifecycle-model selector on `KindContract` (no lifecycle field today; both FM-008 and linter LNT-004 infer the model by name-match — that dual inference is the deliberate ADR-0010 dual-oracle, not a consolidation target). Worth doing ONLY once custom work-item families are a real use case. | Spec-first REQ, then red/green: a custom family with `lifecycle-model = work-item` accepts the story rungs (red under the else-prose fallback, green after the selector). Rung-set behaviour stays pinned unchanged. | low | gate (slice 4) |
| 12 | splitter-contract-docs | Close out the frontmatter-split finding as DOCUMENTATION, not a merge: record the three genuinely distinct splitter contracts (parser.rs semantic via `py_splitlines`, rewriter.rs byte-lossless round-trip, the checker oracle-faithful copies) and do NOT force a single splitter. Verification showed the proposed "safe now" assembler `frontmatter_line_count` -> `body_offset - 1` merge is NOT drop-in (it mixes `py_splitlines` index space with `str::lines()` and needs a re-parse since `chapter_id` discards the Document), so leave the local assembler helper as-is. | dedup-shared-lib | Documentation only — an arc42 ch8 / crosscutting note naming the semantic vs lossless vs oracle splitter contracts. | No production change under the recommended path; if the owner elects the merge, characterization-pin the assembler over exotic line separators (lone `\r`, `\x0b`/`\x0c`, NEL/LS/PS) FIRST — those are exactly where the two index spaces diverge — before touching it. | low | — |

## Confirmed module homes and the crate rejection

New non-oracle helpers must NOT land in `parser.rs` or the `checkers/` tree — both are oracle-mirrored.
They belong in fresh neutral modules kept `pub(crate)`:

- `src/markdown.rs` — markdown scanning (`heading_level`, `headings_outside_fences`, `included_target_set`) — slice 2.
- `src/util.rs` — POSIX path normalization (`to_posix`) and the re-scoped `store`/`publisher` walker — slice 3.
- `src/date.rs` — ISO calendar-date validation (`is_leap`, `days_in_month`, `is_calendar_date`) — slice 5, with the rewriter half landable pre-gate.

`walkdir` stays rejected under ADR-0014.
The walks are pinned to reproduce Python `sorted(rglob('*.md'))` byte-for-byte, and conformance tests hold that.
`walkdir` would still need manual `sort()` + skip-dirs + `.tpl.md` filtering to match, so it removes near-zero code while adding a supply-chain entry and a silent-ordering/symlink-drift risk.
Record the rejection as an ADR-0014 follow-up note alongside slice 3.

## Deferred owner decisions

These are carried from the assessment's open questions and tagged to the slice where each surfaces.

- Gate timing for the high-value fix (slices 6, and the 7 ride-alongs): land required-meta one-source pre-gate via simultaneous Rust+Python single-source loading (ships the only high-value item sooner, more effort/risk), or post-gate Rust-only (simpler, waits on task #78)?
  The whole post-gate band hangs off this milestone.
- IRI-namespace boundary (blocks slice 10 part b, decided in slice 8): is the segment after `arqix:` "naming" (configuration per ADR-0011) or part of the protected ontology-vocabulary identity the `arqix:` keep defends?
  ADR-0011 and ADR-0012 both scoped it out; an explicit amendment is needed.
  If ruled protected, slice 10 collapses to internal dedup only.
- Ontology coarse-label decision (bounds slice 9 payoff, decided in slice 8): add the coarse labels functional/quality/constraint to the ontology so `KIND_CLASSES` and `kind_short` fully derive, or keep those IRI-to-label maps in code?
  Only pure-membership uses (`GATE_KIND_RE`) derive automatically either way.
- Section-kind home (slice 7): `[frontmatter] section-kinds` (row C9 option a, recommended, no ontology-ADR dependency) versus ontology-backed section-kind individuals (option b, one-source consistency but couples to the ontology-as-config ADR)?
- Lifecycle-model selector (slice 11): do it now, or leave as a documented deliberate keep until custom work-item families are a real use case?
  The finding's own value is low and gated on that use case appearing.
- Frontmatter-split close-out (slice 12): leave the assembler `frontmatter_line_count` helper as-is (recommended), or invest in switching the assembler body loop to `py_splitlines` so count and iteration share one splitter?
- Shared-module homes: confirm `src/markdown.rs`, `src/util.rs`, and `src/date.rs` as the neutral `pub(crate)` homes, and confirm `walkdir` stays rejected per ADR-0014.

## Process

1. Land the three prep artefacts (see STATUS.md, Phase 0) plus this plan package as the refactor-program PR — no code moves yet.
2. Make the deferred owner decisions above in review.
3. Execute Phase A (slices 1–3, plus the documentation-only slice 12) — independent of the gate.
4. Land the gate (slice 4, task #78 self-hosting strand).
5. Execute the post-gate band (slices 5–11) per the owner's gate-timing decision, each spec-first or characterization-first as its row states.
