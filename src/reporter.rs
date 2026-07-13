//! Report & Export: `report bundle` exports a scoped evidence bundle —
//! linked requirements, stories, and trace evidence for a chosen scope,
//! deterministic for identical inputs (REQ-03-01-04-01..03), in the audit
//! formats Markdown, CSV, and JSON with stable schemas and caller-provided
//! generation metadata (REQ-04-01-12-01..03; the injected-clock discipline
//! keeps the wall clock out of the engine).

use crate::OutputFormat;
use crate::diag::SCHEMA_VERSION;
use crate::trace::Model;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::process::ExitCode;

// arqix:implements REQ-03-01-04-01
// arqix:implements REQ-03-01-04-02
// arqix:implements REQ-03-01-04-03
// arqix:implements REQ-04-01-12-01
// arqix:implements REQ-04-01-12-02
// arqix:implements REQ-04-01-12-03
/// `arqix report bundle <ID>... [--out <dir>] [--stamp <text>]`
pub fn bundle(
    ids: &[String],
    out: Option<&str>,
    stamp: Option<&str>,
    format: OutputFormat,
) -> ExitCode {
    let model = crate::trace::corpus_model();
    let (coverage, _) = crate::trace::coverage_report(&model);

    // Resolve the scope: requirement IDs stand for themselves; a story ID
    // pulls in every requirement derived from it (the declared triples are
    // the source of truth, ADR-0012).
    let scope: BTreeSet<String> = match crate::trace::resolve_scope(&model, ids) {
        Ok(scope) => scope,
        Err(message) => {
            eprintln!("error: {message}");
            return ExitCode::from(2);
        }
    };

    let rows: Vec<Value> = coverage["requirements"]
        .as_array()
        .map(|rows| {
            rows.iter()
                .filter(|row| row["id"].as_str().is_some_and(|id| scope.contains(id)))
                .cloned()
                .collect()
        })
        .unwrap_or_default();
    let inputs = crate::trace::requirement_files(&model, &scope);

    let mut scope_list: Vec<String> = ids.to_vec();
    scope_list.sort();
    let bundle = json!({
        "schema_version": SCHEMA_VERSION,
        "stamp": stamp.unwrap_or("unstamped"),
        "scope": scope_list,
        "requirements": rows,
        "inputs": inputs,
    });

    // The bundle directory: JSON for automation, Markdown for review, the
    // scoped matrix as CSV — reviewable without reshaping.
    let out_dir = Path::new(out.unwrap_or("bundle"));
    if let Err(err) = std::fs::create_dir_all(out_dir) {
        eprintln!("error: cannot create {}: {err}", out_dir.display());
        return ExitCode::from(2);
    }
    let json_text = serde_json::to_string_pretty(&bundle).expect("valid JSON");
    let mut markdown = String::from(
        "| requirement | kind | story | verified by | planned by | implemented by |\n\
         | --- | --- | --- | --- | --- | --- |\n",
    );
    for row in &rows {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            row["id"].as_str().unwrap_or("?"),
            row["kind"].as_str().unwrap_or("?"),
            row["story"].as_str().unwrap_or("—"),
            joined(&row["verified_by"]),
            joined(&row["planned_by"]),
            joined(&row["implemented_by"]),
        ));
    }
    let csv = crate::trace::matrix_csv_scoped(&model, &scope);
    for (name, content) in [
        ("bundle.json", json_text.as_str()),
        ("evidence.md", markdown.as_str()),
        ("matrix.csv", csv.as_str()),
    ] {
        if let Err(err) = std::fs::write(out_dir.join(name), content) {
            eprintln!(
                "error: cannot write {}: {err}",
                out_dir.join(name).display()
            );
            return ExitCode::from(2);
        }
    }

    match format {
        OutputFormat::Json => println!("{json_text}"),
        OutputFormat::Text => println!(
            "bundled {} requirement(s) to {}",
            rows.len(),
            out_dir.display()
        ),
    }
    ExitCode::SUCCESS
}

