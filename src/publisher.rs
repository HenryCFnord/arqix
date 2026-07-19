//! Publish & Render Orchestrator, site half: `publish site` stages
//! artefact-ready inputs and orchestrates the configured site toolchain —
//! it never renders HTML itself, exactly as `render pdf` orchestrates
//! Pandoc instead of rendering PDF (owner decision 2026-07-11). Staging is
//! per language root (REQ-04-01-07-01): includes expanded, directives gone,
//! the arqix frontmatter reduced to the toolchain-consumable part
//! (REQ-04-01-03-02). Toolchain failures are system errors, exit 2, never
//! findings (REQ-04-01-07-02). Like the assembler, it only writes generated
//! artefacts — sources are never mutated (ADR-0004).

use crate::OutputFormat;
use crate::diag::SCHEMA_VERSION;
use serde_json::json;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

// arqix:implements REQ-04-01-03-01
// arqix:implements REQ-04-01-03-02
// arqix:implements REQ-04-01-03-03
// arqix:implements REQ-04-01-07-01
// arqix:implements REQ-04-01-07-02
/// `arqix publish site [--lang <lang>]`
pub fn site(lang: Option<&str>, format: OutputFormat) -> ExitCode {
    let policy = crate::config::publish_policy(Path::new("."));
    // v1 stitches single-page: the assembled document is the unit of
    // publication. The split-on-outline mode is decided (stitching
    // discussion 2026-07-11) and lands with the ADR-0013 assembler slice.
    if policy.stitching != "single-page" {
        eprintln!(
            "error: [policies.publish] stitching '{}' is not available yet: v1 stitches single-page (split lands with the ADR-0013 slice)",
            policy.stitching
        );
        return ExitCode::from(2);
    }
    // No built-in renderer and no fallback: rendering is the toolchain's
    // job, and publishing without one is a config error — exactly like
    // `render pdf` without Pandoc.
    let Some(command) = &policy.site_command else {
        eprintln!(
            "error: no site toolchain configured: set [policies.publish] site-command (e.g. a pinned Zensical or MkDocs invocation; see docs/en/processes/configuration.md)"
        );
        return ExitCode::from(2);
    };

    let default_lang = crate::config::default_lang(Path::new("."));
    let lang = lang.unwrap_or(&default_lang);
    // The language's staging target: the default language owns the staging
    // root, every other language its own subdirectory (REQ-04-01-07-01).
    let staging = if lang == default_lang {
        PathBuf::from(&policy.staging_dir)
    } else {
        Path::new(&policy.staging_dir).join(lang)
    };

    // Staging is generated output: a stale tree makes a local build disagree
    // with a fresh CI checkout (REQ-04-01-07-05).
    // arqix:implements REQ-04-01-07-05
    let _ = std::fs::remove_dir_all(&staging);

    let mut staged = 0usize;
    let skip = crate::config::skip_dirs(Path::new("."));
    // Fragments pulled in by an `arqix:include` reach the site through their
    // parent page and are not staged as standalone pages — unless a corpus
    // page links to them: the landing page embeds the scoreboard unit, the
    // report catalog links it, so it serves both roles (REQ-04-01-07-03).
    let included = included_targets(&skip);
    let linked = linked_targets(&skip);
    for root in crate::config::roots(Path::new(".")) {
        // The language's root: <root>/<lang> where the layout has one; the
        // bare root serves the default language otherwise.
        let lang_root = Path::new(&root).join(lang);
        let lang_root = if lang_root.is_dir() {
            lang_root
        } else if lang == default_lang {
            PathBuf::from(&root)
        } else {
            continue;
        };

        let mut files = Vec::new();
        crate::util::collect_markdown(&lang_root, &skip, &mut files);
        for file in files {
            // Skip a fragment another page stitches in — it is published
            // through that page, not as its own page (REQ-04-01-07-03).
            // arqix:implements REQ-04-01-07-03
            if std::fs::canonicalize(&file)
                .map(|c| included.contains(&c) && !linked.contains(&c))
                .unwrap_or(false)
            {
                continue;
            }
            let assembled = match crate::assembler::expand_document(&file) {
                Ok(text) => text,
                Err(diagnostic) => {
                    eprintln!(
                        "error: {}: {}",
                        diagnostic.file.as_deref().unwrap_or("?"),
                        diagnostic.message
                    );
                    return ExitCode::from(2);
                }
            };
            // Relative CSV links point at the staged table pages on the
            // site; the committed page keeps linking the raw CSV.
            // arqix:implements REQ-04-01-19-02
            let assembled = rewrite_csv_links(&assembled);
            let doc = crate::parser::parse(&file.to_string_lossy(), &assembled);
            let rel = file.strip_prefix(&lang_root).unwrap_or(&file).to_path_buf();
            // The publish scope: excluded subtrees never leave the repo.
            let rel_posix = crate::util::to_posix(&rel);
            if policy.exclude.iter().any(|e| {
                let prefix = e.trim_end_matches('/');
                rel_posix == prefix || rel_posix.starts_with(&format!("{prefix}/"))
            }) {
                continue;
            }
            if let Err(code) = write_staged(&staging.join(&rel), &doc, StagingMode::Site, false) {
                return code;
            }
            staged += 1;
        }

        // The site toolchain renders Markdown only, so every corpus CSV is
        // staged as a generated table page at its corpus location — the CSV
        // stays the single committed artefact (US-04-01-19).
        // arqix:implements REQ-04-01-19-01
        let mut csvs = Vec::new();
        collect_csv(&lang_root, &skip, &mut csvs);
        for file in csvs {
            let rel = file.strip_prefix(&lang_root).unwrap_or(&file).to_path_buf();
            let rel_posix = crate::util::to_posix(&rel);
            if policy.exclude.iter().any(|e| {
                let prefix = e.trim_end_matches('/');
                rel_posix == prefix || rel_posix.starts_with(&format!("{prefix}/"))
            }) {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&file) else {
                continue;
            };
            let page = csv_table_page(&file.to_string_lossy(), &text);
            let target = staging.join(rel.with_extension("md"));
            if let Some(parent) = target.parent()
                && let Err(err) = std::fs::create_dir_all(parent)
            {
                eprintln!("error: cannot create {}: {err}", parent.display());
                return ExitCode::from(2);
            }
            if let Err(err) = std::fs::write(&target, page) {
                eprintln!("error: cannot write {}: {err}", target.display());
                return ExitCode::from(2);
            }
            staged += 1;
        }
    }

    // The specification catalogue: the excluded story/requirement sources
    // return in bundled form — one generated page per workflow group
    // (US-04-01-17). Default language only: the specification sources are
    // authored in the corpus language.
    if policy.specification_catalogue
        && lang == default_lang
        && let Err(code) = stage_catalogue(&staging)
    {
        return code;
    }

    // The graph explorer stages with the site (ADR-0020): regenerated from
    // the current corpus, never committed. Default language only — the page
    // projects the whole corpus, not a translation.
    // arqix:implements REQ-08-01-42-03
    if lang == default_lang {
        let target = staging.join("graph.html");
        if let Err(err) = std::fs::write(&target, crate::reporter::graph_page()) {
            eprintln!("error: cannot write {}: {err}", target.display());
            return ExitCode::from(2);
        }
        staged += 1;
    }

    // Configured assets ride along so the toolchain can reference them. An
    // asset under a doc root's language dir is staged at the same
    // language-root-relative path as the pages (which strip that prefix), so a
    // page's relative link to it resolves (REQ-04-01-07-04); anything else —
    // e.g. brand assets under `assets/` — is staged verbatim.
    for asset in &policy.assets {
        let source = Path::new(asset);
        let target = PathBuf::from(&policy.staging_dir).join(staged_asset_rel(asset, lang));
        if let Err(code) = copy_asset(source, &target) {
            return code;
        }
    }

    // Orchestrate the toolchain over the staged inputs, inheriting stdio so
    // its own output and errors surface transparently.
    let mut parts = command.split_whitespace();
    let Some(program) = parts.next() else {
        eprintln!("error: [policies.publish] site-command is empty");
        return ExitCode::from(2);
    };
    match Command::new(program).args(parts).status() {
        Ok(status) if status.success() => {}
        Ok(status) => {
            eprintln!(
                "error: site toolchain failed: '{command}' exited with {}",
                status
                    .code()
                    .map_or("signal".to_string(), |c| c.to_string())
            );
            return ExitCode::from(2);
        }
        Err(err) => {
            eprintln!("error: site toolchain failed: cannot run '{command}': {err}");
            return ExitCode::from(2);
        }
    }

    match format {
        OutputFormat::Json => {
            let result = json!({
                "schema_version": SCHEMA_VERSION,
                "lang": lang,
                "staging_dir": staging.to_string_lossy(),
                "staged": staged,
                "toolchain": command,
            });
            println!(
                "{}",
                serde_json::to_string_pretty(&result).expect("valid JSON")
            );
        }
        OutputFormat::Text => println!(
            "staged {staged} page(s) to {}; toolchain '{command}' ok",
            staging.display()
        ),
    }
    ExitCode::SUCCESS
}

