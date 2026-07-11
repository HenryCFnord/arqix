# arqix task runner — thin wrappers over the real gate and cargo.
# The process truth lives in scripts/arqix verify and .github/workflows/ci.yml;
# these recipes only save typing. `just --list` shows everything.

# The daily gate: checkers, marker gate, cargo test, dogfooded arqix verify, markdownlint
verify:
    python3 scripts/arqix verify

# Rust test suite
test:
    cargo test

# Rust lints and formatting
lint:
    cargo clippy --all-targets -- -D warnings
    cargo fmt --check

# Markdown hygiene over the corpus (rules: .markdownlint.jsonc).
# Pinned to the same version as the gate (scripts/arqix verify).
lint-md:
    npx --yes markdownlint-cli2@0.23.0

# Trace-oracle conformance cross-check (arc42 chapter 8)
conformance:
    cargo build
    python3 scripts/check_conformance.py
    ARQIX_BIN={{justfile_directory()}}/scripts/arqix cargo test --test cli_trace

# Regenerate the report snapshots (units + matrices)
reports:
    cargo build
    python3 scripts/arqix_report.py --snapshot "$(git rev-parse --short HEAD), $(date +%F)"
    ./target/debug/arqix trace matrix > docs/en/reports/trace/matrix.csv
    ./target/debug/arqix trace matrix --type us-req > docs/en/reports/trace/matrix-us-req.csv

# Everything CI runs, locally
ci: verify lint conformance
