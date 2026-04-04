---
id: experiment-codex-cli-user-story-normalisation
title: Prompting Codex CLI for User Story Normalisation
status: draft
tags:
  experiment
  ai
  codex
  prompting
  documentation
  user-stories
  normalisation
owner: hcf
created: 2026-04-04
updated: 2026-04-04
lang: en
translation_of:
translation_status:
generated: false
supersedes:
superseded_by:
related:
  personas: []
  workflows: []
  stories: []
  requirements: []
  docs:
    - ai-transparency
source: manual experiment log
---

# Prompting Codex CLI for User Story Normalisation

**Question:** Can a chat-driven prompt design session produce a practical set of Codex CLI prompts for safe, scoped normalisation of user stories in `docs/en/architecture/stories`?

**Answer: Yes, with an important caveat.**

The interaction produced a usable sequence of Codex CLI prompts for the target branch and directory. The result was strongest when the work was split into narrow, reviewable steps instead of being framed as one broad clean-up task.

The caveat is that the prompt quality depended heavily on making the constraints explicit. As soon as the task touched identifiers, filenames, frontmatter, Markdown headings, and story wording at the same time, the prompts benefited from being separated into small, deterministic passes.

## Context

The session focused on user story files under:

- `docs/en/architecture/stories`

on branch:

- `docs/add-personas-user-stories`

The repository already contained a large set of user story documents with several inconsistencies. During the review, the following problems were identified:

- duplicate user story IDs
- mixed filename casing (`US-...` and `us-...`)
- underscores and hyphens used inconsistently in filenames
- mismatches between filename IDs and frontmatter `id`
- `iri` values not always aligned with the intended identifier scheme
- `meta.updated` needing refresh for touched files
- frontmatter `title` not always in Title Case
- first Markdown heading not always matching the frontmatter `title`
- named persona labels appearing in the story sentence instead of generic role labels

The immediate goal was not to let Codex improvise a repository-wide tidy-up, but to derive prompts that would constrain Codex tightly enough to do the work safely from the CLI.

## Hypothesis

If the task is decomposed into a sequence of narrow prompts with explicit scope, explicit non-goals, and explicit validation steps, Codex CLI can be guided to perform this kind of documentation normalisation more safely than with one large prompt.

## Method

The experiment was run as a chat-assisted prompt design session.

The interaction followed this pattern:

1. inspect the branch and story directory
2. identify concrete classes of inconsistency
3. derive a first Codex CLI prompt for identifier and filename normalisation
4. refine that prompt to touch only non-compliant files
5. derive a second prompt for title and heading alignment
6. derive a third prompt for persona-reference normalisation in the story sentence
7. create a planning package entry to record the session context
8. document the prompting strategy as an experiment

The prompts were deliberately written in English because:

- the repository content is English
- the filenames and metadata are English
- concepts such as `frontmatter`, `heading`, `Title Case`, and `preserve all other content` are concise and precise in English

This was not treated as a language requirement of Codex CLI, but as a practical choice to reduce ambiguity for English target artefacts.

## Prompt design principles observed during the session

Several principles turned out to be useful.

### 1. Scope must be stated repeatedly

The strongest prompts repeated the path and branch boundaries explicitly, for example:

- work on the current git branch only
- modify only files under `docs/en/architecture/stories`
- do not modify files outside that directory

This repetition looked slightly redundant, but it helped anchor the task.

### 2. Compliance rules work better than vague clean-up language

Prompts improved when they defined compliance rules instead of merely asking Codex to “normalise” files.

Examples:

- filename must match `US-XX-YY-ZZ-slug.md`
- `id` must exactly match the file ID
- `iri` must be exactly `arqix:user-stories/us-XX-YY-ZZ`
- the first Markdown heading must be exactly `## <title>`

That framing turned the task into rule enforcement rather than stylistic editing.

### 3. Change only non-compliant files

A useful refinement was to add a defensive instruction such as:

