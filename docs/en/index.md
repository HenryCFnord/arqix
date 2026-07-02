---
title: "arqix documentation"
description: "Entry point for the arqix project documentation"
date: 2026-03-20
status: active
---

# arqix documentation

Welcome. This is the main entry point for arqix project documentation.

arqix is a Rust CLI for structured technical documentation and Architecture-as-Code workflows. It is text-first, Git-friendly, and designed to treat Markdown documents as structured units rather than loose text files.

This `docs/` directory serves two purposes: it is the human-readable documentation for the project, and it is intended to become the first real corpus for arqix itself as features are implemented.

## Contents

- [Project documents](project/) — stable, version-controlled project artefacts
- [Plans](plans/README.md) — branch-local planning packages for OpenClaw intake
- [Blog](blog/) — reflective posts grounded in project development
- [Notes](notes/) — informal working notes (may be rough or incomplete)
- [Experiments](experiments/) — documented experiments with explicit scope and outcome

## Reading guide

If you are new to the project, start with the [README](../../README.md) for a short overview, then read [why arqix had to exist](../blog/2026-03-why-arqix-had-to-exist.md) for the reasoning behind it.

If you want to understand where the project is going, read the [roadmap](../project/roadmap.md).

If you are curious about how AI tooling is used in this project, read the [AI transparency document](../project/ai-transparency.md).

## Documentation conventions

- All documents in `project/` and `blog/` use YAML frontmatter with at minimum `title`, `date`, and `status`.
- Links are standard Markdown links, not Obsidian wikilinks, to keep this directory portable and GitHub-friendly.
- `notes/` and `experiments/` are less formal spaces and may contain incomplete or exploratory content, with frontmatter being optional. This is intentional.
- Documents in `project/` are considered stable and reviewed before update.
