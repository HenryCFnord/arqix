#!/usr/bin/env bash
# Render the documentation PDFs through arqix's render orchestrator, backed by
# the fat Pandoc/LaTeX/eisvogel container (REQ-04-01-03-04, REQ-04-01-03-05).
#
# `arqix render pdf` stages the assembled pages and calls the configured
# pdf-command (scripts/pandoc-docker.sh), which runs pandoc in the container.
# This wrapper only pre-pulls the image so the first render is not a silent
# multi-gigabyte download, then delegates. Extra arguments are forwarded to
# `arqix render pdf` verbatim (e.g. specific files, --lang, --out).
#
# STATUS: scaffold. Authored without a Docker daemon, so the container render is
# unverified. First real run is `just render-pdf`; on that run, confirm that the
# embedded C4 SVGs resolve from the staged pages and that eisvogel renders them,
# then pin PANDOC_IMAGE to the digest (as we did for Kroki).
set -euo pipefail

PANDOC_IMAGE="${PANDOC_IMAGE:-pandoc/extra:latest}"
export PANDOC_IMAGE

echo "pulling $PANDOC_IMAGE ..."
docker pull "$PANDOC_IMAGE"

# Always build the local binary so the render uses this checkout's arqix, not a
# stale ./target/debug or a globally installed release (cargo install arqix is
# the published version, where render pdf may still be a stub). The build is
# incremental and cheap; override ARQIX_BIN to skip it with a chosen binary.
ARQIX="${ARQIX_BIN:-./target/debug/arqix}"
if [[ -z "${ARQIX_BIN:-}" ]]; then
  echo "building arqix ..."
  cargo build
fi

exec "$ARQIX" render pdf "$@"