// arqix:implements REQ-04-01-03-04
// arqix:implements REQ-04-01-03-05
// arqix:implements REQ-04-01-03-06
// arqix:implements REQ-04-01-03-07
// arqix:implements REQ-04-01-03-09
/// `arqix render pdf [<file>...] [--lang <lang>] [--out <path>]` — the PDF
/// half of the orchestrator: stage artefact-ready pages (or take the
/// selected Markdown files verbatim), then invoke the configured renderer
/// (Pandoc unless overridden) with the configured defaults file and
/// template; the artefact lands per the configured artefact mode. PDF
/// renders one artefact per top-level document (ADR-0013, REQ-04-01-03-09):
/// each declared (or auto-discovered) document is staged body-only, its
/// headings re-levelled so the title lands at H1, and rendered in its own
/// Pandoc invocation with the document title as explicit metadata. An
/// explicit file selection or `--out` keeps the single-invocation path.
pub fn pdf(
    files: &[String],
    lang: Option<&str>,
    out: Option<&str>,
    format: OutputFormat,
) -> ExitCode {
    let default_lang = crate::config::default_lang(Path::new("."));
    let lang = lang.unwrap_or(&default_lang);
    let publish = crate::config::publish_policy(Path::new("."));
    let skip = crate::config::skip_dirs(Path::new("."));

    let mut artefacts: Vec<String> = Vec::new();
    let mut renderer = String::new();
    // An explicit file selection or `--out` keeps the single-invocation
    // path: one artefact, either the selected files verbatim or the whole
    // package staged and concatenated. Otherwise PDF splits per document.
    let single = !files.is_empty() || out.is_some();
    for root in crate::config::roots(Path::new(".")) {
        let package = Path::new(&root)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| root.clone());
        let policy = crate::config::render_policy(Path::new("."), &package);
        renderer = policy.pdf_command.clone();

        let lang_root = Path::new(&root).join(lang);
        let lang_root = if lang_root.is_dir() {
            lang_root
        } else if lang == default_lang {
            PathBuf::from(&root)
        } else {
            continue;
        };

        if single {
            // The renderer's inputs: the selected files verbatim, or the
            // staged artefact-ready pages of the whole package in path order.
            let inputs: Vec<String> = if files.is_empty() {
                let staging = Path::new("render-staging").join(&package);
                let mut members = Vec::new();
                crate::util::collect_markdown(&lang_root, &skip, &mut members);
                // The whole package is many files concatenated into one PDF:
                // each keeps its own title as a chapter (collection semantics).
                let is_collection = members.len() > 1;
                match stage_members(
                    &members,
                    &lang_root,
                    &lang_root,
                    &staging,
                    &publish,
                    is_collection,
                ) {
                    Ok(staged) => staged,
                    Err(code) => return code,
                }
            } else {
                files.to_vec()
            };
            if inputs.is_empty() {
                continue;
            }
            let target = match out {
                Some(path) => PathBuf::from(path),
                None if policy.artefact_mode == "detached" => {
                    Path::new(&policy.artefact_dir).join(format!("{package}.pdf"))
                }
                None => lang_root.join("artefacts").join(format!("{package}.pdf")),
            };
            if let Err(code) = run_renderer(&policy, &inputs, &target, None) {
                return code;
            }
            artefacts.push(target.to_string_lossy().to_string());
            // The single-invocation path renders once, not once per root.
            break;
        }

        // One PDF per top-level document (REQ-04-01-03-09): resolve the
        // declared documents (or auto-discover them from the language root).
        for document in resolve_documents(&policy, &lang_root, &skip) {
            let mut members = collect_document_members(&document.path, &skip);
            // A fragment that another member includes is inlined by the
            // assembler, never a standalone input (the #71/#83 rule).
            drop_included_fragments(&mut members);
            // The index.md landing is a site-navigation stub, not a chapter.
            drop_landing_index(&mut members, &document.path);
            order_members(&mut members, &document.path);

            // A document that stages several members is a collection (each ADR,
            // each blog post is its own chapter): keep each member's leading
            // title as its H1. A single-member document (a unit family's
            // assembled page, a standalone page) drops the wrapper title and
            // lifts its sections to chapters (REQ-04-01-03-09).
            let is_collection = members.len() > 1;
            let staging = Path::new("render-staging").join(&document.name);
            let staged = match stage_members(
                &members,
                &lang_root,
                &document.base,
                &staging,
                &publish,
                is_collection,
            ) {
                Ok(staged) => staged,
                Err(code) => return code,
            };
            if staged.is_empty() {
                continue;
            }

            let target = if policy.artefact_mode == "detached" {
                Path::new(&policy.artefact_dir).join(format!("{}.pdf", document.name))
            } else {
                lang_root
                    .join("artefacts")
                    .join(format!("{}.pdf", document.name))
            };
            if let Err(code) = run_renderer(&policy, &staged, &target, Some(&document.title)) {
                return code;
            }
            artefacts.push(target.to_string_lossy().to_string());
        }
    }

    match format {
        OutputFormat::Json => {
            let result = json!({
                "schema_version": SCHEMA_VERSION,
                "lang": lang,
                "renderer": renderer,
                "artefacts": artefacts,
            });
            println!(
                "{}",
                serde_json::to_string_pretty(&result).expect("valid JSON")
            );
        }
        OutputFormat::Text => {
            for artefact in &artefacts {
                println!("rendered {artefact} via '{renderer}'");
            }
        }
    }
    ExitCode::SUCCESS
}

