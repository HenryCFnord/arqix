# Releasing

How an arqix release is prepared, checked, and published.
This process is governed by US-01-01-15 (REQ-01-01-15-01..04); the version-consistency rule is machine-checked by the test suite.

## Versioning policy

- arqix follows [SemVer](https://semver.org).
  Before 1.0.0, minor versions carry features and breaking changes, patch versions carry fixes; 1.0.0 freezes the public contracts.
- The public contracts are versioned individually per ADR-0009: every JSON interface carries its own `schema_version`, the configuration schema is versioned separately as `config_version` (currently schema v1 in `arqix.toml`, incremented only for breaking configuration changes), and the exit codes 0/1/2 (and 70 for stubs) are a stable contract (REQ-00-00-00-02) that no release may change.
- A breaking change to any contract requires a **Migration** note in its [CHANGELOG.md](CHANGELOG.md) entry (REQ-01-01-15-04).

## Consistency rules

- The top release section of [CHANGELOG.md](CHANGELOG.md) matches the version in `Cargo.toml` — checked by `release_documents_stay_consistent_with_the_crate_version` in the test suite (REQ-01-01-15-01).
- An unreleased version carries `— unreleased` in its heading; releasing replaces it with the ISO date.

## Release steps

1. Verify the tree is releasable: `just ci` (cargo test, the dogfooded `arqix verify`, markdownlint, and the Rust lints — the full CI mirror).
2. Confirm the report snapshots are fresh (`./target/debug/arqix report snapshot --check`) and the roadmap coverage number matches the marker gate.
3. Stamp the release: replace `— unreleased` in the top CHANGELOG section with the ISO date.
4. Commit, tag `vX.Y.Z` on `main`, push the tag, and create the GitHub release with the CHANGELOG section as its body.
5. Publish the crate: `cargo publish` (a `cargo publish --dry-run` first never hurts).
   The package is pinned to the tool by the `include` list in `Cargo.toml`; a published version is immutable — a broken one is yanked (`cargo yank --vers X.Y.Z`), never replaced.
6. Start the next cycle: bump the `Cargo.toml` version and open its `## [x.y.z] — unreleased` CHANGELOG section in one commit, so the top section always matches the crate version (the machine-checked rule above).

## Automation boundary

Agents prepare releases; a human releases (REQ-01-01-15-03).
Preparation — CHANGELOG entries, consistency checks, gate runs, this document — is agent work; creating the tag, publishing the GitHub release, and `cargo publish` (which uses the owner's crates.io token) are reserved for the repository owner.
