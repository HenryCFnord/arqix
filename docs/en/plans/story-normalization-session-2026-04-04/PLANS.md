---
title: "Story normalization session plan"
date: 2026-04-04
status: reviewed
category: docs
branch: docs/add-personas-user-stories
---

# Plan

## Summary

Document the story-normalization session for `docs/en/architecture/stories` on branch `docs/add-personas-user-stories`.

This plan distinguishes the reviewed execution intent from the implementation work already present on the branch.

## Scope

- In scope: planning-package documentation for the session focused on `docs/en/architecture/stories`
- Out of scope: further implementation edits outside this planning package

## Branch context

- Branch: `docs/add-personas-user-stories`
- Session scope: `docs/en/architecture/stories`
- Branch evidence of implementation work already present:
  - `3f87aaa` `Normalize user story IDs and filenames`
  - `cb79104` `Normalize non-compliant user story metadata and headings`
  - `91ca72d` `Normalize persona references in user story sentences`

## Identified issues

- duplicate user story IDs
- inconsistent filename casing
- underscores vs hyphens in filenames
- filename ID vs frontmatter `id` mismatches
- incorrect `iri` values
- `meta.updated` needing refresh on touched files
- frontmatter `title` not consistently in Title Case
- first Markdown heading not matching the frontmatter title
- named persona references in user story sentences instead of generic roles

## Intended normalization rules

- Normalize filenames to `US-XX-YY-ZZ-slug.md`.
- Keep `XX` and `YY` stable.
- Reassign `ZZ` only as needed to make each `XX-YY` group unique, collision-free, and strictly ascending.
- Use uppercase `US` and hyphens only in filenames and IDs.
- Keep the slug close to the original filename slug.
- Align frontmatter `id` with the filename ID.
- Set `iri` to `arqix:user-stories/us-XX-YY-ZZ`.
- Refresh `meta.updated` on touched files.
- Keep frontmatter titles in proper Title Case.
- Make the first Markdown heading exactly `## <title>`.
- Replace named persona labels in the leading user-story sentence with generic role forms only.

## Execution artefacts prepared during the session

Three Codex prompts were prepared during the session. They are session artefacts, not stored prompt files in this package.

1. Filename and metadata normalization prompt.
Purpose: normalize filenames, `id`, `iri`, and `meta.updated`, and resolve duplicate or conflicting story IDs.

2. Title and heading alignment prompt.
Purpose: normalize frontmatter titles to Title Case and align the first Markdown heading with the frontmatter title.

3. Persona-reference normalization prompt.
Purpose: replace named persona labels in the leading user-story sentence with the required generic role wording.

## Expected validation checks

- no duplicate user story IDs remain in `docs/en/architecture/stories`
- touched story files still have valid frontmatter
- touched titles are in Title Case
- touched files begin with a first heading that exactly matches the frontmatter title
- touched leading user-story sentences remain valid `As ..., I want ..., so that ...` sentences
- touched leading user-story sentences no longer use named persona labels
- each normalization pass changes only non-compliant files for that pass

## Risks and caveats

- Renumbering story IDs can affect any downstream references outside `docs/en/architecture/stories`.
- Title Case normalization needs human review for domain-specific capitalization.
- Persona normalization should affect only the leading user-story sentence, not acceptance criteria or notes.
- This package is being documented after implementation activity already occurred on the branch, so it records the session retrospectively.

## Suggested execution strategy

1. Audit the story directory and identify non-compliant files per concern.
2. Normalize filenames, IDs, `iri`, and `meta.updated` first.
3. Normalize titles and first headings second.
4. Normalize named persona labels in the leading user-story sentence third.
5. Run validation after each pass and keep each pass reviewable as a focused commit.
6. Finish with human review of both the story directory and this planning package.