// arqix:implements REQ-04-01-17-01
// arqix:implements REQ-04-01-17-02
// arqix:implements REQ-04-01-17-03
/// Stage the specification catalogue: one generated page per workflow
/// group, bundling the group's stories and their derived requirements —
/// an HTML anchor per ID for deep links, the coverage status straight
/// from the trace graph. Everything is ordered by ID and carries no
/// wall-clock values, so identical corpus state yields identical pages.
fn stage_catalogue(staging: &Path) -> Result<(), ExitCode> {
    let docs = crate::store::documents();
    let model = crate::trace::corpus_model();
    let (coverage, _) = crate::trace::coverage_report(&model);

    // Coverage status per requirement id: verified beats planned beats
    // uncovered (the trace engine's vocabulary).
    let mut status = std::collections::BTreeMap::new();
    if let Some(rows) = coverage["requirements"].as_array() {
        for row in rows {
            if let Some(id) = row["id"].as_str() {
                let non_empty =
                    |key: &str| row[key].as_array().is_some_and(|list| !list.is_empty());
                let value = if non_empty("verified_by") {
                    "verified"
                } else if non_empty("planned_by") {
                    "planned"
                } else {
                    "uncovered"
                };
                status.insert(id.to_string(), value);
            }
        }
    }

    let retired = |doc: &&crate::parser::Document| {
        !doc.frontmatter
            .iter()
            .any(|line| line.trim() == "lifecycle-status: retired")
    };
    // Stories per workflow group, from the declared is-part-of-workflow
    // triple (ADR-0012: declared relations are the source of truth).
    let mut groups: std::collections::BTreeMap<String, Vec<&crate::parser::Document>> =
        std::collections::BTreeMap::new();
    for doc in docs
        .iter()
        .filter(|d| d.classes.iter().any(|c| c == "user-story"))
        .filter(retired)
    {
        let Some(group) = doc
            .triples
            .iter()
            .find(|t| t.predicate == "is-part-of-workflow")
            .and_then(|t| t.object.rsplit('/').next())
        else {
            continue;
        };
        groups.entry(group.to_string()).or_default().push(doc);
    }
    // Requirements per owning story, from the declared derived-from
    // triples; a cross-cutting requirement appears under every story that
    // derives it.
    let mut by_story: std::collections::BTreeMap<String, Vec<&crate::parser::Document>> =
        std::collections::BTreeMap::new();
    for doc in docs
        .iter()
        .filter(|d| d.classes.iter().any(|c| c.ends_with("requirement")))
        .filter(retired)
    {
        for triple in doc.triples.iter().filter(|t| t.predicate == "derived-from") {
            by_story.entry(triple.object.clone()).or_default().push(doc);
        }
    }

    let mut index = String::from(
        "---\ntitle: \"Specification\"\n---\n# Specification\n\nThe story and requirement catalogue, one page per workflow group; every ID is a stable anchor, every requirement carries its live coverage status.\n\n",
    );
    for (group, stories) in &groups {
        let mut page =
            format!("---\ntitle: \"Specification — {group}\"\n---\n# Specification — {group}\n");
        let mut stories = stories.clone();
        stories.sort_by_key(|d| d.id.clone());
        let mut anchored: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
        for story in &stories {
            let (Some(id), Some(title)) = (&story.id, &story.title) else {
                continue;
            };
            page.push_str(&format!(
                "\n<a id=\"{id}\"></a>\n\n## {id} — {title}\n\n{}\n",
                first_paragraph(&story.body)
            ));
            let mut reqs = story
                .iri
                .as_ref()
                .and_then(|iri| by_story.get(iri))
                .cloned()
                .unwrap_or_default();
            reqs.sort_by_key(|d| d.id.clone());
            for req in reqs {
                let (Some(req_id), Some(req_title)) = (&req.id, &req.title) else {
                    continue;
                };
                if anchored.insert(req_id) {
                    page.push_str(&format!("\n<a id=\"{req_id}\"></a>\n"));
                }
                page.push_str(&format!(
                    "\n### {req_id} — {req_title}\n\n{}\n\n*Coverage: {}.*\n",
                    first_paragraph(&req.body),
                    status.get(req_id.as_str()).copied().unwrap_or("uncovered")
                ));
            }
        }
        let path = staging.join("specification").join(format!("{group}.md"));
        if let Some(parent) = path.parent()
            && let Err(err) = std::fs::create_dir_all(parent)
        {
            eprintln!("error: cannot create {}: {err}", parent.display());
            return Err(ExitCode::from(2));
        }
        if let Err(err) = std::fs::write(&path, &page) {
            eprintln!("error: cannot write {}: {err}", path.display());
            return Err(ExitCode::from(2));
        }
        index.push_str(&format!(
            "- [{group}]({group}.md) — {} story(ies)\n",
            stories.len()
        ));
    }
    if !groups.is_empty()
        && let Err(err) = std::fs::write(staging.join("specification/index.md"), index)
    {
        eprintln!("error: cannot write the specification index: {err}");
        return Err(ExitCode::from(2));
    }
    Ok(())
}