fn joined(value: &Value) -> String {
    let items: Vec<&str> = value
        .as_array()
        .map(|a| a.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();
    if items.is_empty() {
        "—".to_string()
    } else {
        items.join("; ")
    }
}

// arqix:implements REQ-05-01-15-01
// arqix:implements REQ-05-01-15-02
// arqix:implements REQ-05-01-15-03
/// `arqix report knowledge [--out <dir>]` — the corpus as an Open Knowledge
/// Format bundle (US-05-01-15): one artefact-ready concept document per
/// living corpus document, OKF fields mapped from declared metadata, the
/// publish scope and the lifecycle honoured.
pub fn knowledge(out: Option<&str>, format: OutputFormat) -> ExitCode {
    let policy = crate::config::publish_policy(Path::new("."));
    let default_lang = crate::config::default_lang(Path::new("."));
    let out_dir = Path::new(out.unwrap_or("knowledge"));

    let mut exported = 0usize;
    for root in crate::config::roots(Path::new(".")) {
        // The default language's root, exactly as the publisher resolves it.
        let lang_root = Path::new(&root).join(&default_lang);
        let lang_root = if lang_root.is_dir() {
            lang_root
        } else {
            std::path::PathBuf::from(&root)
        };

        for doc in crate::store::documents() {
            let file = Path::new(&doc.file);
            let Ok(rel) = file.strip_prefix(&lang_root) else {
                continue;
            };
            let rel_posix = rel.to_string_lossy().replace('\\', "/");
            // The publish scope and the lifecycle: excluded subtrees and
            // retired documents never become living knowledge (ADR-0010).
            if policy.exclude.iter().any(|e| {
                let prefix = e.trim_end_matches('/');
                rel_posix == prefix || rel_posix.starts_with(&format!("{prefix}/"))
            }) {
                continue;
            }
            if doc
                .frontmatter
                .iter()
                .any(|line| line.trim() == "lifecycle-status: retired")
            {
                continue;
            }

            let assembled = match crate::assembler::expand_document(file) {
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
            let expanded = crate::parser::parse(&doc.file, &assembled);
            if let Err(code) = write_concept(&out_dir.join(rel), &doc, &expanded) {
                return code;
            }
            exported += 1;
        }
    }

    match format {
        OutputFormat::Json => println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "schema_version": SCHEMA_VERSION,
                "out": out_dir.to_string_lossy(),
                "concepts": exported,
            }))
            .expect("valid JSON")
        ),
        OutputFormat::Text => {
            println!("exported {exported} concept(s) to {}", out_dir.display());
        }
    }
    ExitCode::SUCCESS
}

/// Write one OKF concept document: fields mapped from declared metadata —
/// `type` from the declared class (the generic document type otherwise),
/// `title` verbatim, `timestamp` from the declared update date; absent
/// metadata is omitted, never fabricated (REQ-05-01-15-02).
fn write_concept(
    path: &Path,
    doc: &crate::parser::Document,
    expanded: &crate::parser::Document,
) -> Result<(), ExitCode> {
    let mut front = String::from("---\n");
    let concept_type = doc
        .classes
        .first()
        .cloned()
        .unwrap_or_else(|| "document".to_string());
    front.push_str(&format!("type: {concept_type}\n"));
    if let Some(title) = &doc.title {
        let quoted = title.replace('\\', "\\\\").replace('"', "\\\"");
        front.push_str(&format!("title: \"{quoted}\"\n"));
    }
    if let Some(updated) = doc.frontmatter.iter().find_map(|line| {
        line.trim()
            .strip_prefix("updated:")
            .map(str::trim)
            .filter(|v| !v.is_empty())
    }) {
        front.push_str(&format!("timestamp: {updated}\n"));
    }
    front.push_str("---\n");

    let mut body = String::new();
    for line in expanded.body.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("<!--") && trimmed.ends_with("-->") && trimmed.contains("arqix:") {
            continue;
        }
        body.push_str(line);
        body.push('\n');
    }

    if let Some(parent) = path.parent()
        && let Err(err) = std::fs::create_dir_all(parent)
    {
        eprintln!("error: cannot create {}: {err}", parent.display());
        return Err(ExitCode::from(2));
    }
    if let Err(err) = std::fs::write(path, format!("{front}{body}")) {
        eprintln!("error: cannot write {}: {err}", path.display());
        return Err(ExitCode::from(2));
    }
    Ok(())
}

// --- Report-unit snapshots (the Rust port of scripts/arqix_report.py) -----
//
// Every unit answers exactly one named question from the living catalog in
// docs/en/reports/QUESTIONS.md (ADR-0008); the presentation follows the
// question, not the data structure. Units are deterministic projections of
// the trace model — identical corpus, identical bytes — and the snapshot
// stamp (commit + date) is injected via `--stamp`, never taken from the wall
// clock. The Python `arqix_report.py` stays the conformance oracle for the
// grace period, so `arqix report snapshot` must reproduce it byte-for-byte.

