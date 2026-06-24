```codex
Work on the current git branch only.

In docs/en/architecture/stories, normalize all user story files to this filename format:

US-XX-YY-ZZ-slug.md

Rules:
- Keep XX and YY unchanged.
- Reassign ZZ so that within each XX-YY group the sequence is strictly ascending and collision-free.
- Use uppercase US in filenames and ids.
- Use hyphens only; replace any underscores in filenames with hyphens.
- Keep the slug content as close as possible to the current filename slug, only normalize case/format where needed.

For every modified file:
- Set frontmatter id to the exact file ID, e.g. US-01-01-08
- Set frontmatter iri to arqix:user-stories/us-XX-YY-ZZ
- Set frontmatter meta.updated to 2026-04-04
- Preserve all other content unless required by the rename/id normalization

Also fix cases where filename ID and frontmatter id currently disagree.

After applying changes:
- verify that no duplicate user story IDs remain in docs/en/architecture/stories
- verify all touched files still have valid frontmatter structure
- create a git commit with this message:

Normalize user story IDs and filenames
```