/// The first prose paragraph of a document body: the text after the
/// leading heading, up to the next blank line or heading — the story's
/// "As a ..." sentence, a requirement's obligation.
fn first_paragraph(body: &str) -> String {
    let mut lines = Vec::new();
    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') || (trimmed.starts_with("<!--") && trimmed.ends_with("-->")) {
            if !lines.is_empty() {
                break;
            }
            continue;
        }
        if trimmed.is_empty() {
            if !lines.is_empty() {
                break;
            }
            continue;
        }
        lines.push(trimmed);
    }
    lines.join("\n")
}

/// Copy one configured asset (file or directory tree) into the staging
/// dir; a missing source is a config error naming the path.
/// Rewrite relative Markdown links to `.csv` targets onto the staged `.md`
/// table pages; absolute URLs stay untouched.
fn rewrite_csv_links(text: &str) -> String {
    let re = regex::Regex::new(r"\]\(([^)\s]+?)\.csv\)").expect("static regex");
    re.replace_all(text, |caps: &regex::Captures| {
        let target = &caps[1];
        if target.starts_with("http://")
            || target.starts_with("https://")
            || target.starts_with('/')
        {
            caps[0].to_string()
        } else {
            format!("]({target}.md)")
        }
    })
    .into_owned()
}

/// Collect CSV files under `dir`, path-sorted, honouring the skip list —
/// the CSV sibling of `collect_markdown`.
fn collect_csv(dir: &Path, skip: &[String], out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut children: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
    children.sort();
    for path in children {
        let name = path.file_name().unwrap_or_default().to_string_lossy();
        if path.symlink_metadata().is_ok_and(|m| m.is_symlink()) {
            continue;
        }
        if path.is_dir() {
            if !skip.contains(&name.to_string()) {
                collect_csv(&path, skip, out);
            }
        } else if name.ends_with(".csv") {
            out.push(path);
        }
    }
}

/// One minimal RFC-4180 record: fields split on unquoted commas, quoted
/// fields unwrapped, doubled quotes collapsed.
fn csv_record(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '"' if in_quotes && chars.peek() == Some(&'"') => {
                field.push('"');
                chars.next();
            }
            '"' => in_quotes = !in_quotes,
            ',' if !in_quotes => fields.push(std::mem::take(&mut field)),
            c => field.push(c),
        }
    }
    fields.push(field);
    fields
}

/// The generated table page for one staged CSV: heading, provenance line,
/// and the rows as a Markdown table with pipes escaped.
fn csv_table_page(source: &str, text: &str) -> String {
    let name = Path::new(source)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();
    let mut out = format!(
        "# {name}

Generated at staging time from `{source}`.

"
    );
    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let Some(header) = lines.next() else {
        return out;
    };
    let cells = |line: &str| -> String {
        let escaped: Vec<String> = csv_record(line)
            .iter()
            .map(|f| f.replace('|', "\\|"))
            .collect();
        format!("| {} |\n", escaped.join(" | "))
    };
    let header_cells = csv_record(header).len();
    out.push_str(&cells(header));
    out.push_str(&format!("|{}\n", " --- |".repeat(header_cells)));
    for line in lines {
        out.push_str(&cells(line));
    }
    out
}

fn copy_asset(source: &Path, target: &Path) -> Result<(), ExitCode> {
    if source.is_dir() {
        let entries = match std::fs::read_dir(source) {
            Ok(entries) => entries,
            Err(err) => {
                eprintln!("error: cannot read asset dir {}: {err}", source.display());
                return Err(ExitCode::from(2));
            }
        };
        for entry in entries.flatten() {
            copy_asset(&entry.path(), &target.join(entry.file_name()))?;
        }
        return Ok(());
    }
    if !source.is_file() {
        eprintln!(
            "error: configured asset does not exist: {}",
            source.display()
        );
        return Err(ExitCode::from(2));
    }
    if let Some(parent) = target.parent()
        && let Err(err) = std::fs::create_dir_all(parent)
    {
        eprintln!("error: cannot create {}: {err}", parent.display());
        return Err(ExitCode::from(2));
    }
    if let Err(err) = std::fs::copy(source, target) {
        eprintln!("error: cannot copy {}: {err}", source.display());
        return Err(ExitCode::from(2));
    }
    Ok(())
}

