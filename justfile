# arqix task runner — thin wrappers over the gate and cargo.
# The process truth lives in .github/workflows/ci.yml; these recipes only
# save typing. Bare `just` lists them.

# Show the available recipes (default)
list:
    @just --list

# The daily gate: cargo test, the dogfooded arqix verify (the corpus checks), markdownlint
verify:
    cargo build
    cargo test --quiet
    ./target/debug/arqix verify
    npx --yes markdownlint-cli2@0.23.0

# Rust test suite
test:
    cargo test

# Rust lints and formatting
lint:
    cargo clippy --all-targets -- -D warnings
    cargo fmt --check

# Markdown hygiene over the corpus (rules: .markdownlint.jsonc).
# Pinned to the same version as the gate (just verify).
lint-md:
    npx --yes markdownlint-cli2@0.23.0

# Regenerate the report snapshots (units + matrices)
reports:
    cargo build
    ./target/debug/arqix report snapshot --stamp "$(git rev-parse --short HEAD), $(date +%F)"
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
ci: verify lint
