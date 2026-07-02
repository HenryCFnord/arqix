```codex
Work on the current git branch only.

Use the planning package at:
docs/plans/story-normalization-session-2026-04-04/

Follow the conventions from:
docs/plans/README.md

Task:
Document the current session in this planning package.

Required inputs:
- docs/plans/README.md
- docs/plans/story-normalization-session-2026-04-04/IDEA.md

Create or update:
- docs/plans/story-normalization-session-2026-04-04/PLANS.md
- docs/plans/story-normalization-session-2026-04-04/STATUS.md

Keep IDEA.md as the original intake artefact.

The planning package should document the current session around normalization of user stories in:
docs/en/architecture/stories

The documented session must capture at least:
- branch context: docs/add-personas-user-stories
- scope: docs/en/architecture/stories
- identified issues:
  - duplicate user story IDs
  - inconsistent filename casing
  - underscores vs hyphens in filenames
  - filename ID vs frontmatter id mismatches
  - incorrect iri values
  - meta.updated needing refresh on touched files
  - frontmatter title not consistently in Title Case
  - first Markdown heading not matching the frontmatter title
  - named persona references in user story sentences instead of generic roles
- intended normalization rules
- expected validation checks
- risks / caveats
- suggested execution strategy

Also document that three Codex prompts were prepared during the session:
1. a normalization prompt for filenames, id, iri, and meta.updated
2. a normalization prompt for title and first heading alignment
3. a normalization prompt for replacing named persona labels with generic role references in the user story sentence

In the planning package:
- mention these three prompts as session outputs or execution artefacts
- summarize their purpose clearly
- do not invent file names for stored prompt files unless you actually create them
- do not claim the prompts were executed unless that is explicitly supported by the current branch state
- distinguish clearly between analysis/planning work and implementation work

Content guidance:
- PLANS.md should be the reviewed execution plan
- STATUS.md should capture current state, next recommended action, blockers, and notes
- Keep the package concise and easy to review on mobile
- Preserve the intent of docs/plans/README.md

Constraints:
- Modify only files inside docs/plans/story-normalization-session-2026-04-04/
- Do not modify implementation files
- Do not rewrite IDEA.md into a plan
- Do not claim completion of normalization work unless it actually happened in the repo

After editing:
- verify the planning package contains IDEA.md, PLANS.md, and STATUS.md
- verify the package is consistent with docs/plans/README.md
- verify the three prepared prompt types are mentioned accurately
- create a git commit with this message:

Document story normalization planning package
````