/// Collect Markdown sources under `dir`, path-sorted, honouring the skip
/// set; templates (`.tpl.md`) are never published.
// arqix:implements REQ-04-01-07-04
/// The staging-relative path for a configured asset. An asset under a doc
/// root's language directory is placed at the same language-root-relative
/// location as the pages (which strip that prefix on staging), so a page's
/// relative link resolves against the staged asset; any other asset keeps its
/// configured path.
fn staged_asset_rel(asset: &str, lang: &str) -> PathBuf {
    for root in crate::config::roots(Path::new(".")) {
        let lang_root = Path::new(&root).join(lang);
        let prefix = if lang_root.is_dir() {
            format!("{}/", crate::util::to_posix(&lang_root))
        } else {
            format!("{root}/")
        };
        if let Some(rel) = asset.strip_prefix(&prefix) {
            return PathBuf::from(rel);
        }
    }
    PathBuf::from(asset)
}

// arqix:implements REQ-04-01-07-03
/// The canonical paths of every fragment pulled in by an `arqix:include`
/// directive anywhere in the corpus roots. Such a fragment is published
/// through its parent page, never as a standalone page — otherwise the site
/// carries both the stitched page and each raw fragment (found on arqix.dev).
/// Every corpus file that a relative Markdown link points at — the set that
/// keeps a link-referenced include target on the site as its own page.
fn linked_targets(skip: &[String]) -> HashSet<PathBuf> {
    let re = regex::Regex::new(r"\]\(([^)\s]+?\.md)(?:#[^)]*)?\)").expect("static regex");
    let mut sources = Vec::new();
    for root in crate::config::roots(Path::new(".")) {
        crate::util::collect_markdown(Path::new(&root), skip, &mut sources);
    }
    let mut linked = HashSet::new();
    for file in &sources {
        let Ok(text) = std::fs::read_to_string(file) else {
            continue;
        };
        let dir = file.parent().unwrap_or(Path::new("."));
        for caps in re.captures_iter(&text) {
            let target = &caps[1];
            if target.starts_with("http://") || target.starts_with("https://") {
                continue;
            }
            if let Ok(resolved) = std::fs::canonicalize(dir.join(target)) {
                linked.insert(resolved);
            }
        }
    }
    linked
}

fn included_targets(skip: &[String]) -> HashSet<PathBuf> {
    let mut sources = Vec::new();
    for root in crate::config::roots(Path::new(".")) {
        crate::util::collect_markdown(Path::new(&root), skip, &mut sources);
    }
    crate::markdown::included_target_set(&sources)
}

/// The PDF staging plan for a body (REQ-04-01-03-09): whether to drop the
/// leading heading and the shift that lifts the first *kept* heading to H1
/// (`1 − level`). Fence-aware; inspects only the first two headings.
///
/// A single-document page (a unit family's assembled page, a standalone page)
/// drops its own leading title — it duplicates the document title, which rides
/// the title page as metadata — and lifts its sections to chapters. A
/// collection member (one ADR, one blog post: `keep_leading_title`) instead
/// keeps its title as its H1 chapter, with its sections as subsections, so the
/// member's identity survives.
fn pdf_staging_plan(
    body: &str,
    title: Option<&str>,
    keep_leading_title: bool,
) -> (bool, Option<i64>) {
    // Only the first two headings decide the plan; the shared scan is
    // fence-aware, and `doc.body` outlives this call, so the text can stay
    // borrowed.
    let headings: Vec<(i64, &str)> = crate::markdown::headings_outside_fences(body)
        .take(2)
        .collect();
    let leading_is_title = matches!(
        (headings.first(), title),
        (Some((_, text)), Some(title)) if *text == title
    );
    let drop_leading = leading_is_title && !keep_leading_title;
    let anchor = if drop_leading {
        headings.get(1).map(|(level, _)| *level)
    } else {
        headings.first().map(|(level, _)| *level)
    };
    (drop_leading, anchor.map(|level| 1 - level))
}

/// One resolved top-level PDF document (REQ-04-01-03-09): its artefact
/// `name`, the `path` that scopes its members, the `base` its staged pages
/// are laid out relative to (the directory itself, or a file's parent), and
/// the `title` passed to the renderer as metadata.
struct DocTarget {
    name: String,
    path: PathBuf,
    base: PathBuf,
    title: String,
}

/// The top-level documents for a language root: the declared `documents`
/// list, or auto-discovery when it is absent (ADR-0013).
fn resolve_documents(
    policy: &crate::config::RenderPolicy,
    lang_root: &Path,
    skip: &[String],
) -> Vec<DocTarget> {
    match &policy.documents {
        Some(entries) => entries
            .iter()
            .map(|entry| {
                let path = lang_root.join(&entry.path);
                let base = if path.is_dir() {
                    path.clone()
                } else {
                    path.parent()
                        .map(Path::to_path_buf)
                        .unwrap_or_else(|| lang_root.to_path_buf())
                };
                let title = entry
                    .title
                    .clone()
                    .or_else(|| derive_title(&path))
                    .unwrap_or_else(|| entry.name.clone());
                DocTarget {
                    name: entry.name.clone(),
                    path,
                    base,
                    title,
                }
            })
            .collect(),
        None => {
            let mut out = Vec::new();
            discover_documents(lang_root, true, skip, &mut out);
            out
        }
    }
}

/// Auto-discover documents under `dir`: each immediate `.md` child at the
/// language root is a standalone document; each directory with an `index.md`
/// is a document collecting its subtree; a directory without one (a mere
/// container like `architecture/`) is descended into to find the documents
/// beneath it.
fn discover_documents(dir: &Path, is_root: bool, skip: &[String], out: &mut Vec<DocTarget>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };
    let mut paths: Vec<_> = entries.flatten().map(|e| e.path()).collect();
    paths.sort();
    for path in paths {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        if path.is_dir() {
            if path.symlink_metadata().is_ok_and(|m| m.is_symlink()) || skip.contains(&name) {
                continue;
            }
            if path.join("index.md").is_file() {
                let title = derive_title(&path).unwrap_or_else(|| name.clone());
                out.push(DocTarget {
                    name,
                    base: path.clone(),
                    path,
                    title,
                });
            } else {
                discover_documents(&path, false, skip, out);
            }
        } else if is_root && name.ends_with(".md") && !name.ends_with(".tpl.md") {
            let stem = path
                .file_stem()
                .and_then(|n| n.to_str())
                .unwrap_or(&name)
                .to_string();
            let base = path.parent().map(Path::to_path_buf).unwrap_or_default();
            let title = derive_title(&path).unwrap_or_else(|| stem.clone());
            out.push(DocTarget {
                name: stem,
                path: path.clone(),
                base,
                title,
            });
        }
    }
}