const UNITS_DIR: &str = "docs/en/reports/units";
const TRACE_DIR: &str = "docs/en/reports/trace";

/// One report unit: its filename and the projection that renders it. The
/// signature is uniform so the set can be iterated; a unit ignores the inputs
/// it does not need.
type Unit = fn(&Model, &Value, &str, &BTreeSet<String>) -> String;

const UNITS: [(&str, Unit); 8] = [
    ("story-progress.md", unit_story_progress),
    ("scoreboard.md", unit_scoreboard),
    ("test-to-requirement.md", unit_test_to_requirement),
    ("test-to-story.md", unit_test_to_story),
    ("test-to-workflow.md", unit_test_to_workflow),
    ("adr-to-requirement.md", unit_adr_to_requirement),
    ("code-to-requirement.md", unit_code_to_requirement),
    ("doc-to-code.md", unit_doc_to_code),
];

/// The generated provenance header shared by every unit — do-not-edit notice,
/// the answered question and its catalog id, the injected snapshot stamp, and
/// the regeneration recipe (still the oracle command, as provenance).
fn header(question: &str, qid: &str, snapshot: &str) -> String {
    format!(
        "<!-- GENERATED SNAPSHOT — do not edit by hand.\n\
         \x20    Question: {qid} (see docs/en/reports/QUESTIONS.md)\n\
         \x20    Snapshot: {snapshot}\n\
         \x20    Regenerate: python3 scripts/arqix_report.py --snapshot \"<sha>, <date>\" -->\n\
         \n# {question}\n"
    )
}

/// Escape a value for a Markdown table cell — a legal `|` in a title must not
/// add a column.
fn cell(value: &str) -> String {
    value.replace('|', "\\|")
}

/// The document's rendered title, falling back to its id when it declares
/// none (`info.get("title") or doc_id`).
fn title_of(model: &Model, doc_id: &str) -> String {
    model
        .documents
        .get(doc_id)
        .and_then(|info| info.title.clone())
        .filter(|title| !title.is_empty())
        .unwrap_or_else(|| doc_id.to_string())
}

/// Sorted `(test, location, requirement, ignored)` rows for marker edges of a
/// kind that carry an attached test name (the Markdown markers do not).
fn marker_rows(model: &Model, kind: &str) -> Vec<(String, String, String, bool)> {
    let mut rows: Vec<(String, String, String, bool)> = model
        .edges
        .iter()
        .filter(|e| e.kind == kind && e.test.is_some())
        .map(|e| {
            (
                e.test.clone().unwrap_or_default(),
                format!("{}:{}", e.from, e.line),
                e.to.clone(),
                e.ignored,
            )
        })
        .collect();
    rows.sort();
    rows
}

/// The stories a requirement is derived from (`derived-from` edges to a
/// `US-…`), sorted; not deduplicated, exactly like the oracle generator.
fn stories_of_requirement(model: &Model, req_id: &str) -> Vec<String> {
    let mut stories: Vec<String> = model
        .edges
        .iter()
        .filter(|e| e.from == req_id && e.kind == "derived-from" && e.to.starts_with("US-"))
        .map(|e| e.to.clone())
        .collect();
    stories.sort();
    stories
}

/// The workflows a story is part of (`is-part-of-workflow` edges), sorted.
fn workflows_of_story(model: &Model, story_id: &str) -> Vec<String> {
    let mut workflows: Vec<String> = model
        .edges
        .iter()
        .filter(|e| e.from == story_id && e.kind == "is-part-of-workflow")
        .map(|e| e.to.clone())
        .collect();
    workflows.sort();
    workflows
}

/// Python 3 `round(100 * n / d)`: nearest integer, ties to even. Computed over
/// the exact rational so it matches the oracle bit for bit; `d == 0` yields 0.
fn percent(n: u64, d: u64) -> u64 {
    if d == 0 {
        return 0;
    }
    let num = 100 * n;
    let quotient = num / d;
    let remainder = num % d;
    match (2 * remainder).cmp(&d) {
        std::cmp::Ordering::Less => quotient,
        std::cmp::Ordering::Greater => quotient + 1,
        std::cmp::Ordering::Equal if quotient.is_multiple_of(2) => quotient,
        std::cmp::Ordering::Equal => quotient + 1,
    }
}

