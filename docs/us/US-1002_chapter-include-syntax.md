---
id: US-1002
kind: user_story
title: Chapter/Include Syntax
status: draft
tags:
- user-story
owner: hendrik
created: 2026-02-22
updated: 2026-02-22
priority: medium
related:
  requirements:
  - REQ-US-1002-01
  - REQ-US-1002-02
  - REQ-US-1002-03
  docs: []
  adrs: []
  personas:
  - PER-0001
lang: en
translation_of: US-1002
translation_status: draft
generated: false
source:
persona: PER-0001
old_id: US-0002
---
# Chapter/Include Syntax

## Story
As a maintainer, I want to use chapter and include directives in Markdown, so that I can structure documents declaratively and include units deterministically.

## Acceptance Criteria
- Directives `<!-- arqix:chapter ... -->` and `<!-- arqix:include ... -->` are parsed.
- Include targets are restricted to allowed roots via configuration.
- Glob includes are expanded deterministically (sorting via config).

## Notes
TODO
