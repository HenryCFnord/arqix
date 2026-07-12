# Configuration schema v1

`arqix` resolves its effective configuration from built-in defaults plus overrides in `arqix.toml` in the working directory (REQ-00-00-00-06).
A missing file is valid — it simply means no overrides.
`arqix config show` renders exactly what commands act on; `arqix config validate` checks the file against this schema.

## Keys

| Key | Type | Default | Validated in v1 |
| --- | --- | --- | --- |
| `roots` | array of strings | `["docs"]` | yes — type-checked |
| `skip-dirs` | array of strings | `[".git", "target", "node_modules", "__pycache__", "fixtures"]` | yes — type-checked |
| `kinds` | table | empty | accepted; per-family contract entries are read (below), other content validated in a later schema version |
| `templates` | table | empty | accepted; `dir` is read (below), other content validated in a later schema version |
| `policies` | table | empty | accepted; `policies.verify` and `policies.publish` are read (below), other content validated in a later schema version |
| `i18n` | table | empty | accepted; `default-lang` is read (below), other content validated in a later schema version |

Unknown top-level keys are ignored with a warning — forward compatibility for configs written against newer schema versions.

`skip-dirs` governs document discovery (the store walk under `roots`: `doc list/read/search`, lint, fmt, assemble).
The trace corpus walk keeps its fixed skip set, mirroring the Python oracle for conformance (REQ-01-01-17-01).

## The template directory

`[templates] dir` names the directory `doc new` and `unit new` instantiate template files from (`<kind>.tpl.md`, placeholders `{id}`, `{title}`, `{slug}` — REQ-01-01-20-01/-03):

```toml
[templates]
dir = "docs/templates"
```

- A configured but missing template file is a config error: the diagnostic names the expected path.
- Unconfigured, the engine reads the package-local `templates/` that `doc init` scaffolds, and falls back to the embedded default — an unconfigured repository produces byte-identical documents.
- `doc init` scaffolds the default template files into the template directory and never overwrites a shaped one (REQ-01-01-20-02).

## The frontmatter contracts

`[kinds.<family>]` declares one document family's frontmatter contract — the one source the formatter and the validators share (ADR-0011, REQ-01-01-19-01/-02/-03):

```toml
[kinds.note]
dir = "docs/notes"
key-order = ["title", "id", "rdf", "meta"]
required = ["title", "id"]
required-meta = ["lang"]
```

- `dir` — the directory that selects the family (longest match wins); an entry without it cannot be matched and is skipped.
- `key-order` — the canonical top-level key order `fmt` applies and the frontmatter checker validates (FMT-003); unknown keys are findings (FMT-004).
- `required` — keys that must be present and non-empty (FM-001).
- `required-meta` — the family's required `meta` keys; unset families keep the built-in set.
- `id-pattern` — the family's ID shape as a regex (ADR-0012, REQ-01-01-18-01..04): the checkers validate shape (FM-002, LNT-006), `doc new` mints the next ID from the `(?P<seq>…)` group where the surrounding pattern is literal, and a `(?P<story>…)` group activates the consistency check against the first declared `derived-from` triple (LNT-007).
  The ID stays an opaque label: relations always come from the declared triples, and a group-free pattern yields a complete trace graph.
- Without configuration, the built-in defaults reproduce the present contract — `fmt` stays byte-identical on an unconfigured corpus.
- Configured families carry no built-in id/iri shape rules; ID shapes become configuration with the ID-policy story (US-01-01-18).

## The verify policy

`[policies.verify]` declares which sub-steps `arqix verify` runs and how each result is treated (REQ-04-01-14-01/-02/-03):

```toml
[policies.verify]
steps = ["format", "lint", "trace-scan", "coverage", "ratchet"]
informational = ["coverage"]
```

- `steps` — the sub-steps to run, in order; the known names are `format`, `lint`, `trace-scan`, `coverage`, and `ratchet`. An unknown name is a usage error (exit 2).
- `informational` — steps whose findings are reported without affecting the exit code. Informational forgives findings (exit 1) only, never system errors: a crashed sub-step fails the loop either way.
- The values above are the defaults: coverage measures project progress and must never gate a change by default; everything else gates.
- `ratchet-baseline` — the baseline file the ratchet compares against (REQ-04-01-16-01, config-audit row C17); unset, the committed default snapshot location applies, and an explicit `--baseline` argument overrides both.
- `ratchet` (`trace ratchet [--baseline <path>]`) gates coverage *regressions* against the committed matrix snapshot: a requirement the baseline lists as verified must still be verified by an active test, unless it is retired or gone — a declared specification change is never a regression. Growth stays free; a missing baseline compares nothing and passes.

## The assemble policy

`[policies.assemble]` governs stitching (ADR-0013, REQ-02-01-12-04):

```toml
[policies.assemble]
heading-ownership = "child"
```