/// A ten-cell progress bar: `done // 10` filled blocks, the rest light shade.
fn render_bar(done: u64) -> String {
    let full = (done / 10) as usize;
    "█".repeat(full) + &"░".repeat(10 - full)
}

fn nonempty(value: &Value) -> bool {
    value.as_array().is_some_and(|a| !a.is_empty())
}

/// Q-01: How far along is each user story?
fn unit_story_progress(
    model: &Model,
    coverage: &Value,
    snapshot: &str,
    retired: &BTreeSet<String>,
) -> String {
    // verified, planned, uncovered
    let mut per_story: BTreeMap<String, [u64; 3]> = BTreeMap::new();
    if let Some(rows) = coverage["requirements"].as_array() {
        for row in rows {
            let id = row["id"].as_str().unwrap_or_default();
            // A requirement counts for every story that demands it; when it
            // declares none, its owning story stands in (which may be null —
            // then it counts for no story at all).
            let mut stories = stories_of_requirement(model, id);
            if stories.is_empty()
                && let Some(story) = row["story"].as_str()
            {
                stories.push(story.to_string());
            }
            let verified = nonempty(&row["verified_by"]);
            let planned = nonempty(&row["planned_by"]);
            for story in stories {
                if retired.contains(&story) {
                    continue;
                }
                let bucket = per_story.entry(story).or_insert([0, 0, 0]);
                if verified {
                    bucket[0] += 1;
                } else if planned {
                    bucket[1] += 1;
                } else {
                    bucket[2] += 1;
                }
            }
        }
    }

    let mut lines = vec![header(
        "How far along is each user story?",
        "Q-01",
        snapshot,
    )];
    lines.push(
        "A requirement counts for every story that demands it (`derived-from`), \
         so shared requirements advance several stories at once.\n"
            .to_string(),
    );
    lines.push("| story | title | verified | planned | uncovered | progress |".to_string());
    lines.push("| --- | --- | ---: | ---: | ---: | --- |".to_string());
    for (story, bucket) in &per_story {
        let [verified, planned, uncovered] = *bucket;
        let total = verified + planned + uncovered;
        let done = percent(verified, total);
        let bar = render_bar(done);
        lines.push(format!(
            "| {story} | {} | {verified} | {planned} | {uncovered} | `{bar}` {done}% |",
            cell(&title_of(model, story)),
        ));
    }
    let story_retired = retired
        .iter()
        .filter(|id| {
            model
                .documents
                .get(*id)
                .is_some_and(|d| d.doctype == "user-story")
        })
        .count();
    if story_retired > 0 {
        lines.push(format!(
            "\nRetired stories excluded from this view (ADR-0010): {story_retired}."
        ));
    }
    lines.join("\n") + "\n"
}

/// Q-03: What share of the requirements is verifiably implemented?
fn unit_scoreboard(
    _model: &Model,
    coverage: &Value,
    snapshot: &str,
    _retired: &BTreeSet<String>,
) -> String {
    let mut lines = vec![header(
        "What share of the requirements is verifiably implemented?",
        "Q-03",
        snapshot,
    )];
    lines.push("| kind | verified | planned | uncovered | total | verified % |".to_string());
    lines.push("| --- | ---: | ---: | ---: | ---: | ---: |".to_string());
    for kind in ["functional", "quality", "constraint"] {
        let s = &coverage["summary"][kind];
        let verified = s["verified"].as_u64().unwrap_or(0);
        let planned = s["planned"].as_u64().unwrap_or(0);
        let uncovered = s["uncovered"].as_u64().unwrap_or(0);
        let total = s["total"].as_u64().unwrap_or(0);
        let pct = percent(verified, total);
        lines.push(format!(
            "| {kind} | {verified} | {planned} | {uncovered} | {total} | {pct}% |"
        ));
    }
    lines.push(String::new());
    lines.push(
        "*Verified* means a `verifies` marker on an active test; markers on `#[ignore]`d \
         tests only *plan* verification (red-skeleton lifecycle, ADR-0006)."
            .to_string(),
    );
    lines.join("\n") + "\n"
}

