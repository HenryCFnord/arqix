---
title: "OpenClaw mobile-first planning and delivery flow"
date: 2026-03-24
status: draft
---

# OpenClaw mobile-first planning and delivery flow for arqix

## Status

Preferred OpenClaw workflow for `arqix`.

This document defines the current preferred OpenClaw workflow for `arqix`.

## Problem statement

The older flow assumed that a handoff document had to exist in the repository before OpenClaw or Codex could act on it.

That creates unnecessary friction for a mobile-first workflow:

- a draft idea must first be turned into a repository artifact,
- that artifact must be committed before a branch can be created from it,
- and only then can the actual preparation workflow begin.

For smartphone-driven work, this is too cumbersome.

## Design goal

Make the flow easy to start from a phone while keeping the repository as the source of truth as early as possible.

The new flow should allow:

1. a free-text idea to be sent to OpenClaw,
2. OpenClaw to classify the work and create a branch,
3. OpenClaw to generate draft planning artifacts on that branch,
4. the human to review and refine the plan on the phone,
5. OpenClaw to run Codex only after the reviewed plan has been pushed,
6. OpenClaw to notify the user and create a draft PR at the correct time.

## Core principles

- mobile-first intake
- repository artifacts created early
- clear human review checkpoints
- no direct implementation on `main`
- branch-first, plan-first, implementation-second
- OpenClaw orchestrates
- Codex implements
- GitHub remains the source of truth
