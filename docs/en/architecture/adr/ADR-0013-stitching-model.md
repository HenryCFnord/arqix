---
id: ADR-0013
title: Stitching Model
slug: stitching-model
iri: arqix:adrs/adr-0013

rdf:
  type:
    - arqix:classes/adr

triples:
  - predicate: arqix:properties/guides-design-of
    object:
      - arqix:requirements/req-02-01-12-01
  - predicate: arqix:properties/guides-implementation-of
    object:
      - arqix:requirements/req-02-01-12-02
      - arqix:requirements/req-02-01-12-03
      - arqix:requirements/req-02-01-12-04

properties:
  decision-status: accepted

external-references: []

meta:
  lifecycle-status: draft
  owner: hcf
  created: 2026-07-11
  updated: 2026-07-14
  lang: en
  translation-of:
  generated: false
---

## Stitching Model

### Context

Pages are stitched from units by include expansion, and units have arbitrary granularity: a unit can be a chapter, a subchapter, or a single paragraph.
A fragment therefore cannot know the outline position of every place it is included — the same unit may sit one level deep in one page and three levels deep in another.
The v1 assembler inlines fragments verbatim, so the assembled outline is an accident of authoring: a fragment authored at `##` is wrong everywhere except at exactly that depth, and reuse at two depths cannot be right in both.
On top of that sits a recurring authoring debate: does the section heading belong to the parent (the page declares its outline and pulls headingless fragments) or to the child (the fragment owns its title and the surrounding outline adapts)?
Both are legitimate corpus styles, but mixing them silently produces broken outlines — and every downstream consumer needs a coherent one: the site's anchors and table of contents, any outline-based page split, and Pandoc's `--toc` for PDF.

### Decision

<!-- arqix:references-artefact arqix:requirements/req-02-01-12-01 -->
**The include directive declares where a fragment's headings land; the assembler re-levels the whole fragment to fit.**

```markdown
<!-- arqix:include units/unit-x.md level=3 -->   absolute: the fragment's first heading becomes h3
<!-- arqix:include units/unit-x.md level=+1 -->  relative: one level below the heading in effect here
```

- The delta between the declared level and the fragment's first heading applies to **every** heading in the fragment — internal structure is preserved, only shifted.
- A relative level resolves against the heading level in effect at the include position (the last heading the assembler has seen in the parent; before any heading, the base is zero, so `+1` yields h1).
  Moving an include into a deeper section re-levels the fragment without touching it.
- **Heading ownership is a corpus policy**, not a per-include patch: `[policies.assemble] heading-ownership = "child"` (the default — fragments own their headings, a bare include behaves as `level=+1`) or `"parent"` (fragments are authored headingless and the page declares the outline; a level argument then governs only a fragment's internal headings).
  The parent style is validated by the per-family frontmatter contract (US-01-01-19): whether a body must start with a heading is exactly the contract knob that strand configures.
- A shift beyond h6 is a structural error (ASM-005) naming the fragment and the heading — never a silent clamp.
- **Splitting happens on the assembled outline, never on fragment boundaries**: units are not chapters, so the site's `split` stitching mode cuts at a configured heading depth (`split-level`) of the assembled document.
- **PDF renders one artefact per top-level document**, not one per package or root: a document is a content family (a directory with an `index.md`, whose subtree is collected) or a standalone top-level page, and its boundaries are declared by `[policies.render] documents` (or auto-discovered from the language root when that list is absent).
  Each document is staged body-only.
  A family's `index.md` landing page is not staged as a chapter when the family carries other content: it is a site-navigation stub, and its title becomes the document title rather than a near-empty opening chapter.
  Staging then depends on whether the document is a single work or a collection.
  A single-document page — a unit family's assembled page, or a standalone page — drops its own leading title heading (it duplicates the document title, which rides the title page as metadata) and re-levels the remaining body so its first section lands at H1, opening the body at the actual chapters rather than nesting them under a repeated wrapper title.
  A collection — several sibling files under one directory, like the decision records or the blog — instead keeps each member's own title as its H1 chapter, with the member's sections re-levelled below it, so each ADR or post survives as an identifiable chapter; a document staging more than one member is treated as a collection.
  The document title is passed to Pandoc as explicit metadata and rendered on the eisvogel title page and running header.
  Within a document the assembled outline remains a single stitching source shared with the site, and `--toc` derives from it.
- The `arqix:chapter` directive is retired from the grammar: the level argument supplies the semantics it never had, and chapter identity remains what it already is — frontmatter ids.

### Alternatives Considered

- **Verbatim inlining (the status quo):** rejected — the outline becomes an accident of where a fragment happens to be included, and a unit reused at two depths cannot be correct in both.
- **Splitting at include boundaries:** rejected — units have arbitrary granularity; equating fragments with chapters bakes a false assumption into the site structure.
- **Suppressing the child's heading where the parent owns it:** rejected — mechanically discarding content is the opposite of the project's generated-artefact discipline; ownership is declared once per corpus and validated, not patched per include.
- **Giving `arqix:chapter` the level semantics instead:** rejected — a second numbering mechanism next to the include argument invites divergence, and the directive has been decorative since the assembler shipped.

### Consequences

- The include grammar gains the optional `level` argument; REQ-02-01-09-01 is reworded to the include-plus-level grammar and the chapter directive leaves it.
- The assembler gains the re-levelling walk, the ASM-005 overflow diagnostic, and the resolved level in each assembly-log record (reviewability).
- `[policies.assemble] heading-ownership` joins the configuration; parent-style corpora also configure the body-heading rule of their frontmatter contract off (US-01-01-19 coupling).
- The publisher's `stitching = "split"` mode with `split-level` becomes implementable on the assembled outline once this slice ships; until then `single-page` remains the only mode.
- US-02-01-12 carries the implementation; existing corpora are untouched — a bare include under the `child` default behaves as today's corpus expects (`+1` under the page's section headings).