/// Q-02: Which tests verify which requirements?
fn unit_test_to_requirement(
    model: &Model,
    _coverage: &Value,
    snapshot: &str,
    _retired: &BTreeSet<String>,
) -> String {
    let mut lines = vec![header(
        "Which tests verify which requirements?",
        "Q-02",
        snapshot,
    )];
    lines.push("| test | location | requirement | status |".to_string());
    lines.push("| --- | --- | --- | --- |".to_string());
    for (test, loc, req, ignored) in marker_rows(model, "verifies") {
        let status = if ignored {
            "planned (ignored)"
        } else {
            "active"
        };
        lines.push(format!("| `{test}` | {loc} | {req} | {status} |"));
    }
    lines.join("\n") + "\n"
}

/// Q-05: Which user story belongs to which integration test?
fn unit_test_to_story(
    model: &Model,
    _coverage: &Value,
    snapshot: &str,
    _retired: &BTreeSet<String>,
) -> String {
    let mut pairs: BTreeSet<(String, String)> = BTreeSet::new();
    for (test, _loc, req, _ignored) in marker_rows(model, "verifies") {
        for story in stories_of_requirement(model, &req) {
            pairs.insert((test.clone(), story));
        }
    }
    let mut lines = vec![header(
        "Which user story belongs to which integration test?",
        "Q-05",
        snapshot,
    )];
    lines.push("Joined test → requirement (`verifies`) → story (`derived-from`).\n".to_string());
    lines.push("| test | story | title |".to_string());
    lines.push("| --- | --- | --- |".to_string());
    for (test, story) in &pairs {
        lines.push(format!(
            "| `{test}` | {story} | {} |",
            cell(&title_of(model, story))
        ));
    }
    lines.join("\n") + "\n"
}

/// Q-06: Which workflow belongs to which integration test?
fn unit_test_to_workflow(
    model: &Model,
    _coverage: &Value,
    snapshot: &str,
    _retired: &BTreeSet<String>,
) -> String {
    let mut pairs: BTreeSet<(String, String)> = BTreeSet::new();
    for (test, _loc, req, _ignored) in marker_rows(model, "verifies") {
        for story in stories_of_requirement(model, &req) {
            for workflow in workflows_of_story(model, &story) {
                pairs.insert((test.clone(), workflow));
            }
        }
    }
    let mut lines = vec![header(
        "Which workflow belongs to which integration test?",
        "Q-06",
        snapshot,
    )];
    lines.push(
        "Joined test → requirement → story → workflow (`is-part-of-workflow`).\n".to_string(),
    );
    lines.push("| test | workflow | title |".to_string());
    lines.push("| --- | --- | --- |".to_string());
    for (test, workflow) in &pairs {
        lines.push(format!(
            "| `{test}` | {workflow} | {} |",
            cell(&title_of(model, workflow))
        ));
    }
    lines.join("\n") + "\n"
}

/// Q-07: Which ADRs are linked to which requirements?
fn unit_adr_to_requirement(
    model: &Model,
    _coverage: &Value,
    snapshot: &str,
    _retired: &BTreeSet<String>,
) -> String {
    // The corpus edges are already ordered by (from, line, to, kind); a stable
    // sort by (from, to) then matches the oracle's `sorted(key=(from, str(to)))`.
    let mut edges: Vec<_> = model
        .edges
        .iter()
        .filter(|e| e.kind == "guides-implementation-of")
        .collect();
    edges.sort_by(|a, b| (&a.from, &a.to).cmp(&(&b.from, &b.to)));
    let mut lines = vec![header(
        "Which ADRs are linked to which requirements?",
        "Q-07",
        snapshot,
    )];
    lines.push("| adr | title | requirement |".to_string());
    lines.push("| --- | --- | --- |".to_string());
    for e in edges {
        lines.push(format!(
            "| {} | {} | {} |",
            e.from,
            cell(&title_of(model, &e.from)),
            e.to
        ));
    }
    lines.join("\n") + "\n"
}

