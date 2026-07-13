# arqix task runner — thin wrappers over the real gate and cargo.
# The process truth lives in scripts/arqix verify and .github/workflows/ci.yml;
# these recipes only save typing. Bare `just` lists them.

# Show the available recipes (default)
list:
    @just --list

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

# Render the C4 views from the model to committed SVGs (needs Docker; ADR-0016)
render-views:
    ./scripts/render_views.sh

# Fail if the committed C4 view images are stale against a fresh render (needs Docker)
render-views-check:
    ./scripts/render_views.sh {{justfile_directory()}}/target/views-fresh
    git --no-pager diff --no-index --exit-code docs/en/architecture/model/generated {{justfile_directory()}}/target/views-fresh

# Render the documentation PDFs via Pandoc in the fat container (needs Docker;
# pulls pandoc/extra). Extra args pass through to `arqix render pdf`
# (e.g. `just render-pdf --lang en --out out.pdf`, or specific files).
render-pdf *ARGS:
    ./scripts/render_pdf.sh {{ARGS}}

# Everything CI runs, locally
ci: verify lint conformance
