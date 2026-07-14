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

    let mut staged = 0usize;
    let skip = crate::config::skip_dirs(Path::new("."));
    // Fragments pulled in by an `arqix:include` reach the site through their
    // parent page; they are never staged as standalone pages (REQ-04-01-07-03).
    let included = included_targets(&skip);
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
        collect_markdown(&lang_root, &skip, &mut files);
        for file in files {
            // Skip a fragment another page stitches in — it is published
            // through that page, not as its own page (REQ-04-01-07-03).
            if std::fs::canonicalize(&file)
                .map(|c| included.contains(&c))
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
            let doc = crate::parser::parse(&file.to_string_lossy(), &assembled);
            let rel = file.strip_prefix(&lang_root).unwrap_or(&file).to_path_buf();
            // The publish scope: excluded subtrees never leave the repo.
            let rel_posix = rel.to_string_lossy().replace('\\', "/");
            if policy.exclude.iter().any(|e| {
                let prefix = e.trim_end_matches('/');
                rel_posix == prefix || rel_posix.starts_with(&format!("{prefix}/"))
            }) {
                continue;
            }
            if let Err(code) = write_staged(&staging.join(&rel), &doc) {
                return code;
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
/// `arqix render pdf [<file>...] [--lang <lang>] [--out <path>]` — the PDF
/// half of the orchestrator: stage artefact-ready pages (or take the
/// selected Markdown files verbatim), then invoke the configured renderer
/// (Pandoc unless overridden) with the configured defaults file and
/// template; the artefact lands per the configured artefact mode. PDF is
/// always single-page (ADR-0013): one invocation, one artefact per package.
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

        // The renderer's inputs: the selected files verbatim, or the
        // staged artefact-ready pages of this package in path order.
        let inputs: Vec<String> = if files.is_empty() {
            let staging = Path::new("render-staging").join(&package);
            let mut sources = Vec::new();
            collect_markdown(&lang_root, &skip, &mut sources);
            let mut staged = Vec::new();
            for file in sources {
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
                let doc = crate::parser::parse(&file.to_string_lossy(), &assembled);
                let rel = file.strip_prefix(&lang_root).unwrap_or(&file).to_path_buf();
                let rel_posix = rel.to_string_lossy().replace('\\', "/");
                if publish.exclude.iter().any(|e| {
                    let prefix = e.trim_end_matches('/');
                    rel_posix == prefix || rel_posix.starts_with(&format!("{prefix}/"))
                }) {
                    continue;
                }
                let target = staging.join(&rel);
                if let Err(code) = write_staged(&target, &doc) {
                    return code;
                }
                staged.push(target.to_string_lossy().to_string());
            }
            staged
        } else {
            files.to_vec()
        };
        if inputs.is_empty() {
            continue;
        }

        // The artefact target per the configured mode; an explicit --out
        // always wins.
        let target = match out {
            Some(path) => PathBuf::from(path),
            None if policy.artefact_mode == "detached" => {
                Path::new(&policy.artefact_dir).join(format!("{package}.pdf"))
            }
            None => lang_root.join("artefacts").join(format!("{package}.pdf")),
        };
        if let Some(parent) = target.parent()
            && let Err(err) = std::fs::create_dir_all(parent)
        {
            eprintln!("error: cannot create {}: {err}", parent.display());
            return ExitCode::from(2);
        }

        let mut parts = policy.pdf_command.split_whitespace();
        let Some(program) = parts.next() else {
            eprintln!("error: [policies.render] pdf-command is empty");
            return ExitCode::from(2);
        };
        let mut command = Command::new(program);
        command.args(parts).args(&inputs).arg("-o").arg(&target);
        if let Some(defaults) = &policy.defaults {
            command.arg("--defaults").arg(defaults);
        }
        if let Some(template) = &policy.template {
            command.arg("--template").arg(template);
        }
        // Inherit stdio so the renderer's own output and errors surface
        // transparently (REQ-04-01-03-07).
        match command.status() {
            Ok(status) if status.success() => {}
            Ok(status) => {
                eprintln!(
                    "error: renderer failed: '{}' exited with {}",
                    policy.pdf_command,
                    status
                        .code()
                        .map_or("signal".to_string(), |c| c.to_string())
                );
                return ExitCode::from(2);
            }
            Err(err) => {
                eprintln!(
                    "error: renderer failed: cannot run '{}': {err}",
                    policy.pdf_command
                );
                return ExitCode::from(2);
            }
        }
        artefacts.push(target.to_string_lossy().to_string());

        // Selected files render once, not once per package.
        if !files.is_empty() {
            break;
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
            format!("{}/", lang_root.to_string_lossy().replace('\\', "/"))
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
fn included_targets(skip: &[String]) -> HashSet<PathBuf> {
    let mut sources = Vec::new();
    for root in crate::config::roots(Path::new(".")) {
        collect_markdown(Path::new(&root), skip, &mut sources);
    }
    let mut targets = HashSet::new();
    for file in &sources {
        let Ok(text) = std::fs::read_to_string(file) else {
            continue;
        };
        let dir = file.parent().unwrap_or_else(|| Path::new("."));
        let mut in_fence = false;
        for line in text.lines() {
            if line.trim_start().starts_with("```") {
                in_fence = !in_fence;
                continue;
            }
            if in_fence {
                continue;
            }
            if let Some((target, _)) = crate::linter::include_directive(line)
                && let Ok(canon) = std::fs::canonicalize(dir.join(&target))
            {
                targets.insert(canon);
            }
        }
    }
    targets
}

fn collect_markdown(dir: &Path, skip: &[String], files: &mut Vec<PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };
    let mut paths: Vec<_> = entries.flatten().map(|e| e.path()).collect();
    paths.sort();
    for path in paths {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if path.is_dir() {
            if !path.symlink_metadata().is_ok_and(|m| m.is_symlink())
                && !skip.iter().any(|s| s == name)
            {
                collect_markdown(&path, skip, files);
            }
        } else if name.ends_with(".md") && !name.ends_with(".tpl.md") {
            files.push(path);
        }
    }
}

/// Write one staged page: the assembled body under a minimal frontmatter
/// carrying only what site toolchains consume (the title). Dropping the
/// arqix identity keys also keeps staged copies out of document discovery
/// and the trace graph — no duplicate ids from generated artefacts.
fn write_staged(path: &Path, doc: &crate::parser::Document) -> Result<(), ExitCode> {
    let mut out = String::new();
    if let Some(title) = &doc.title {
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
    let mut leading_heading_seen = false;
    for line in doc.body.lines() {
        let trimmed = line.trim();
        // Directives and marker comments never reach the toolchain.
        if trimmed.starts_with("<!--") && trimmed.ends_with("-->") && trimmed.contains("arqix:") {
            continue;
        }
        if !leading_heading_seen && trimmed.starts_with('#') {
            leading_heading_seen = true;
            if trimmed == duplicate_heading {
                continue;
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
        let dir = std::env::temp_dir().join("arqix-staged-title-test");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("wf.md");
        super::write_staged(&path, &doc).unwrap();
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
        let dir = std::env::temp_dir().join("arqix-staged-frontmatter-test");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("x.md");
        super::write_staged(&path, &doc).unwrap();
        let staged = std::fs::read_to_string(&path).unwrap();
        assert!(staged.contains("title: \"A Title\""));
        assert!(!staged.contains("iri:") && !staged.contains("id: X-01"));
        assert!(!staged.contains("arqix:references-artefact"));
        assert!(staged.contains("Body."));
    }
}