/// Q-04: Which code implements which requirement?
fn unit_code_to_requirement(
    model: &Model,
    _coverage: &Value,
    snapshot: &str,
    _retired: &BTreeSet<String>,
) -> String {
    // EVERY implements marker answers the question — including one whose
    // following item is not a fn (a const, a module header): those carry no
    // attached name but are still implementing code.
    let mut rows: Vec<(String, String, String)> = model
        .edges
        .iter()
        .filter(|e| e.kind == "implements")
        .map(|e| {
            (
                e.test.clone().unwrap_or_else(|| "—".to_string()),
                format!("{}:{}", e.from, e.line),
                e.to.clone(),
            )
        })
        .collect();
    rows.sort();
    let mut lines = vec![header(
        "Which code implements which requirement?",
        "Q-04",
        snapshot,
    )];
    if rows.is_empty() {
        lines.push(
            "No `implements` markers exist yet — the Rust implementation phase has not \
             started. This unit fills up as stories move from red to green."
                .to_string(),
        );
    } else {
        lines.push("| code | location | requirement |".to_string());
        lines.push("| --- | --- | --- |".to_string());
        for (name, loc, req) in rows {
            let code = if name != "—" {
                format!("`{name}`")
            } else {
                name
            };
            lines.push(format!("| {code} | {loc} | {req} |"));
        }
    }
    lines.join("\n") + "\n"
}

/// Q-08: Where is the documentation for a given piece of code?
fn unit_doc_to_code(
    _model: &Model,
    _coverage: &Value,
    snapshot: &str,
    _retired: &BTreeSet<String>,
) -> String {
    let mut lines = vec![header(
        "Where is the documentation for a given piece of code?",
        "Q-08",
        snapshot,
    )];
    lines.push(
        "Partially answerable today: code → requirement (`implements` markers, see Q-04) → \
         story/unit via the ontology. A direct code → document convention (e.g. an \
         `arqix:documented-by` marker or unit frontmatter listing source paths) is an open \
         design decision; until it is made, this unit stays a placeholder naming that gap."
            .to_string(),
    );
    lines.join("\n") + "\n"
}

/// Extract the embedded snapshot stamp from a committed unit (the
/// `     Snapshot: …` header line), or the empty string — mirrors the oracle's
/// `SNAPSHOT_RE`. The stamp records when a unit was taken, so a fresh unit is
/// regenerated with the stamp it already carries and never goes stale by
/// itself.
fn embedded_snapshot(text: &str) -> String {
    text.lines()
        .find_map(|line| line.strip_prefix("     Snapshot: "))
        .unwrap_or("")
        .to_string()
}

/// (Re)generate every unit into `out_dir` from the current corpus.
fn snapshot_generate(out_dir: &str, stamp: &str, format: OutputFormat) -> ExitCode {
    let (model, retired) = crate::trace::snapshot_inputs();
    let (coverage, _) = crate::trace::coverage_report(&model);
    let out = Path::new(out_dir);
    if let Err(err) = std::fs::create_dir_all(out) {
        eprintln!("error: cannot create {}: {err}", out.display());
        return ExitCode::from(2);
    }
    let mut written = Vec::new();
    for (filename, unit) in UNITS {
        let content = unit(&model, &coverage, stamp, &retired);
        let path = out.join(filename);
        if let Err(err) = std::fs::write(&path, content) {
            eprintln!("error: cannot write {}: {err}", path.display());
            return ExitCode::from(2);
        }
        written.push(path.to_string_lossy().into_owned());
    }
    match format {
        OutputFormat::Json => println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "schema_version": SCHEMA_VERSION,
                "stamp": stamp,
                "out": out.to_string_lossy(),
                "units": written,
            }))
            .expect("valid JSON")
        ),
        OutputFormat::Text => {
            for path in &written {
                println!("wrote {path}");
            }
        }
    }
    ExitCode::SUCCESS
}

