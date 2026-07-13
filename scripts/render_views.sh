#!/usr/bin/env bash
# Render the C4 architecture views from the model to SVG (ADR-0016, US-04-01-18).
#
# docs/en/architecture/model/workspace.dsl is the single source of truth; these
# SVGs are generated artefacts, committed and kept fresh by the freshness gate
# (`just render-views-check` / the architecture-diagrams workflow).
#
# STATUS: scaffold. Authored without a Docker daemon (the arqix.dev dev sandbox
# had none), so the Kroki invocation below is unverified. First real run happens
# in CI or locally via `just render-views`; on that run, confirm:
#   - whether the Kroki gateway image renders structurizr directly or needs the
#     `yuzutech/kroki-structurizr` companion container,
#   - the exact view-key mechanism (header vs diagram option),
#   - SVG determinism (strip any generated ids/timestamps before committing, or
#     the freshness diff will be noisy).
set -euo pipefail

MODEL_DIR="docs/en/architecture/model"
OUT="${1:-$MODEL_DIR/generated}"
# The views declared in workspace.dsl that documents embed.
VIEWS=("SystemContext" "Containers")
# Pin the renderer: Kroki renders structurizr via PlantUML, whose layout
# coordinates depend on the version, so the freshness gate is only stable
# against a fixed image tag. Override KROKI_IMAGE to match your local pull.
KROKI_IMAGE="${KROKI_IMAGE:-yuzutech/kroki}"

KROKI_URL="${KROKI_URL:-}"
started=""
if [[ -z "$KROKI_URL" ]]; then
  docker run -d --rm --name arqix-kroki -p 8000:8000 "$KROKI_IMAGE" >/dev/null
  started="1"
  KROKI_URL="http://localhost:8000"
  for _ in $(seq 1 30); do curl -sf "$KROKI_URL/health" >/dev/null 2>&1 && break; sleep 1; done
fi
cleanup() { [[ -n "$started" ]] && docker stop arqix-kroki >/dev/null 2>&1 || true; }
trap cleanup EXIT

mkdir -p "$OUT"
for view in "${VIEWS[@]}"; do
  slug="$(echo "$view" | tr '[:upper:]' '[:lower:]')"
  curl -sf "$KROKI_URL/structurizr/svg" \
    -H "Content-Type: text/plain" \
    -H "Kroki-Diagram-Options-View-Key: $view" \
    --data-binary "@$MODEL_DIR/workspace.dsl" \
    -o "$OUT/$slug.svg"
  echo "rendered $view -> $OUT/$slug.svg"
done