- change only files that violate one or more of the rules below
- do not touch already compliant files

This matters for documentation-heavy repositories because unnecessary edits create noisy diffs and reduce trust.

### 4. Separate structural passes from content passes

The session produced better prompts once the work was split into separate passes:

- structural metadata and filename normalisation
- title and first-heading alignment
- named persona replacement in the leading story sentence

This separation reduced the chance that Codex would perform broader rewrites than intended.

### 5. Examples and mappings reduce drift

The persona-replacement prompt became more deterministic once a concrete role mapping was included, for example:

- `Mara Maintainer -> maintainer`
- `Dan Developer -> developer`
- `Aria Architect -> architect`

Without explicit mapping, a model may produce acceptable but inconsistent variants.

## Prompt sequence produced in the session

The interaction produced three main Codex CLI prompt types.

### Prompt 1 — Identifier, filename, IRI, and metadata normalisation

Purpose:

- rename user story files to `US-XX-YY-ZZ-slug.md`
- keep `XX` and `YY`
- reassign `ZZ` to remove collisions and make numbering ascending within each group
- align `id`
- align `iri`
- set `meta.updated`

This prompt worked best when it also explicitly forbade body rewrites.

### Prompt 2 — Title and heading alignment

Purpose:

- enforce Title Case in frontmatter `title`
- ensure the first Markdown heading exactly matches the title
- replace placeholders such as `## User-story`

This was intentionally separated from filename and identifier work to keep the change set easier to reason about.

### Prompt 3 — Persona reference normalisation in story sentences

Purpose:

- replace named persona labels in the leading user story sentence
- use generic role references only
- preserve the rest of the sentence as much as possible

This prompt explicitly targeted the story sentence rather than the full body text.

## Observations

### The chat was more useful for diagnosis than direct implementation

The interaction was particularly strong at identifying classes of inconsistency and turning them into rules. It was less useful as a direct implementation channel because repository-wide document changes still benefited from running Codex in the actual CLI environment.

### Prompt decomposition was the key improvement

The quality of the resulting prompts improved sharply once the work was decomposed. The initial framing pointed towards a single broad normalisation pass. The later framing treated the work as a sequence of controlled transformations. That was a better fit for documentation governance work.

### Defensive wording mattered

Phrases such as these materially improved the expected safety profile:

- preserve all body text, acceptance criteria, and notes
- do not rewrite story content
- do not touch already compliant files
- verify that compliant files were left unchanged

These instructions are especially important when working with narrative Markdown rather than code.

### English prompting was the better practical choice here

For this specific experiment, English was the better prompt language because the target content, file naming scheme, and metadata vocabulary were all English. The gain was not about model capability in the abstract, but about keeping the instruction surface aligned with the target artefacts.

## Result

**Result: successful as a prompt-design experiment.**

The session produced a practical and staged Codex CLI prompting strategy for user story normalisation.

The strongest outcome was not a single perfect prompt, but a reusable pattern:

- diagnose first
- define compliance rules
- narrow the scope aggressively
- split structural and textual edits
- add explicit validations
- commit after each pass or at another reviewable boundary

This is a useful outcome for `arqix`, because the project is explicitly interested in disciplined documentation workflows rather than loose AI-assisted rewriting.

## Limitations

This experiment did not prove that every generated prompt will execute perfectly on first run.

It demonstrated something narrower and still useful:

- a human-guided chat session can produce better Codex CLI prompts than an ad hoc one-shot instruction
- prompt quality improves significantly once the repository constraints are made explicit
- documentation governance tasks benefit from decomposition more than from prompt verbosity alone

## Conclusion

This interaction suggests that Codex CLI works best for repository-wide documentation normalisation when it is treated as a constrained executor, not as an open-ended editor.

For `arqix`, that is a good sign.

The project does not need flamboyant automation. It needs automation that behaves predictably inside declared boundaries, leaves clean diffs, and respects the role of documentation as a governed artefact.

That was the real lesson of the session.
