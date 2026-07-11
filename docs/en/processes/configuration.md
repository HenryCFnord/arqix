# Configuration schema v1

`arqix` resolves its effective configuration from built-in defaults plus overrides in `arqix.toml` in the working directory (REQ-00-00-00-06).
A missing file is valid — it simply means no overrides.
`arqix config show` renders exactly what commands act on; `arqix config validate` checks the file against this schema.

## Keys

| Key | Type | Default | Validated in v1 |
| --- | --- | --- | --- |
| `roots` | array of strings | `["docs"]` | yes — type-checked |
| `skip-dirs` | array of strings | `[".git", "target", "node_modules", "__pycache__", "fixtures"]` | yes — type-checked |
| `kinds` | table | empty | accepted, content validated in a later schema version |
| `templates` | table | empty | accepted, content validated in a later schema version |
| `policies` | table | empty | accepted; `policies.verify` and `policies.publish` are read (below), other content validated in a later schema version |
| `i18n` | table | empty | accepted; `default-lang` is read (below), other content validated in a later schema version |

Unknown top-level keys are ignored with a warning — forward compatibility for configs written against newer schema versions.

`skip-dirs` governs document discovery (the store walk under `roots`: `doc list/read/search`, lint, fmt, assemble).
The trace corpus walk keeps its fixed skip set, mirroring the Python oracle for conformance (REQ-01-01-17-01).

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
- `ratchet` (`trace ratchet [--baseline <path>]`) gates coverage *regressions* against the committed matrix snapshot: a requirement the baseline lists as verified must still be verified by an active test, unless it is retired or gone — a declared specification change is never a regression. Growth stays free; a missing baseline compares nothing and passes.

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