- `heading-ownership` — who owns section headings in a stitched corpus.
  `child` (the default): fragments own their headings, and a bare `<!-- arqix:include <path> -->` behaves as `level=+1` under the heading in effect at the include position.
  `parent`: the page declares the outline, fragments are authored headingless, and a bare include inlines verbatim.
- The include directive's optional level argument overrides the default per include: `level=N` places the fragment's first heading at level N (1–6), `level=+N` places it N levels below the heading in effect.
  The delta applies to every heading of the fragment; a shift out of the h1–h6 range fails the assembly (ASM-005).
- Relative links inside included fragments are rebased to the including page's location during assembly, so assembled pages stay artefact-ready.

## The reports policy

`[policies.reports]` governs how the reference sequencer's freshness gate treats the committed report snapshots (config-audit row C17):

```toml
[policies.reports]
snapshot-strategy = "committed"
```

- `committed` (the default) — snapshots are committed and the freshness gate runs everywhere; parallel branches rebase and regenerate before merging.
- `main-only` — the freshness gate runs only on the default branch; parallel branches skip it, trading merge friction for a regeneration step on the default branch after merging.
  How the default branch regains freshness is a per-repository choice: an auto-commit step in CI (the gate regenerates and pushes the snapshots before verifying), or the next change brings the regeneration along (cheaper, but the reports lag one step between merges).
- `on-demand` — the freshness gate never runs; snapshots are regenerated when wanted.

The strategy is read by the reference sequencer (`scripts/arqix verify`); the product's own `arqix verify` does not gate snapshot freshness.
This repository runs `main-only` with the auto-commit variant (owner decision 2026-07-12): the gate workflow refreshes and commits the snapshots on the default branch before verifying, and parallel branches never touch `docs/en/reports/`.

## The render policy

`[policies.render]` governs `render pdf` (REQ-04-01-03-04..08):

```toml
[policies.render]
pdf-command = "pandoc"
defaults = "pandoc-defaults.yaml"
template = "eisvogel"
artefact-mode = "package"

[policies.render.package.docs]
template = "another-template"
```

- `pdf-command` — the renderer invocation (`pandoc` by default); arqix appends the input pages, `-o <target>`, and the configured options, and forwards the tool's errors transparently as exit 2.
- `defaults` — a Pandoc `--defaults` file, passed through when configured.
- `template` — a Pandoc `--template` value (e.g. `eisvogel`), passed through when configured.
- `artefact-mode` — where the artefact lands: `package` (the default) stores `<package>.pdf` into the package's `artefacts/` directory; `detached` stores into `artefact-dir` (default `render-out`).
- `[policies.render.package.<name>]` — per-doc-package overrides: any key above, winning over the global table for that package.
- Inputs are the staged artefact-ready pages of the language root (the publish `exclude` scope holds), or the Markdown files given on the command line; an explicit `--out` overrides the artefact mode.

## The publish policy

`[policies.publish]` and `[i18n]` govern `publish site` (REQ-04-01-03-01/-02/-03, REQ-04-01-07-01/-02):

```toml
[i18n]
default-lang = "en"

[policies.publish]
staging-dir = "site-src"
stitching = "single-page"
site-command = "zensical build"
```

- arqix **stages and orchestrates — it never renders**: rendering is the site toolchain's job, exactly as Pandoc renders PDF. There is no built-in renderer and no fallback; a publish without a configured `site-command` is a config error (exit 2).
- `staging-dir` — where artefact-ready inputs are staged (default `site-src`; belongs in `.gitignore`). The default language stages to its root, every other language to `staging-dir/<lang>/`; the language root is `<root>/<lang>` where the layout has one, the bare root for the default language.
- Staged pages are artefact-ready: includes expanded (single-page stitching), directives and marker comments removed, the arqix frontmatter reduced to the toolchain-consumable part (`title`) — which also keeps staged copies out of document discovery and the trace graph.
- `stitching` — `single-page` (v1: the assembled document is the unit of publication). The `split` mode on the assembled outline is decided and lands with the ADR-0013 assembler slice.
- `exclude` — language-root-relative path prefixes that never stage (the publish scope): internal corpus areas stay off the public site. The lifecycle-based `final` filter (ADR-0010) is the specified successor for prose documents.
- `site-command` — the toolchain invocation (whitespace-split, stdio inherited), run after staging. Recommended: a pinned [Zensical](https://zensical.org) (or MkDocs) invocation whose `docs_dir` points at the staging dir; pin the version in CI for reproducible publishes. A failing or unrunnable command is a system error: exit 2 with a diagnostic naming the invocation.

## Diagnostics

| Code | Severity | Meaning |
| --- | --- | --- |
| `CFG-001` | error | schema violation — the message names the failing key and the expected type |
| `CFG-002` | warning | unknown top-level key, ignored |

Diagnostics follow the tool-wide shape (severity, stable code, message, file — REQ-00-00-00-03) and carry `schema_version`; only errors drive exit code 1.
