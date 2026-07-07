---
title: "Why arqix had to exist"
description: "A project post on the real tensions that led to arqix"
date: 2026-03-20
status: active
tags: [origin, motivation, documentation, architecture]
---

# Why arqix had to exist

Every tool exists because someone ran into a wall enough times.

The wall in this case is familiar to most engineers who have worked on mid-sized or long-running projects: documentation that starts clean and ends up a problem.
Not because anyone wanted it to.
Not because they were careless.
But because the tooling and the workflow did not make good structure sustainable.

This is an attempt to understand that wall, and to describe the thing that would help.

## The documentation drift problem

It usually starts fine.
There is a wiki, or a `docs/` folder, or a Confluence space.
Early decisions get written down.
Architecture diagrams get drawn.
A few ADRs get created in a burst of enthusiasm.

Then the project moves.
The code changes.
The architecture evolves.
And the documentation does not quite keep up — not because no one cares, but because the cost of keeping it synchronized is always slightly higher than the cost of letting it drift a little more.

Six months later, someone asks "where is the decision about the database schema?" and the answer involves searching three tools, reading three contradictory documents, and ultimately asking someone who was there.

This is not a documentation discipline problem.
It is a tooling and structure problem.

## Why Markdown is almost the answer

Plain Markdown with YAML frontmatter is already close to what you want:

- Human-readable without special tooling
- Version-controlled alongside code in Git
- Editable in any editor
- Structured enough to carry metadata if you use frontmatter consistently
- Compatible with a growing ecosystem of tools (Obsidian, mdBook, Pandoc, GitHub rendering)

The format is not the problem.
The problem is that nothing enforces structure, validates metadata, or helps you understand what documents exist, what state they are in, or how they relate to each other.

You can write `status: deprecated` in a YAML header, but nothing tells you it is deprecated when you stumble across it six months later in a search result.

## The recurring custom parser

Here is a pattern that appears in many projects: someone writes a small script to extract structure from the documentation.
Maybe it parses frontmatter to generate a changelog, or scrapes titles to build a navigation index, or checks for required fields in a CI step.

It works.
Then it breaks when the format changes.
Then it gets rewritten.
Then it gets abandoned when the person who wrote it moves to a different project.

The pattern repeats because the need is real, but no standard tool addresses it at the right level.
Static site generators solve the rendering problem, not the structure and validation problem.
Linters address code, not documentation.
Wiki tools add a heavy layer of infrastructure where plain files would have been enough.

## What arqix is trying to do

arqix is a bet that a small, focused CLI can fill that gap without adding significant overhead.

The core ideas:

- **Metadata as a contract.**
  YAML frontmatter is intentional, standardized, and machine-meaningful.
  Required fields are required.
  The tool can validate this.
- **Units, not monoliths.**
  Small Markdown documents are easier to review, easier to update, and easier to trace. arqix treats them as composable units that can be assembled into larger outputs when needed.
- **Structure without capture.**
  The documents remain plain Markdown files.
  You do not need arqix to read them.
  You do not need arqix to edit them.
  The tool adds value when you want validation, indexing, or assembly — not as a dependency for the format itself.
- **Git-friendly by default.**
  Documents are files.
  Files go in Git.
  Everything works with the tools engineers already use.

## Why now

Two things have made this more pressing recently.

First, AI assistance in writing and coding is now common enough that the volume of documentation people can produce has increased significantly.
More documentation means more potential drift.
Better structure helps manage that — both for humans and for the retrieval and context tasks that AI workflows increasingly depend on.

Second, there is a growing recognition that documentation quality is not just a human usability problem.
Documents that are well-structured, consistently formatted, and correctly linked are better for any system that needs to work with them: search, summarization, traceability, or whatever comes next.

## What arqix is not

It is not a documentation platform.
It is not a wiki engine.
It is not a static site generator.
It is not an AI tool.
It is not enterprise documentation management software.

It is a small, pragmatic CLI for engineers who already use Markdown and Git and want just enough structure to keep things coherent over time.

## A note on scope

arqix is deliberately minimal.
The plan is to build the smallest thing that is genuinely useful, and to grow it based on real use.
The project itself — its own documentation, its own decisions, its own notes — will serve as the first test corpus.

If it cannot organize its own documentation well, it has no business organizing anyone else's.
