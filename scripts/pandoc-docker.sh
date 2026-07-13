#!/usr/bin/env bash
# Pandoc-in-a-container shim for `arqix render pdf`.
#
# arqix's [policies.render] pdf-command points here. arqix stages the assembled
# pages, then invokes this script with the pandoc arguments it built: the staged
# inputs, `-o <target>`, and the configured `--defaults`. We forward them
# unchanged to pandoc running inside the fat Pandoc/LaTeX/eisvogel image, with
# the repository mounted at /data so every relative path — staged pages, the
# embedded C4 SVGs, the defaults file, and the output target — resolves exactly
# as a local pandoc run would (REQ-04-01-03-04, REQ-04-01-03-05).
#
# The image carries eisvogel and rsvg-convert, so the SVG architecture views
# (ADR-0016) are converted to PDF automatically. Pin PANDOC_IMAGE to a digest
# for a reproducible render, exactly as render_views.sh pins the Kroki image.
set -euo pipefail

# The render image is pinned by digest in one shared place (scripts/pandoc-image.env);
# override PANDOC_IMAGE to use a different local pull.
source "$(dirname "${BASH_SOURCE[0]}")/pandoc-image.env"

# No --user: under rootless Docker (the common local setup — note the "IPv4
# forwarding is disabled" warning) container-root maps to the invoking host user,
# so the output PDF lands owned by you; passing --user <host-uid> instead maps to
# an unprivileged subuid that cannot write the mount. Under rootful Docker the
# output is root-owned — chown it, or set an explicit --user via a wrapper.
exec docker run --rm \
  -v "$PWD":/data -w /data \
  -e HOME=/tmp \
  "$PANDOC_IMAGE" "$@"
