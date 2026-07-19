---
title: "Three rules instead of a thousand examples"
description: "What property-based testing found — and did not find — in a documentation compiler"
date: 2026-07-19
status: draft
tags: [testing, property-based-testing, proptest, verification]
---

# Three rules instead of a thousand examples

The usual test checks examples you thought of.
That is exactly why it can be falsely green: it only checks what you thought of.
A rule test turns this around — you state the rule, and the tool invents inputs and tries to break it.

arqix now carries three such rules, added one at a time, each a single afternoon.
This post says what each rule means here, what running it actually produced, and where the approach earns its keep — and where it does not.

## The idea, without the vocabulary

Category theory is only the source of one question: **what must stay true when I transform something?**
For a documentation compiler — a tool that parses, reorders, stitches, and exports Markdown — that question has three obvious answers.

1. **Order does not matter.** Assembling and then exporting must equal exporting and then assembling.
2. **There and back.** Exporting and re-reading yields the original — or you know exactly what is lost.
3. **Nothing vanishes.** Every statement from a unit appears in the assembled document.

Three sentences, all three machine-checkable.
In Rust the tool is `proptest`: it generates inputs from a strategy, runs hundreds of cases, and when one fails it shrinks the input to the smallest counterexample.

## Rule 1 — order does not matter

The concrete instance sits at the assemble-to-export seam.
Assembling moves a line from its unit into a page in another directory; every relative link in that line is rebased so it still works.
The rule: for **every** in-repository link and **every** move, the rebased link must reach exactly the file the original link reached, anchor included.

The property generates directories, `../` chains, path segments, and anchors, and uses the path semantics itself as the oracle — there is no table of expected outputs to get wrong.
It held on the first run, 256 cases per property.

What did we gain if nothing broke?
Before the test, "rebasing is correct" was a conjecture supported by a handful of authored examples.
After it, the statement is: the rule holds over the generated input space.
That is a different epistemic category, and it cost an afternoon.

## Rule 2 — there and back

Two faces of the same rule, both on the formatter.

The first is the byte-lossless split: frontmatter parsing decomposes a document into the opening fence, raw lines, and the verbatim rest, and composing the three back must yield the exact input — CRLF line endings, trailing whitespace, and unicode included.
The named loss is: none.
That *is* the formatter's contract, and now it is checked against inputs nobody authored.

The second face is idempotence: whatever one `fmt` pass normalizes must be at rest afterwards — a second pass changes nothing, for every generated document.
Idempotence is the round trip of a normalizer: the target of the journey is the fixed point.

Both held on the first run.

## Rule 3 — nothing vanishes

The assembler stitches units into pages, re-leveling headings on the way.
The rule: every line of every included unit appears in the assembled page — prose verbatim, headings under some re-leveled depth, and lines inside code fences **strictly verbatim**, because the heading shift must never reach inside a fence.
Unit frontmatter, by contract, is the one thing that must *not* appear.

This property could not run as a pure function test, and the reason is the honest finding of the exercise: the first attempt was rejected by the assembler's own containment guards, which refuse include targets outside the working corpus.
The guards are correct — so the property became an end-to-end test that builds a fresh scratch corpus per case and runs the real binary against it.
Thirty-two generated corpora per run; the rule held.

## The evaluation

**What held is not wasted.**
All three rules passed, and each passing run converted a belief into a checked statement.
The counterfactual matters too: while writing rule 1 for the claim-lifting feature earlier the same week, the equivalent line-anchoring assumption *did* break — a marker quoted in prose was treated as a real one — and it was generated-input thinking, not an authored example, that surfaced it.

**Rules complement example suites; they do not replace them.**
arqix's checker contracts are pinned by mirrored example fixtures, and those stay: an example pins one exact behaviour forever, which is what a ported contract needs.
The rules cover the space between the examples, which is where false green lives.

**The economics are uneven, and that is fine.**
On pure functions — path rebasing, splitting, formatting — a property costs an afternoon and runs in milliseconds.
On filesystem pipelines it costs structure: a scratch corpus per case, a real binary, guards to respect.
The gradient is useful information in itself: the harder a property is to state, the more the code under it is entangled with its environment.

**One rule at a time.**
The discipline of landing each rule separately is not ceremony: when a property goes red, the diagnosis is only cheap if exactly one assumption is on trial.

**Honest limits.**
A generator encodes assumptions just as an example does — the alphabet of segments, the depth of nesting, the shapes we thought to generate.
A held property is strong evidence over its input space, not proof beyond it.
The difference from example testing is that the space is orders of magnitude larger and its boundary is written down in one place, where it can be widened.

## Where this goes

The three rules now run inside the same gate as everything else — one `just ci`, no special ceremony.
The next candidates follow the same question: what must stay true when the corpus is published, when a claim is lifted into the graph, when a snapshot regenerates?
Every one of those is a sentence first, a strategy second, and an afternoon third.
