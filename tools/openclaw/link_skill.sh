#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="${HOME}/projects/arqix"
SOURCE_DIR="${PROJECT_ROOT}/.agents/skills"
TARGET_DIR="/usr/lib/node_modules/openclaw/skills"

link_skill() {
  local skill_name="$1"
  sudo ln -sfn "${SOURCE_DIR}/${skill_name}" "${TARGET_DIR}/${skill_name}"
  ls -la "${TARGET_DIR}" | grep "${skill_name}"
}

link_skill "arqix-repo-readonly"