/// The document's canonical title: the `index.md` frontmatter title for a
/// directory document, the file's own for a standalone page.
fn derive_title(path: &Path) -> Option<String> {
    let primary = if path.is_dir() {
        path.join("index.md")
    } else {
        path.to_path_buf()
    };
    let text = std::fs::read_to_string(&primary).ok()?;
    crate::parser::parse(&primary.to_string_lossy(), &text).title
}

/// A document's Markdown members: the whole subtree of a directory document,
/// or the single file of a standalone page.
fn collect_document_members(path: &Path, skip: &[String]) -> Vec<PathBuf> {
    let mut members = Vec::new();
    if path.is_dir() {
        crate::util::collect_markdown(path, skip, &mut members);
    } else if path.is_file() {
        members.push(path.to_path_buf());
    }
    members
}

/// Drop every member that another member includes: the assembler inlines it
/// into its includer, so it must not also render as a standalone input.
fn drop_included_fragments(members: &mut Vec<PathBuf>) {
    let targets = crate::markdown::included_target_set(members);
    members.retain(|m| match m.canonicalize() {
        Ok(canon) => !targets.contains(&canon),
        Err(_) => true,
    });
}

/// Drop a family's `index.md` landing page from the PDF inputs when the family
/// carries other content (REQ-04-01-03-09): it is a site-navigation stub, and
/// its title already becomes the document title via metadata, so staging it
/// would only add a near-empty opening chapter. A family that is nothing but
/// its `index.md` keeps it, so a single-page document still renders.
fn drop_landing_index(members: &mut Vec<PathBuf>, doc_path: &Path) {
    if !doc_path.is_dir() || members.len() <= 1 {
        return;
    }
    let index = doc_path.join("index.md");
    members.retain(|member| *member != index);
}

/// Order a document's members path-sorted. A family's `index.md` landing is
/// already dropped for multi-page documents (`drop_landing_index`); a lone
/// index simply sorts to the front.
fn order_members(members: &mut [PathBuf], doc_path: &Path) {
    members.sort();
    if doc_path.is_dir() {
        let index = doc_path.join("index.md");
        if let Some(pos) = members.iter().position(|m| *m == index) {
            members[..=pos].rotate_right(1);
        }
    }
}

/// Stage a document's members into `staging`, laid out relative to `base`,
/// as body-only PDF pages (REQ-04-01-03-09). The publish scope
/// (`[policies.publish] exclude`) is evaluated language-root-relative, as on
/// the site path.
fn stage_members(
    members: &[PathBuf],
    lang_root: &Path,
    base: &Path,
    staging: &Path,
    publish: &crate::config::PublishPolicy,
    keep_leading_title: bool,
) -> Result<Vec<String>, ExitCode> {
    let mut staged = Vec::new();
    for file in members {
        let assembled = match crate::assembler::expand_document(file) {
            Ok(text) => text,
            Err(diagnostic) => {
                eprintln!(
                    "error: {}: {}",
                    diagnostic.file.as_deref().unwrap_or("?"),
                    diagnostic.message
                );
                return Err(ExitCode::from(2));
            }
        };
        let doc = crate::parser::parse(&file.to_string_lossy(), &assembled);
        let rel_posix = crate::util::to_posix(file.strip_prefix(lang_root).unwrap_or(file));
        if publish.exclude.iter().any(|e| {
            let prefix = e.trim_end_matches('/');
            rel_posix == prefix || rel_posix.starts_with(&format!("{prefix}/"))
        }) {
            continue;
        }
        let rel = file.strip_prefix(base).unwrap_or(file);
        let target = staging.join(rel);
        write_staged(&target, &doc, StagingMode::Pdf, keep_leading_title)?;
        staged.push(target.to_string_lossy().to_string());
    }
    Ok(staged)
}

/// Run the configured renderer over `inputs` to `target`, forwarding the
/// configured defaults/template and, when given, the document title as
/// explicit metadata. Stdio is inherited so tool output surfaces
/// transparently (REQ-04-01-03-07).
fn run_renderer(
    policy: &crate::config::RenderPolicy,
    inputs: &[String],
    target: &Path,
    title: Option<&str>,
) -> Result<(), ExitCode> {
    if let Some(parent) = target.parent()
        && let Err(err) = std::fs::create_dir_all(parent)
    {
        eprintln!("error: cannot create {}: {err}", parent.display());
        return Err(ExitCode::from(2));
    }
    let mut parts = policy.pdf_command.split_whitespace();
    let Some(program) = parts.next() else {
        eprintln!("error: [policies.render] pdf-command is empty");
        return Err(ExitCode::from(2));
    };
    let mut command = Command::new(program);
    command.args(parts).args(inputs).arg("-o").arg(target);
    if let Some(defaults) = &policy.defaults {
        command.arg("--defaults").arg(defaults);
    }
    if let Some(template) = &policy.template {
        command.arg("--template").arg(template);
    }
    if let Some(title) = title {
        command.arg("--metadata").arg(format!("title={title}"));
    }
    match command.status() {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => {
            eprintln!(
                "error: renderer failed: '{}' exited with {}",
                policy.pdf_command,
                status
                    .code()
                    .map_or("signal".to_string(), |c| c.to_string())
            );
            Err(ExitCode::from(2))
        }
        Err(err) => {
            eprintln!(
                "error: renderer failed: cannot run '{}': {err}",
                policy.pdf_command
            );
            Err(ExitCode::from(2))
        }
    }
}

