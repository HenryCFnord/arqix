---
id: ADR-0015
title: Marker Freshness
slug: marker-freshness
iri: arqix:adrs/adr-0015
rdf:
  type:
    - arqix:classes/adr
triples:
  - predicate: arqix:properties/guides-implementation-of
    object:
      - arqix:requirements/req-03-01-11-01
      - arqix:requirements/req-03-01-11-02
      - arqix:requirements/req-03-01-11-03
properties:
  decision-status: accepted
external-references: []
meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-13
  updated: 2026-07-13
  lang: en
  translation-of:
  generated: false
---

## Marker Freshness

### Context

Coverage answers whether a requirement carries a verifying marker; it does not answer whether that verification is still current.
A `verifies` or `implements` marker resolves structurally even after the requirement it targets has been rewritten, so a green "verified" can rest on a marker placed against an older version of the requirement (US-03-01-11).
Closing that gap turns arqix from architecture-as-code into its own verification apparatus, the way an IVVQ process validates a system against its specification.

The signal is git arithmetic: a marker is out of date when the requirement it points at, or that requirement's owning story, was committed more recently than the marker's own file.
This raises three tensions with the existing design.
First, arqix has never shelled out to git — the whole binary is deterministic and free of ambient state, and `Command::new` is confined to configured external renderers and the binary re-invoking itself.
Second, determinism (REQ-00-00-00-01) demands byte-identical output for identical inputs, and git history is now part of "the inputs".
Third, the Python trace oracle is frozen on its retirement path (arc42 chapter 8), so a new analysis must not perturb the ported `trace scan/coverage/matrix` surface.

### Decision

**`trace freshness` computes possibly-stale active markers from git commit recency, with the git shell-out isolated and the decision pure.**

- The command is a new verb in the existing `trace` family (ADR-0005: every analysis exists exactly once); it does not touch `trace coverage` output, so the frozen oracle surface and its conformance suite are untouched and no oracle port is needed — the `--results` and `ratchet` precedents.
- The freshness decision is a pure function over commit timestamps, exactly the shape of the finalise clock (ADR-0004): the caller supplies the numbers, the function reads nothing external.
  A single small function shells out to git (`git log -1 --format=%ct -- <path>`) and is the only new impurity; unit tests inject timestamps and never call git, and one integration test builds a git fixture with pinned `GIT_COMMITTER_DATE` so history — and therefore output — is byte-stable.
- A marker whose file, requirement, or story has no reachable git history is treated as fresh, not as an error (REQ-03-01-11-02): a released tarball without `.git`, or an untracked file, degrades to reporting nothing stale.
- Granularity is file-level; the signal is reported as "possibly stale" and the verify sub-step is informational (REQ-03-01-11-03), never gating, because a file-level comparison also fires on metadata-only commits.
  Line-level attribution (`git log -L`, `git blame`) is the named hardening path if the false-positive rate proves too high in practice.

### Alternatives Considered

- **A `--freshness` flag or extra column on `trace coverage`:** rejected — it conflates two analyses (ADR-0005) and would either change coverage's default JSON, breaking value-equality with the frozen oracle, or bolt on opt-in gymnastics to avoid that; a separate verb keeps coverage byte-identical.
- **Frontmatter `updated` dates instead of git:** rejected — `updated` is itself stamped by `finalise`, so comparing it against a marker would measure metadata bumps rather than substantive change, and it cannot see the marker file's own history at all.
- **Line-level attribution from the start (`git log -L`):** deferred — it removes the metadata-commit false positives but is heavier, carries its own rename and determinism caveats, and is not needed to ship an informational signal; the file-level version earns the concept first.
- **A gating (non-informational) check:** rejected for now — file-level granularity is too coarse to fail a build on; freshness informs review until line-level attribution makes it precise enough to gate.

### Consequences

- arqix now depends on `git` at runtime for one command; the dependency is isolated to a single function, absent everywhere else, and degrades gracefully when git is missing.
- CI that runs freshness must fetch full history (`actions/checkout` `fetch-depth: 0`); a shallow clone truncates history and would make the signal wrong rather than merely absent.
- The trace engine keeps its determinism contract: the pure core is fully unit-tested from injected timestamps, and the git-dependent path is pinned by a fixture in the one integration test.
- The Python oracle is not extended; `trace scan/coverage/matrix` stay value-equal, and the ARQIX_BIN conformance run is unaffected.
