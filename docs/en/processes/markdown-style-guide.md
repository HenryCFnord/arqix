# Markdown Style Guide

This guide defines how Markdown is authored in this repository.
It has two layers: the [markdownlint](https://github.com/markdownlint/markdownlint/blob/main/docs/RULES.md) rule set for structural hygiene, and two project conventions markdownlint cannot express.

## markdownlint

The baseline is markdownlint with every rule on by default.
The repository-root `.markdownlint.jsonc` is the machine-readable source of truth; run it with `npx markdownlint-cli2 "docs/**/*.md"`.

Two rules are disabled, each for a documented reason:

- **MD013 (line length)** is off, because arqix prose is written one sentence per line and a sentence may exceed any column limit (see below).
- **MD041 (first line must be a top-level heading)** is off, because an arqix document carries its title in YAML frontmatter and its first body heading is the `## <Title>` the frontmatter checker enforces, never an `#` H1.

Everything else stays on: heading increment (MD001), no trailing spaces or hard tabs (MD009, MD010), single blank lines (MD012), blank lines around headings, lists, and fenced code (MD022, MD031, MD032), a language on every fence (MD040), and a single trailing newline (MD047).

## One sentence per line

Write each sentence on its own line, however long.
Never wrap a sentence across several lines to fit a column width, and never put two sentences on one line.
Separate paragraphs with a single blank line.
This lets a reviewer comment on an individual sentence instead of an arbitrary wrapped fragment, and keeps diffs to the sentences that actually changed.

For a list item that needs more than one sentence, put each further sentence on its own continuation line, indented to the item's text.

## Markers and directives above the block

An `<!-- arqix:… -->` marker or directive (`arqix:include`, `arqix:references-artefact`, `arqix:documented-by`, …) goes on its own whole line, placed directly above the block it annotates — like a code comment above the code it describes.
Leave a blank line above the marker and none between the marker and its block, so the marker reads as attached to what follows.