/// How a page is staged: the `Site` toolchain wants a `title:` YAML header
/// with the leading duplicate heading dropped; the `Pdf` path wants the body
/// only, re-levelled so its first heading lands at H1 (ADR-0013).
#[derive(Clone, Copy, PartialEq)]
enum StagingMode {
    Site,
    Pdf,
}

/// Write one staged page: the assembled body reduced to what the target
/// consumes. Dropping the arqix identity keys also keeps staged copies out
/// of document discovery and the trace graph — no duplicate ids from
/// generated artefacts.
///
/// `Site` emits a minimal `title:` frontmatter and drops the one leading
/// `## <Title>` duplicate (the toolchain renders the title as the page H1).
/// `Pdf` emits body only — nothing must compete with the document title the
/// publisher passes as metadata — and re-levels every heading so the page's
/// first heading lands at H1, restoring the document title as a real `#`
/// and giving Pandoc a clean `1 / 1.1 / 1.1.1` outline (REQ-04-01-03-09).
fn write_staged(
    path: &Path,
    doc: &crate::parser::Document,
    mode: StagingMode,
    keep_leading_title: bool,
) -> Result<(), ExitCode> {
    let mut out = String::new();
    if mode == StagingMode::Site
        && let Some(title) = &doc.title
    {
        // Always a quoted YAML scalar: a colon in an unquoted title is
        // invalid YAML, and the toolchain then silently falls back to the
        // file slug (found on arqix.dev with WF-08-01).
        let quoted = title.replace('\\', "\\\\").replace('"', "\\\"");
        out.push_str(&format!("---\ntitle: \"{quoted}\"\n---\n"));
    }
    // The toolchain renders the frontmatter title as the page H1, and the
    // corpus convention starts every body with `## <Title>` — dropping that
    // one leading duplicate keeps titles from appearing twice on the site
    // (found on arqix.dev, 2026-07-12). Deeper repeats stay untouched.
    let duplicate_heading = doc
        .title
        .as_ref()
        .map(|title| format!("## {title}"))
        .unwrap_or_default();
    // Pdf mode drops the page's own leading title (it rides the title page as
    // metadata) and shifts the remaining outline so its first kept heading
    // becomes h1 — the same delta the assembler applies to an included fragment
    // (src/assembler.rs, `shift = target - first`).
    let (drop_leading, shift) = match mode {
        StagingMode::Pdf => pdf_staging_plan(&doc.body, doc.title.as_deref(), keep_leading_title),
        StagingMode::Site => (false, None),
    };
    let mut leading_heading_seen = false;
    let mut in_fence = false;
    for line in doc.body.lines() {
        let trimmed = line.trim();
        // Fenced code is opaque: no headings, no directives.
        if trimmed.starts_with("```") {
            in_fence = !in_fence;
            out.push_str(line);
            out.push('\n');
            continue;
        }
        // Directives and marker comments never reach the toolchain.
        if !in_fence
            && trimmed.starts_with("<!--")
            && trimmed.ends_with("-->")
            && trimmed.contains("arqix:")
        {
            continue;
        }
        if !in_fence && let Some(level) = crate::markdown::heading_level(line) {
            if mode == StagingMode::Site {
                if !leading_heading_seen {
                    leading_heading_seen = true;
                    if trimmed == duplicate_heading {
                        continue;
                    }
                }
            } else {
                // Pdf: drop the page's own leading title, then lift the rest.
                if !leading_heading_seen {
                    leading_heading_seen = true;
                    if drop_leading {
                        continue;
                    }
                }
                if let Some(shift) = shift {
                    // Clamp defensively into h1..h6: staging must never fail the
                    // whole render on an odd body outline.
                    let effective = (level + shift).clamp(1, 6) as usize;
                    let rest = &line[line.len() - line.trim_start_matches('#').len()..];
                    out.push_str(&"#".repeat(effective));
                    out.push_str(rest);
                    out.push('\n');
                    continue;
                }
            }
        }
        out.push_str(line);
        out.push('\n');
    }

    if let Some(parent) = path.parent()
        && let Err(err) = std::fs::create_dir_all(parent)
    {
        eprintln!("error: cannot create {}: {err}", parent.display());
        return Err(ExitCode::from(2));
    }
    if let Err(err) = std::fs::write(path, out) {
        eprintln!("error: cannot write {}: {err}", path.display());
        return Err(ExitCode::from(2));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    fn fresh_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("arqix-pub-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    // arqix:no-requirement
    #[test]
    fn staged_titles_survive_yaml_special_characters() {
        // Found on arqix.dev: "Automation Agent: Story-by-story ..." staged
        // as an unquoted scalar is invalid YAML (the colon), so the
        // toolchain fell back to the file slug. The staged title must be
        // quoted like any emitted YAML string.
        let doc = crate::parser::parse(
            "docs/wf.md",
            "---\nid: WF-X\ntitle: \"Automation Agent: Story-by-story\"\n---\n\nBody.\n",
        );
        let dir = fresh_dir("title");
        let path = dir.join("wf.md");
        super::write_staged(&path, &doc, super::StagingMode::Site, false).unwrap();
        let staged = std::fs::read_to_string(&path).unwrap();
        assert!(
            staged.starts_with("---\ntitle: \"Automation Agent: Story-by-story\"\n---\n"),
            "the staged title must be a quoted YAML scalar: {staged}"
        );
    }

    // arqix:no-requirement
    #[test]
    fn staged_pages_carry_only_the_consumable_frontmatter() {
        let doc = crate::parser::parse(
            "docs/x.md",
            "---\nid: X-01\ntitle: A Title\niri: arqix:x/x-01\n---\n\n## A Title\n\nBody.\n<!-- arqix:references-artefact arqix:x/y -->\n",
        );
        let dir = fresh_dir("frontmatter");
        let path = dir.join("x.md");
        super::write_staged(&path, &doc, super::StagingMode::Site, false).unwrap();
        let staged = std::fs::read_to_string(&path).unwrap();
        assert!(staged.contains("title: \"A Title\""));
        assert!(!staged.contains("iri:") && !staged.contains("id: X-01"));
        assert!(!staged.contains("arqix:references-artefact"));
        assert!(staged.contains("Body."));
    }

    // arqix:verifies REQ-04-01-03-09
    #[test]
    fn pdf_staging_drops_the_leading_title_and_lifts_content_to_top_level() {
        // The PDF path stages body-only and drops the page's own leading title
        // (it rides the title page as metadata instead), then re-levels the
        // remaining body so its first real section lands at H1 — the body opens
        // at the actual chapters, not one level deep under a wrapper title
        // (REQ-04-01-03-09).
        let doc = crate::parser::parse(
            "docs/x.md",
            "---\nid: X-01\ntitle: Context Model\niri: arqix:x/x-01\n---\n\n## Context Model\n\n### Overview\n\nBody.\n",
        );
        let dir = fresh_dir("pdf-drop");
        let path = dir.join("x.md");
        super::write_staged(&path, &doc, super::StagingMode::Pdf, false).unwrap();
        let staged = std::fs::read_to_string(&path).unwrap();
        assert!(
            !staged.contains("Context Model"),
            "the page's own leading title is dropped, not kept as a chapter: {staged}"
        );
        assert!(
            staged.lines().any(|l| l == "# Overview"),
            "the first real section lifts to H1: {staged}"
        );
        assert!(
            !staged.contains("title:") && !staged.contains("---"),
            "the PDF path emits no per-page title YAML or frontmatter: {staged}"
        );
    }

    // arqix:no-requirement
    #[test]
    fn pdf_staging_keeps_a_leading_heading_that_is_not_the_title() {
        // Only the leading title duplicate is dropped: a page whose first
        // heading is not its title keeps it, re-levelled to H1.
        let doc = crate::parser::parse(
            "docs/x.md",
            "---\nid: X-01\ntitle: Some Title\niri: arqix:x/x-01\n---\n\n## A Section\n\n### Deeper\n\nBody.\n",
        );
        let dir = fresh_dir("pdf-keep");
        let path = dir.join("x.md");
        super::write_staged(&path, &doc, super::StagingMode::Pdf, false).unwrap();
        let staged = std::fs::read_to_string(&path).unwrap();
        assert!(
            staged.lines().any(|l| l == "# A Section"),
            "a non-title leading heading is kept and promoted to H1: {staged}"
        );
        assert!(
            staged.lines().any(|l| l == "## Deeper"),
            "deeper headings shift with it: {staged}"
        );
    }

    // arqix:verifies REQ-04-01-03-09
    #[test]
    fn pdf_staging_collection_member_keeps_its_title_as_a_chapter() {
        // A collection member (an ADR, a blog post) keeps its own title as its
        // H1 chapter with its sections as subsections, so the member's identity
        // survives — unlike a single-document page, which drops its wrapper
        // title (REQ-04-01-03-09).
        let doc = crate::parser::parse(
            "docs/adr/ADR-0001-x.md",
            "---\nid: ADR-0001\ntitle: My Decision\niri: arqix:adrs/adr-0001\n---\n\n## My Decision\n\n### Context\n\nBody.\n",
        );
        let dir = fresh_dir("pdf-collection");
        let path = dir.join("adr.md");
        super::write_staged(&path, &doc, super::StagingMode::Pdf, true).unwrap();
        let staged = std::fs::read_to_string(&path).unwrap();
        assert!(
            staged.lines().any(|l| l == "# My Decision"),
            "the member keeps its title as its H1 chapter: {staged}"
        );
        assert!(
            staged.lines().any(|l| l == "## Context"),
            "its sections become subsections: {staged}"
        );
    }

    /// Characterization pin for the markdown walker's traversal contract
    /// (slice 3): per-level sort order, depth-first recursion, `skip-dirs`
    /// pruning, the `.md` / not-`.tpl.md` filter, and non-markdown exclusion.
    /// Pins `collect_markdown` before it moves into `crate::util`, so the
    /// extraction proves byte-identical.
    // arqix:no-requirement
    #[test]
    fn collect_markdown_pins_traversal_order_and_filters() {
        let root = fresh_dir("walk-order");
        std::fs::create_dir_all(root.join("sub")).unwrap();
        std::fs::create_dir_all(root.join("node_modules")).unwrap();
        for rel in [
            "z.md",
            "a.md",
            "b.tpl.md",
            "c.txt",
            "sub/m.md",
            "sub/n.tpl.md",
            "node_modules/skipped.md",
        ] {
            std::fs::write(root.join(rel), "x").unwrap();
        }
        let mut files = Vec::new();
        crate::util::collect_markdown(&root, &["node_modules".to_string()], &mut files);
        let got: Vec<String> = files
            .iter()
            .map(|p| {
                p.strip_prefix(&root)
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/")
            })
            .collect();
        // Sorted per level, depth-first: `sub` sorts before `z.md`, so its
        // member is emitted before it; `.tpl.md`, `.txt`, and the skip-dir
        // are all excluded.
        assert_eq!(got, vec!["a.md", "sub/m.md", "z.md"]);
    }

    /// Directory symlinks are never followed (cycle-safety and rglob parity).
    // arqix:no-requirement
    #[test]
    #[cfg(unix)]
    fn collect_markdown_never_follows_directory_symlinks() {
        let root = fresh_dir("walk-symlink");
        std::fs::create_dir_all(root.join("real")).unwrap();
        std::fs::write(root.join("real/inside.md"), "x").unwrap();
        std::os::unix::fs::symlink(root.join("real"), root.join("link")).unwrap();
        let mut files = Vec::new();
        crate::util::collect_markdown(&root, &[], &mut files);
        let got: Vec<String> = files
            .iter()
            .map(|p| {
                p.strip_prefix(&root)
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/")
            })
            .collect();
        // `real/inside.md` is reached once via the real directory; the `link`
        // symlink to the same directory is skipped, not traversed.
        assert_eq!(got, vec!["real/inside.md"]);
    }
}
