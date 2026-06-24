```codex
Work on the current git branch only.

Modify only files under docs/en/architecture/stories.
Change only files that violate one or more of the rules below.
Do not touch already compliant files.

Compliance rules:

1. Filename format
- Filename must match: US-XX-YY-ZZ-slug.md
- US must be uppercase
- XX and YY must remain unchanged
- ZZ must be unique and strictly ascending within each XX-YY group
- Use hyphens only, never underscores

2. Frontmatter
- id must exactly match the file ID, e.g. US-01-01-08
- iri must be exactly: arqix:user-stories/us-XX-YY-ZZ
- meta.updated must be 2026-04-04
- title must be in proper Title Case

3. Markdown heading
- The first Markdown heading must be exactly: ## <title>
- It must exactly match the frontmatter title
- Replace incorrect headings such as ## User-story

Change policy:
- Preserve XX and YY
- Reassign ZZ only where needed to remove collisions and produce a clean ascending sequence within each XX-YY group
- Keep the slug content as close as possible to the current filename slug
- Preserve all body text, acceptance criteria, and notes
- Do not rewrite story content
- Do not modify files outside docs/en/architecture/stories

Before editing, identify the non-compliant files.
Then update only those files.

After applying changes:
- verify that no duplicate user story IDs remain in docs/en/architecture/stories
- verify all touched files still have valid frontmatter structure
- verify all touched files have Title Case titles
- verify all touched files begin with a first heading that exactly matches the frontmatter title
- verify no compliant file was changed unnecessarily

Then create a git commit with this message:
Normalize non-compliant user story metadata and headings

If a file is already compliant, leave it byte-for-byte unchanged.
```