# Roadmap

`arqix` is in an exploratory build phase. The roadmap is deliberately short so the project can learn from real use instead of locking itself into a premature architecture.

## Phase 0: Repository Foundation

- establish the repository structure
- write the initial corpus in normal Markdown
- keep the tool scope intentionally narrow

## Phase 1: Parse and Validate

- read Markdown files with YAML frontmatter
- extract normalized metadata
- validate required fields and basic structural rules

## Phase 2: Units and Assembly

- define a small concept of document units
- assemble ordered units into deterministic outputs
- preserve stable headings and output ordering

## Phase 3: Traceability Basics

- support explicit references between units
- check unresolved references
- expose simple traceability reports

## Out of Scope for Now

- plugins
- multi-crate decomposition without real pressure
- GUI work
- database-backed graph infrastructure
- enterprise workflow automation
