---
title: "Roadmap"
description: "Planned work for arqix, in rough priority order"
date: 2026-03-20
status: active
---

# Roadmap

This document describes planned work for arqix. It is intentionally rough and will evolve as the project matures. Items are in approximate priority order, not a fixed schedule.

---

## Phase 0 — Foundation (current)

The goal of this phase is to establish a coherent, purposeful starting point without overbuilding.

- [x] Rust CLI skeleton that compiles and runs
- [x] Repository layout with docs/ tree
- [x] README, roadmap, AI transparency document
- [x] Initial blog post
- [ ] Basic `.gitignore` and project metadata complete

---

## Phase 1 — YAML frontmatter parsing

The first real feature: read a Markdown file and extract its YAML frontmatter.

- Parse YAML frontmatter from a single Markdown file
- Report missing or malformed frontmatter with a clear error
- Define the minimal set of required and optional fields
- Write tests for the parser

This phase establishes the core contract: metadata is intentional and machine-readable.

---

## Phase 2 — Document validation

With parsing in place, add a `validate` subcommand that checks a document or directory against defined schema rules.

- Required fields present and non-empty
- Known field values (e.g., `status` is one of a defined set)
- File naming conventions (optional, configurable)
- Exit codes suitable for use in CI pipelines

---

## Phase 3 — Document listing and indexing

A `list` subcommand that walks a docs directory and summarizes what it finds.

- List all documents with their title, date, and status
- Filter by status, date range, or directory
- Output as plain text and optionally as JSON or CSV

---

## Phase 4 — Document assembly

The `assemble` subcommand: take a set of document units and produce a larger output document.

- Define an assembly manifest (which units, in which order)
- Concatenate with optional section headers
- Deterministic output (same input always produces same output)

---

## Beyond phase 4

These are possibilities, not commitments:

- Traceability links between documents (requirements → decisions → tests)
- Graph-oriented views of document relationships
- Integration with Obsidian or other Markdown-first tools
- Self-hosting: run arqix on its own docs/ tree as a validation and indexing pass

---

## What this roadmap is not

This is not a promise. It is a direction. The project is early and solo. Priorities will shift based on what is actually useful.