/// Freshness gate for the committed snapshots: regenerate every unit with the
/// stamp it already carries and every matrix, and compare byte-for-byte
/// against the committed files. Exit non-zero on any stale or missing file —
/// the report-freshness gate the sequencer runs.
fn snapshot_check(_format: OutputFormat) -> ExitCode {
    let (model, retired) = crate::trace::snapshot_inputs();
    let (coverage, _) = crate::trace::coverage_report(&model);
    let mut stale: Vec<(String, &str)> = Vec::new();

    for (filename, unit) in UNITS {
        let path = Path::new(UNITS_DIR).join(filename);
        match std::fs::read_to_string(&path) {
            Err(_) => stale.push((path.to_string_lossy().into_owned(), "missing")),
            Ok(text) => {
                let stamp = embedded_snapshot(&text);
                if unit(&model, &coverage, &stamp, &retired) != text {
                    stale.push((path.to_string_lossy().into_owned(), "stale"));
                }
            }
        }
    }
    for (filename, matrix_type) in [("matrix.csv", "req-test"), ("matrix-us-req.csv", "us-req")] {
        let path = Path::new(TRACE_DIR).join(filename);
        match std::fs::read_to_string(&path) {
            Err(_) => stale.push((path.to_string_lossy().into_owned(), "missing")),
            Ok(text) => {
                if crate::trace::matrix_csv(&model, matrix_type) != text {
                    stale.push((path.to_string_lossy().into_owned(), "stale"));
                }
            }
        }
    }

    for (path, why) in &stale {
        println!("FAIL {path}: {why} — regenerate with `just reports`");
    }
    if stale.is_empty() {
        println!("reports: fresh ({} units, 2 matrices)", UNITS.len());
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

// arqix:implements REQ-04-01-12-04
/// `arqix report snapshot [--stamp <text>] [--check] [--out <dir>]` — the
/// question-driven report units (ADR-0008). `--stamp` (re)generates them from
/// the trace graph with the injected provenance stamp; `--check` gates the
/// committed snapshots for freshness. The Python `arqix_report.py` remains the
/// conformance oracle for the grace period (arc42 chapter 8).
pub fn snapshot(
    stamp: Option<&str>,
    check: bool,
    out: Option<&str>,
    format: OutputFormat,
) -> ExitCode {
    if check {
        return snapshot_check(format);
    }
    let Some(stamp) = stamp else {
        eprintln!("error: --stamp is required (injected, never ambient)");
        return ExitCode::from(2);
    };
    snapshot_generate(out.unwrap_or(UNITS_DIR), stamp, format)
}

#[cfg(test)]
mod snapshot_tests {
    //! The Rust port of `arqix_report.py`'s `--selftest`: the same projections
    //! over the same in-memory corpus, asserted line-for-line.
    use super::*;
    use crate::trace::{coverage_report, model_from_corpus, retired_from_corpus};

    const SNAPSHOT: &str = "test, 2026-01-01";

    // The oracle's SELFTEST_CORPUS. The `.rs` entries are single-line escaped
    // literals on purpose: their marker text must not land on a physical `//`
    // source line, or the corpus scanner would treat this file's own tests as
    // markers.
    fn selftest_corpus() -> Vec<(String, String)> {
        vec![
            (
                "docs/r.md".to_string(),
                r#"---
id: REQ-11-11-11-01
iri: arqix:requirements/req-11-11-11-01
rdf:
  type:
    - arqix:classes/functional-requirement
triples:
  - predicate: arqix:properties/derived-from
    object:
      - arqix:user-stories/us-22-22-22
      - arqix:user-stories/us-33-33-33
---
body
"#
                .to_string(),
            ),
            (
                "docs/s2.md".to_string(),
                r#"---
id: US-33-33-33
title: Provide a Cloned Example
iri: arqix:user-stories/us-33-33-33
rdf:
  type:
    - arqix:classes/user-story
triples:
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-11-11-11-01
meta:
  lifecycle-status: retired
---
body
"#
                .to_string(),
            ),
            (
                "docs/s.md".to_string(),
                r#"---
id: US-22-22-22
title: Provide a Linked Example
iri: arqix:user-stories/us-22-22-22
rdf:
  type:
    - arqix:classes/user-story
triples:
  - predicate: arqix:properties/has-requirement
    object:
      - arqix:requirements/req-11-11-11-01
  - predicate: arqix:properties/is-part-of-workflow
    object: arqix:workflows/wf-22-22
---
body
"#
                .to_string(),
            ),
            (
                "docs/w.md".to_string(),
                r#"---
id: WF-22-22
title: Example Workflow
iri: arqix:workflows/wf-22-22
rdf:
  type:
    - arqix:classes/workflow
---
body
"#
                .to_string(),
            ),
            (
                "a.rs".to_string(),
                "// arqix:verifies REQ-11-11-11-01\n#[test]\nfn covers() {}\n".to_string(),
            ),
            (
                "b.rs".to_string(),
                "// arqix:implements REQ-11-11-11-01\nconst SCHEMA: u32 = 1;\n".to_string(),
            ),
        ]
    }

    fn model_and_coverage() -> (Model, Value) {
        let model = model_from_corpus(&selftest_corpus());
        let (coverage, _) = coverage_report(&model);
        (model, coverage)
    }

    // arqix:verifies REQ-04-01-12-04
    #[test]
    fn story_progress_counts_the_verified_requirement() {
        let (model, coverage) = model_and_coverage();
        let progress = unit_story_progress(&model, &coverage, SNAPSHOT, &BTreeSet::new());
        assert!(
            progress.contains("| US-22-22-22 | Provide a Linked Example | 1 | 0 | 0 |"),
            "{progress}"
        );
    }

    // arqix:verifies REQ-04-01-12-04
    #[test]
    fn retired_ids_finds_the_retired_story() {
        let retired = retired_from_corpus(&selftest_corpus());
        assert_eq!(retired, BTreeSet::from(["US-33-33-33".to_string()]));
    }

    // arqix:verifies REQ-04-01-12-04
    #[test]
    fn story_progress_excludes_retired_stories() {
        let (model, coverage) = model_and_coverage();
        let retired = retired_from_corpus(&selftest_corpus());
        let filtered = unit_story_progress(&model, &coverage, SNAPSHOT, &retired);
        assert!(!filtered.contains("US-33-33-33"), "{filtered}");
        assert!(
            filtered.contains("Retired stories excluded from this view (ADR-0010): 1."),
            "{filtered}"
        );
    }

    // arqix:verifies REQ-04-01-12-04
    #[test]
    fn scoreboard_shows_full_verified_functional() {
        let (model, coverage) = model_and_coverage();
        let board = unit_scoreboard(&model, &coverage, SNAPSHOT, &BTreeSet::new());
        assert!(
            board.contains("| functional | 1 | 0 | 0 | 1 | 100% |"),
            "{board}"
        );
    }

    // arqix:verifies REQ-04-01-12-04
    #[test]
    fn test_to_requirement_lists_the_active_test() {
        let (model, coverage) = model_and_coverage();
        let t2r = unit_test_to_requirement(&model, &coverage, SNAPSHOT, &BTreeSet::new());
        assert!(
            t2r.contains("| `covers` | a.rs:1 | REQ-11-11-11-01 | active |"),
            "{t2r}"
        );
    }

    // arqix:verifies REQ-04-01-12-04
    #[test]
    fn test_to_story_joins_through_the_requirement() {
        let (model, coverage) = model_and_coverage();
        let t2s = unit_test_to_story(&model, &coverage, SNAPSHOT, &BTreeSet::new());
        assert!(
            t2s.contains("| `covers` | US-22-22-22 | Provide a Linked Example |"),
            "{t2s}"
        );
    }

    // arqix:verifies REQ-04-01-12-04
    #[test]
    fn test_to_workflow_joins_through_the_story() {
        let (model, coverage) = model_and_coverage();
        let t2w = unit_test_to_workflow(&model, &coverage, SNAPSHOT, &BTreeSet::new());
        assert!(
            t2w.contains("| `covers` | WF-22-22 | Example Workflow |"),
            "{t2w}"
        );
    }

    // arqix:verifies REQ-04-01-12-04
    #[test]
    fn code_unit_lists_implements_markers_without_a_fn() {
        let (model, coverage) = model_and_coverage();
        let q4 = unit_code_to_requirement(&model, &coverage, SNAPSHOT, &BTreeSet::new());
        assert!(q4.contains("| — | b.rs:1 | REQ-11-11-11-01 |"), "{q4}");
    }

    // arqix:verifies REQ-04-01-12-04
    #[test]
    fn units_are_deterministic() {
        let (model, coverage) = model_and_coverage();
        let first = unit_story_progress(&model, &coverage, SNAPSHOT, &BTreeSet::new());
        let again = unit_story_progress(&model, &coverage, SNAPSHOT, &BTreeSet::new());
        assert_eq!(first, again);
    }

    // arqix:verifies REQ-04-01-12-04
    #[test]
    fn table_cells_escape_pipes() {
        assert_eq!(cell("a|b"), "a\\|b");
    }

    // arqix:verifies REQ-04-01-12-04
    #[test]
    fn percent_ties_round_to_even_like_python() {
        // Python's round() breaks ties to even: 12.5 -> 12, 37.5 -> 38.
        assert_eq!(percent(1, 8), 12);
        assert_eq!(percent(3, 8), 38);
        assert_eq!(percent(5, 8), 62);
        assert_eq!(percent(7, 8), 88);
        assert_eq!(percent(2, 3), 67);
        assert_eq!(percent(0, 0), 0);
    }
}
