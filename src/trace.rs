//! Trace Engine: the Rust port of the Python oracle (scripts/arqix_trace.py).
//! Builds the canonical trace graph from markers and frontmatter triples,
//! and projects coverage, per-requirement checks, and matrices (ADR-0006
//! layers, ADR-0007 node identity). The oracle remains the conformance
//! reference: `arqix trace …` must be JSON-value-equal to it on the corpus.

use crate::OutputFormat;
use crate::parser::{self, is_requirement_id};
use regex::Regex;
use serde_json::{Map, Value, json};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::Path;
use std::process::ExitCode;
use std::sync::LazyLock;

const SCHEMA_VERSION: u64 = 1;
const SKIP_DIRS: [&str; 5] = [".git", "target", "node_modules", "__pycache__", "fixtures"];
const REQ_KIND_CLASSES: [&str; 3] = [
    "functional-requirement",
    "quality-requirement",
    "constraint",
];

pub(crate) struct Edge {
    pub(crate) from: String,
    pub(crate) to: String,
    pub(crate) kind: String,
    pub(crate) line: usize,
    pub(crate) ignored: bool,
    pub(crate) test: Option<String>,
    /// Some for frontmatter-triple edges (their location), None for markers.
    file: Option<String>,
}

impl Edge {
    fn to_json(&self) -> Value {
        let mut m = Map::new();
        m.insert("from".into(), json!(self.from));
        m.insert("to".into(), json!(self.to));
        m.insert("kind".into(), json!(self.kind));
        m.insert("line".into(), json!(self.line));
        m.insert("ignored".into(), json!(self.ignored));
        m.insert("test".into(), json!(self.test));
        if let Some(f) = &self.file {
            m.insert("file".into(), json!(f));
        }
        Value::Object(m)
    }

    fn location(&self) -> String {
        format!("{}:{}", self.from, self.line)
    }
}

struct Requirement {
    kind: String,
    file: String,
    kind_declared: bool,
}

pub(crate) struct DocInfo {
    file: String,
    pub(crate) doctype: String,
    /// The document's declared `title`, as the report units render it.
    pub(crate) title: Option<String>,
}

pub(crate) struct Model {
    requirements: BTreeMap<String, Requirement>,
    pub(crate) documents: BTreeMap<String, DocInfo>,
    pub(crate) edges: Vec<Edge>,
}

/// Walk the working directory for .md and .rs corpus files (skip .tpl.md and
/// the skip dirs), returning sorted (posix path, text) pairs.
fn read_corpus() -> Vec<(String, String)> {
    let mut files = Vec::new();
    walk(Path::new("."), &mut files);
    files.sort_by(|a, b| a.0.cmp(&b.0));
    files
}

fn walk(dir: &Path, out: &mut Vec<(String, String)>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if path.is_dir() {
            // The oracle's rglob never follows directory symlinks; doing so
            // here would diverge and make the walk unbounded on a cycle.
            if !SKIP_DIRS.contains(&name) && !path.symlink_metadata().is_ok_and(|m| m.is_symlink())
            {
                walk(&path, out);
            }
            continue;
        }
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if (ext != "md" && ext != "rs") || name.ends_with(".tpl.md") {
            continue;
        }
        if let Ok(text) = std::fs::read_to_string(&path) {
            let rel = path.strip_prefix("./").unwrap_or(&path);
            out.push((crate::util::to_posix(rel), text));
        }
    }
}

/// Oracle id rule: a word character start followed by word characters or
/// hyphens (no dots), matching FRONTMATTER_ID_RE.
fn valid_id(id: &str) -> bool {
    let mut chars = id.chars();
    match chars.next() {
        Some(c) if c.is_alphanumeric() || c == '_' => {}
        _ => return false,
    }
    chars.all(|c| c.is_alphanumeric() || c == '_' || c == '-')
}

fn requirement_kind(classes: &[String]) -> (String, bool) {
    for class in classes {
        if REQ_KIND_CLASSES.contains(&class.as_str()) {
            let short = match class.as_str() {
                "functional-requirement" => "functional",
                "quality-requirement" => "quality",
                _ => "constraint",
            };
            return (short.to_string(), true);
        }
    }
    ("functional".to_string(), false)
}

fn build_model(corpus: &[(String, String)]) -> Model {
    let mut documents: BTreeMap<String, DocInfo> = BTreeMap::new();
    let mut requirements: BTreeMap<String, Requirement> = BTreeMap::new();
    let mut iri_map: HashMap<String, String> = HashMap::new();
    let mut parsed = Vec::new();

    for (path, text) in corpus {
        if !path.ends_with(".md") {
            continue;
        }
        let doc = parser::parse(path, text);
        let id = match &doc.id {
            // The oracle accepts only `[\w][\w-]*` ids; a dotted id like
            // `ontology-arqix-v0.1.0` is not a corpus document there, so it
            // must not be one here either (conformance).
            Some(id) if valid_id(id) => id.clone(),
            _ => continue,
        };
        let doctype = doc.kind();
        documents.insert(
            id.clone(),
            DocInfo {
                file: path.clone(),
                doctype: doctype.clone(),
                title: doc.title.clone(),
            },
        );
        if let Some(iri) = &doc.iri {
            iri_map.insert(iri.clone(), id.clone());
        }
        if doctype == "requirement" {
            let (kind, declared) = requirement_kind(&doc.classes);
            requirements.insert(
                id.clone(),
                Requirement {
                    kind,
                    file: path.clone(),
                    kind_declared: declared,
                },
            );
        }
        parsed.push(doc);
    }

    let mut edges = Vec::new();
    for (path, text) in corpus {
        let is_rust = path.ends_with(".rs");
        let lines: Vec<&str> = parser::py_splitlines(text);
        for (idx, line) in lines.iter().enumerate() {
            let marker = if is_rust {
                rs_marker(line)
            } else {
                md_marker(line)
            };
            if let Some((kind, target)) = marker {
                let (ignored, test) = if is_rust {
                    attached_test(&lines, idx + 1)
                } else {
                    (false, None)
                };
                edges.push(Edge {
                    from: path.clone(),
                    to: target,
                    kind,
                    line: idx + 1,
                    ignored,
                    test,
                    file: None,
                });
            }
        }
    }
    for doc in &parsed {
        let from = doc.id.clone().unwrap();
        for triple in &doc.triples {
            let to = iri_map
                .get(&triple.object)
                .cloned()
                .unwrap_or_else(|| triple.object.clone());
            edges.push(Edge {
                from: from.clone(),
                to,
                kind: triple.predicate.clone(),
                line: triple.line,
                ignored: false,
                test: None,
                file: Some(doc.file.clone()),
            });
        }
        // Body-level reference markers: paragraph-anchored `references-artefact`
        // edges. Triple-shaped (document id `from`, resolved `to`, `file`),
        // located at the body line so the reference points at the sentence.
        for (idx, line) in doc.body.lines().enumerate() {
            if let Some(target) = md_reference_marker(line) {
                let to = iri_map.get(&target).cloned().unwrap_or(target);
                edges.push(Edge {
                    from: from.clone(),
                    to,
                    kind: "references-artefact".to_string(),
                    line: doc.body_offset + idx,
                    ignored: false,
                    test: None,
                    file: Some(doc.file.clone()),
                });
            }
        }
    }
    edges.sort_by(|a, b| (&a.from, a.line, &a.to, &a.kind).cmp(&(&b.from, b.line, &b.to, &b.kind)));

    Model {
        requirements,
        documents,
        edges,
    }
}

fn rs_marker(line: &str) -> Option<(String, String)> {
    marker_body(line.trim().strip_prefix("//")?.trim_start())
}

fn md_marker(line: &str) -> Option<(String, String)> {
    let inner = line
        .trim()
        .strip_prefix("<!--")?
        .strip_suffix("-->")?
        .trim();
    marker_body(inner)
}

/// Parse a whole-line `<!-- arqix:references-artefact <arqix-iri> -->` body
/// marker into its single IRI target — the document-side, paragraph-anchored
/// analogue of a frontmatter `references-artefact` triple (ADR-0009). Kept
/// separate from `marker_body` so it never touches `.rs` verifies/implements
/// parsing or oracle conformance. Shared with the linter so its resolution
/// check (LNT-003) validates exactly what the engine parses.
pub(crate) fn md_reference_marker(line: &str) -> Option<String> {
    let inner = line
        .trim()
        .strip_prefix("<!--")?
        .strip_suffix("-->")?
        .trim();
    let after = inner.strip_prefix("arqix:references-artefact")?;
    if !after.starts_with(char::is_whitespace) {
        return None;
    }
    let mut tokens = after.split_whitespace();
    let target = tokens.next()?;
    if tokens.next().is_some() {
        return None;
    }
    // MD_REF_MARKER_RE captures `arqix:\S+` — a bare `arqix:` is no target.
    (target.starts_with("arqix:") && target.len() > "arqix:".len()).then(|| target.to_string())
}

// arqix:implements REQ-03-01-10-01
/// Parse `arqix:(verifies|implements|plans)\s+<token>` with only trailing
/// space.
fn marker_body(rest: &str) -> Option<(String, String)> {
    let rest = rest.strip_prefix("arqix:")?;
    let (kind, after) = if let Some(r) = rest.strip_prefix("verifies") {
        ("verifies", r)
    } else if let Some(r) = rest.strip_prefix("plans") {
        ("plans", r)
    } else {
        ("implements", rest.strip_prefix("implements")?)
    };
    if !after.starts_with(char::is_whitespace) {
        return None;
    }
    let mut tokens = after.split_whitespace();
    let target = tokens.next()?;
    if tokens.next().is_some() {
        return None;
    }
    Some((kind.to_string(), target.to_string()))
}

/// The marker's contiguous comment/attribute block up to its function:
/// whether the function is `#[ignore]`d and its name (ADR-0006/0007).
fn attached_test(lines: &[&str], after_marker: usize) -> (bool, Option<String>) {
    let mut ignored = false;
    for line in lines.iter().skip(after_marker) {
        let s = line.trim();
        if s.starts_with("#[ignore") {
            ignored = true;
            continue;
        }
        if s.starts_with("//") || s.starts_with("#[") {
            continue;
        }
        if let Some(name) = fn_name(s) {
            return (ignored, Some(name));
        }
        break;
    }
    (ignored, None)
}

fn fn_name(line: &str) -> Option<String> {
    let line = match line.strip_prefix("pub") {
        Some(rest) if rest.starts_with(char::is_whitespace) => rest.trim_start(),
        _ => line,
    };
    let rest = line.strip_prefix("fn")?;
    if !rest.starts_with(char::is_whitespace) {
        return None;
    }
    let name: String = rest
        .trim_start()
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    if name.is_empty() { None } else { Some(name) }
}

/// The owning story per the canonical-owner model: the first derived-from
/// edge to a US-…; falls back to the ID-derived story.
fn owner_story(req_id: &str, edges: &[Edge]) -> Value {
    for e in edges {
        if e.from == req_id && e.kind == "derived-from" && e.to.starts_with("US-") {
            return json!(e.to);
        }
    }
    story_of(req_id)
}

fn story_of(req_id: &str) -> Value {
    // Mirror the oracle's lenient codepoint slice `req_id[4:12]`: byte-indexing
    // (`&req_id[4..12]`) would panic on ids shorter than four bytes or on a
    // multibyte char boundary, and valid_id admits arbitrary word ids.
    let stem: String = req_id.chars().skip(4).take(8).collect();
    if stem == "00-00-00" {
        Value::Null
    } else {
        json!(format!("US-{stem}"))
    }
}

fn graph(model: &Model) -> Value {
    let mut nodes = Vec::new();
    let known: std::collections::BTreeSet<&String> = model.documents.keys().collect();
    for (id, info) in &model.documents {
        let mut node = Map::new();
        node.insert("id".into(), json!(id));
        node.insert("type".into(), json!(info.doctype));
        node.insert("file".into(), json!(info.file));
        if let Some(req) = model.requirements.get(id) {
            node.insert("kind".into(), json!(req.kind));
            node.insert("kind_declared".into(), json!(req.kind_declared));
            node.insert("story".into(), owner_story(id, &model.edges));
        }
        nodes.push(Value::Object(node));
    }
    let mut referenced: Vec<&String> = model.edges.iter().map(|e| &e.to).collect();
    referenced.sort();
    referenced.dedup();
    for target in referenced.iter().filter(|t| !known.contains(*t)) {
        nodes.push(json!({
            "id": target,
            "type": if is_requirement_id(target) { "requirement" } else { "unknown" },
            "unresolved": true,
        }));
    }
    let mut sources: Vec<&String> = model.edges.iter().map(|e| &e.from).collect();
    sources.sort();
    sources.dedup();
    for source in sources.iter().filter(|s| !known.contains(*s)) {
        nodes.push(json!({ "id": source, "type": "artefact", "file": source }));
    }

    let edges: Vec<Value> = model.edges.iter().map(Edge::to_json).collect();
    json!({ "schema_version": SCHEMA_VERSION, "nodes": nodes, "edges": edges })
}

struct Links {
    verified: Vec<String>,
    planned: Vec<String>,
    implemented: Vec<String>,
}

/// The corpus trace model, for downstream consumers (the reporter).
pub(crate) fn corpus_model() -> Model {
    build_model(&read_corpus())
}

/// The corpus trace model together with the set of retired document ids —
/// the inputs the report-unit snapshots (ADR-0008) are a pure projection of.
/// One corpus read feeds both, so retirement is decided over the same bytes
/// the model was built from.
pub(crate) fn snapshot_inputs() -> (Model, BTreeSet<String>) {
    let corpus = read_corpus();
    let model = build_model(&corpus);
    let retired = retired_ids(&model, &corpus);
    (model, retired)
}

/// Build the trace model from an in-memory corpus, for the report-unit tests
/// (they project it exactly as the snapshot command does over the real corpus).
#[cfg(test)]
pub(crate) fn model_from_corpus(corpus: &[(String, String)]) -> Model {
    build_model(corpus)
}

/// The retired document ids for an in-memory corpus, for the report-unit tests.
#[cfg(test)]
pub(crate) fn retired_from_corpus(corpus: &[(String, String)]) -> BTreeSet<String> {
    let model = build_model(corpus);
    retired_ids(&model, corpus)
}

/// Document ids whose file declares `lifecycle-status: retired` — they leave
/// progress denominators (ADR-0010). Mirrors the report oracle's
/// `RETIRED_RE = ^\s*lifecycle-status:\s*retired\s*$` (multiline) over the
/// document's own file text.
fn retired_ids(model: &Model, corpus: &[(String, String)]) -> BTreeSet<String> {
    let mut retired = BTreeSet::new();
    for (id, info) in &model.documents {
        let Some((_, text)) = corpus.iter().find(|(path, _)| path == &info.file) else {
            continue;
        };
        let declares_retired = text.lines().any(|line| {
            line.trim()
                .strip_prefix("lifecycle-status:")
                .is_some_and(|rest| rest.trim() == "retired")
        });
        if declares_retired {
            retired.insert(id.clone());
        }
    }
    retired
}

/// Resolve a bundle scope: a requirement ID stands for itself, any other
/// ID for the requirements derived from it (declared triples, ADR-0012).
pub(crate) fn resolve_scope(
    model: &Model,
    ids: &[String],
) -> Result<std::collections::BTreeSet<String>, String> {
    let mut scope = std::collections::BTreeSet::new();
    for id in ids {
        if model.requirements.contains_key(id) {
            scope.insert(id.clone());
            continue;
        }
        let derived: Vec<String> = model
            .edges
            .iter()
            .filter(|e| e.kind == "derived-from" && &e.to == id)
            .map(|e| e.from.clone())
            .collect();
        if derived.is_empty() {
            return Err(format!(
                "unknown id '{id}': not a requirement, and no requirement derives from it"
            ));
        }
        scope.extend(derived);
    }
    Ok(scope)
}

/// The source files of the scoped requirements — the bundle's inputs.
pub(crate) fn requirement_files(
    model: &Model,
    scope: &std::collections::BTreeSet<String>,
) -> Vec<String> {
    scope
        .iter()
        .filter_map(|id| model.requirements.get(id).map(|r| r.file.clone()))
        .collect()
}

/// The req-test matrix restricted to a scope, same schema and ordering as
/// the full export (REQ-04-01-12-02).
pub(crate) fn matrix_csv_scoped(
    model: &Model,
    scope: &std::collections::BTreeSet<String>,
) -> String {
    let full = matrix_csv(model, "req-test");
    let mut out = String::new();
    for (idx, line) in full.lines().enumerate() {
        if idx == 0 || scope.iter().any(|id| line.starts_with(id.as_str())) {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

/// One joined test outcome from a results report (US-03-01-10).
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Outcome {
    Passed,
    Failed,
    Skipped,
}

/// The coverage rows for one id, as data for the MCP `trace` tool: a
/// requirement id stands for itself, a story id for the requirements
/// derived from it (`resolve_scope`, same rule as the report bundle). Each
/// row is the coverage row plus its derived status — verified when an
/// active test verifies it, planned when only ignored tests do, uncovered
/// otherwise. `None` when the id is neither a requirement nor derivable.
pub(crate) fn trace_json(id: &str) -> Option<Value> {
    let model = corpus_model();
    let scope = resolve_scope(&model, &[id.to_string()]).ok()?;
    let (report, _) = coverage(&model, None);
    let rows: Vec<Value> = report["requirements"]
        .as_array()
        .map(|rows| {
            rows.iter()
                .filter(|row| row["id"].as_str().is_some_and(|i| scope.contains(i)))
                .map(|row| {
                    let filled = |field: &str| row[field].as_array().is_some_and(|a| !a.is_empty());
                    let status = if filled("verified_by") {
                        "verified"
                    } else if filled("planned_by") {
                        "planned"
                    } else {
                        "uncovered"
                    };
                    let mut row = row.clone();
                    row["status"] = json!(status);
                    row
                })
                .collect()
        })
        .unwrap_or_default();
    Some(json!({ "schema_version": SCHEMA_VERSION, "id": id, "requirements": rows }))
}

/// Parse the JUnit XML subset every mainstream runner emits: `<testcase>`
/// elements with a `name` attribute, a child `<failure>`/`<error>` meaning
/// failed, `<skipped>` meaning skipped, nothing meaning passed. Hand-rolled
/// over the text — the tree structure beyond test cases is irrelevant here,
/// and the dependency tree stays closed (the ADR-0014 posture).
pub(crate) fn parse_junit(text: &str) -> BTreeMap<String, Outcome> {
    let mut outcomes = BTreeMap::new();
    let mut rest = text;
    while let Some(start) = rest.find("<testcase") {
        rest = &rest[start + "<testcase".len()..];
        let Some(tag_end) = rest.find('>') else { break };
        let attrs = &rest[..tag_end];
        let self_closing = attrs.trim_end().ends_with('/');
        let name = attrs
            .split("name=\"")
            .nth(1)
            .and_then(|s| s.split('"').next());
        let body = if self_closing {
            ""
        } else {
            let after = &rest[tag_end + 1..];
            after
                .find("</testcase>")
                .map(|end| &after[..end])
                .unwrap_or("")
        };
        if let Some(name) = name {
            let outcome = if body.contains("<failure") || body.contains("<error") {
                Outcome::Failed
            } else if body.contains("<skipped") {
                Outcome::Skipped
            } else {
                Outcome::Passed
            };
            outcomes.insert(name.to_string(), outcome);
        }
    }
    outcomes
}

/// The coverage report as JSON, for downstream consumers.
pub(crate) fn coverage_report(model: &Model) -> (Value, ExitCode) {
    coverage(model, None)
}

fn coverage(model: &Model, results: Option<&BTreeMap<String, Outcome>>) -> (Value, ExitCode) {
    let mut links: BTreeMap<&String, Links> = model
        .requirements
        .keys()
        .map(|id| {
            (
                id,
                Links {
                    verified: vec![],
                    planned: vec![],
                    implemented: vec![],
                },
            )
        })
        .collect();
    // Joined outcomes per requirement (only when a results report is given).
    let mut joined: BTreeMap<String, [Vec<String>; 3]> = BTreeMap::new(); // passed, failed, skipped
    for e in &model.edges {
        if (e.kind == "verifies" || e.kind == "implements" || e.kind == "plans")
            && let Some(l) = links.get_mut(&e.to)
        {
            if e.kind == "implements" {
                l.implemented.push(e.location());
            } else if e.kind == "plans" || e.ignored {
                l.planned.push(e.location());
            } else {
                // An active verifying claim: a joined failed or skipped
                // outcome demotes it to planned (REQ-03-01-10-03); an
                // unjoined claim keeps its marker-derived status — results
                // refine, they never invent evidence.
                let outcome = results
                    .zip(e.test.as_ref())
                    .and_then(|(report, test)| report.get(test).copied());
                match outcome {
                    Some(Outcome::Failed) | Some(Outcome::Skipped) => l.planned.push(e.location()),
                    _ => l.verified.push(e.location()),
                }
                if let (Some(_), Some(test)) = (results, &e.test) {
                    let slot = match outcome {
                        Some(Outcome::Passed) => Some(0),
                        Some(Outcome::Failed) => Some(1),
                        Some(Outcome::Skipped) => Some(2),
                        None => None,
                    };
                    if let Some(slot) = slot {
                        joined.entry(e.to.clone()).or_default()[slot].push(test.clone());
                    }
                }
            }
        }
    }

    let mut rows = Vec::new();
    let mut summary: BTreeMap<String, [u64; 4]> = BTreeMap::new(); // total, verified, planned, uncovered
    let mut diagnostics = Vec::new();

    for (id, req) in &model.requirements {
        let l = &mut links.get_mut(id).unwrap();
        l.verified.sort();
        l.planned.sort();
        l.implemented.sort();
        let mut row = json!({
            "id": id,
            "kind": req.kind,
            "story": owner_story(id, &model.edges),
            "verified_by": l.verified,
            "planned_by": l.planned,
            "implemented_by": l.implemented,
        });
        // The results key exists only when a report was joined, so the
        // default output stays value-equal to the frozen oracle surface.
        if results.is_some() {
            let mut lists = joined.remove(id.as_str()).unwrap_or_default();
            lists.iter_mut().for_each(|l| l.sort());
            let [passed, failed, skipped] = lists;
            row["results"] = json!({
                "passed": passed,
                "failed": failed,
                "skipped": skipped,
            });
        }
        rows.push(row);

        let entry = summary.entry(req.kind.clone()).or_insert([0, 0, 0, 0]);
        entry[0] += 1;
        if !l.verified.is_empty() {
            entry[1] += 1;
        } else if !l.planned.is_empty() {
            entry[2] += 1;
            if req.kind == "functional" {
                diagnostics.push(diag_json(
                    "warning",
                    "TRC-COV-002",
                    format!(
                        "functional requirement {id} is only planned: no active test verifies it yet"
                    ),
                    id,
                    &req.file,
                ));
            }
        } else {
            entry[3] += 1;
            if req.kind == "functional" {
                diagnostics.push(diag_json(
                    "error",
                    "TRC-COV-001",
                    format!("functional requirement {id} has no verifies marker"),
                    id,
                    &req.file,
                ));
            }
        }
        if !req.kind_declared {
            diagnostics.push(diag_json(
                "warning",
                "TRC-KIND-001",
                format!("requirement {id} declares no kind; treated as functional"),
                id,
                &req.file,
            ));
        }
    }

    diagnostics.sort_by(|a, b| {
        let key = |d: &Value| {
            (
                d["file"].as_str().unwrap_or("").to_string(),
                d["code"].as_str().unwrap_or("").to_string(),
                d["requirement"].as_str().unwrap_or("").to_string(),
            )
        };
        key(a).cmp(&key(b))
    });

    let errors = diagnostics
        .iter()
        .filter(|d| d["severity"] == "error")
        .count();
    let summary_json: Map<String, Value> = summary
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                json!({ "total": v[0], "verified": v[1], "planned": v[2], "uncovered": v[3] }),
            )
        })
        .collect();
    let report = json!({
        "schema_version": SCHEMA_VERSION,
        "diagnostics": diagnostics,
        "requirements": rows,
        "summary": summary_json,
    });
    let code = if errors > 0 {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    };
    (report, code)
}

fn diag_json(severity: &str, code: &str, message: String, req: &str, file: &str) -> Value {
    json!({
        "severity": severity,
        "code": code,
        "message": message,
        "requirement": req,
        "file": file,
    })
}

fn check(model: &Model, req_id: &str) -> (Value, ExitCode) {
    if !model.requirements.contains_key(req_id) {
        return (
            json!({
                "schema_version": SCHEMA_VERSION,
                "requirement": req_id,
                "error": "unknown requirement",
            }),
            ExitCode::from(1),
        );
    }
    let mut verifies = Vec::new();
    let mut implements = Vec::new();
    let mut derived_from = Vec::new();
    for e in &model.edges {
        if e.to == req_id && (e.kind == "verifies" || e.kind == "implements") {
            let loc =
                json!({ "file": e.from, "line": e.line, "ignored": e.ignored, "test": e.test });
            if e.kind == "verifies" {
                verifies.push(loc);
            } else {
                implements.push(loc);
            }
        } else if e.from == req_id && e.kind == "derived-from" {
            derived_from.push(json!(e.to));
        }
    }
    (
        json!({
            "schema_version": SCHEMA_VERSION,
            "requirement": req_id,
            "verifies": verifies,
            "implements": implements,
            "derived_from": derived_from,
        }),
        ExitCode::SUCCESS,
    )
}

/// Mirror csv.writer's minimal quoting (the oracle's dialect): quote a
/// field containing the delimiter, a quote, or a line break; double
/// embedded quotes.
fn csv_field(s: &str) -> String {
    if s.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

fn csv_row(fields: &[String]) -> String {
    let quoted: Vec<String> = fields.iter().map(|f| csv_field(f)).collect();
    format!("{}\n", quoted.join(","))
}

pub(crate) fn matrix_csv(model: &Model, matrix_type: &str) -> String {
    let mut out = String::new();
    if matrix_type == "us-req" {
        out.push_str("story,requirement\n");
        let mut pairs: Vec<(&String, &String)> = model
            .edges
            .iter()
            .filter(|e| e.kind == "derived-from" && e.to.starts_with("US-"))
            .map(|e| (&e.to, &e.from))
            .collect();
        pairs.sort();
        pairs.dedup();
        for (story, req) in pairs {
            out.push_str(&csv_row(&[story.to_string(), req.to_string()]));
        }
        return out;
    }

    out.push_str("requirement,kind,verified_markers,planned_markers,implements_markers\n");
    let (report, _) = coverage(model, None);
    if let Some(rows) = report["requirements"].as_array() {
        for row in rows {
            out.push_str(&csv_row(&[
                row["id"].as_str().unwrap_or("").to_string(),
                row["kind"].as_str().unwrap_or("").to_string(),
                join_strs(&row["verified_by"]),
                join_strs(&row["planned_by"]),
                join_strs(&row["implemented_by"]),
            ]));
        }
    }
    out
}

fn join_strs(value: &Value) -> String {
    value
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str())
                .collect::<Vec<_>>()
                .join(";")
        })
        .unwrap_or_default()
}

fn emit_json(value: &Value) {
    println!(
        "{}",
        serde_json::to_string_pretty(value).expect("valid JSON")
    );
}

// arqix:implements REQ-03-01-05-01
// arqix:implements REQ-03-01-05-02
// arqix:implements REQ-03-01-05-04
// arqix:implements REQ-03-01-05-05
/// `arqix trace scan`
pub fn scan(format: OutputFormat) -> ExitCode {
    let model = build_model(&read_corpus());
    let payload = graph(&model);
    match format {
        OutputFormat::Json => emit_json(&payload),
        OutputFormat::Text => {
            for e in &model.edges {
                println!("{}:{}: {} {}", e.from, e.line, e.kind, e.to);
            }
            let nodes = payload["nodes"].as_array().map_or(0, Vec::len);
            println!("scanned: {} node(s), {} edge(s)", nodes, model.edges.len());
        }
    }
    ExitCode::SUCCESS
}

// arqix:implements REQ-01-01-08-01
// arqix:implements REQ-01-01-08-02
// arqix:implements REQ-01-01-08-03
/// `arqix trace coverage`
// arqix:implements REQ-03-01-10-02
// arqix:implements REQ-03-01-10-03
pub fn coverage_command(results: Option<&str>, format: OutputFormat) -> ExitCode {
    let outcomes = match results {
        Some(path) => match std::fs::read_to_string(path) {
            Ok(text) => Some(parse_junit(&text)),
            Err(err) => {
                eprintln!("error: cannot read results report {path}: {err}");
                return ExitCode::from(2);
            }
        },
        None => None,
    };
    let model = build_model(&read_corpus());
    let (report, code) = coverage(&model, outcomes.as_ref());
    match format {
        OutputFormat::Json => emit_json(&report),
        OutputFormat::Text => print!("{}", coverage_text(&report)),
    }
    code
}

// arqix:implements REQ-03-01-06-01
// arqix:implements REQ-03-01-06-02
// arqix:implements REQ-03-01-06-03
/// `arqix trace check <requirement>`
pub fn check_command(req_id: &str, format: OutputFormat) -> ExitCode {
    let model = build_model(&read_corpus());
    let (result, code) = check(&model, req_id);
    match format {
        OutputFormat::Json => emit_json(&result),
        OutputFormat::Text => {
            if let Some(err) = result["error"].as_str() {
                println!("{req_id}: {err}");
            } else {
                for kind in ["verifies", "implements"] {
                    let locs = result[kind]
                        .as_array()
                        .map(|a| {
                            a.iter()
                                .map(|l| {
                                    let base = format!(
                                        "{}:{}",
                                        l["file"].as_str().unwrap_or("?"),
                                        l["line"]
                                    );
                                    match l["test"].as_str() {
                                        Some(t) => format!("{base} ({t})"),
                                        None => base,
                                    }
                                })
                                .collect::<Vec<_>>()
                                .join(", ")
                        })
                        .filter(|s| !s.is_empty())
                        .unwrap_or_else(|| "none".to_string());
                    println!("{req_id}: {kind}: {locs}");
                }
                let stories = result["derived_from"]
                    .as_array()
                    .map(|a| {
                        a.iter()
                            .filter_map(|v| v.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    })
                    .filter(|s| !s.is_empty())
                    .unwrap_or_else(|| "none".to_string());
                println!("{req_id}: derived-from: {stories}");
            }
        }
    }
    code
}

// arqix:implements REQ-03-01-02-01
// arqix:implements REQ-03-01-02-02
// arqix:implements REQ-03-01-02-03
/// `arqix trace matrix [--type req-test|us-req]`
pub fn matrix_command(matrix_type: &str, _format: OutputFormat) -> ExitCode {
    if matrix_type != "req-test" && matrix_type != "us-req" {
        eprintln!("error: unknown matrix type '{matrix_type}'");
        return ExitCode::from(2);
    }
    let model = build_model(&read_corpus());
    print!("{}", matrix_csv(&model, matrix_type));
    ExitCode::SUCCESS
}

/// The requirement ids at least one active (non-ignored) test verifies —
/// the computed side of the done claim (LNT-005) and the ratchet.
pub(crate) fn verified_requirement_ids() -> Vec<String> {
    let model = build_model(&read_corpus());
    let mut verified: Vec<String> = model
        .edges
        .iter()
        .filter(|e| e.kind == "verifies" && !e.ignored)
        .map(|e| e.to.clone())
        .collect();
    verified.sort();
    verified.dedup();
    verified
}

/// The default ratchet baseline: the committed req-test matrix snapshot,
/// kept fresh by the report-freshness gate. A configured
/// `[policies.verify] ratchet-baseline` replaces it (C17,
/// REQ-04-01-16-01); an explicit `--baseline` overrides both.
const RATCHET_BASELINE: &str = "docs/en/reports/trace/matrix.csv";

// arqix:implements REQ-04-01-15-01
// arqix:implements REQ-04-01-15-02
/// `arqix trace ratchet [--baseline <path>]` — verified coverage must never
/// decrease against the baseline. The comparison is over active
/// requirements: retiring one removes it from both sides (ADR-0010), and a
/// requirement absent from the baseline is specification growth, never a
/// finding.
pub fn ratchet_command(baseline: Option<&str>, format: OutputFormat) -> ExitCode {
    // arqix:implements REQ-04-01-16-01
    // Resolution order: explicit --baseline, the configured path, the
    // built-in default snapshot location.
    let configured = crate::config::verify_policy(Path::new(".")).ratchet_baseline;
    let path = baseline.unwrap_or_else(|| configured.as_deref().unwrap_or(RATCHET_BASELINE));
    let Ok(baseline_text) = std::fs::read_to_string(path) else {
        match format {
            OutputFormat::Json => emit_json(&json!({
                "schema_version": SCHEMA_VERSION,
                "baseline": path,
                "regressions": [],
                "ok": true,
                "note": "no baseline, nothing to compare",
            })),
            OutputFormat::Text => println!("ratchet: no baseline at {path}, nothing to compare"),
        }
        return ExitCode::SUCCESS;
    };

    let corpus = read_corpus();
    let model = build_model(&corpus);
    let mut verified: Vec<&str> = Vec::new();
    for e in &model.edges {
        if e.kind == "verifies" && !e.ignored {
            verified.push(&e.to);
        }
    }

    let mut regressions = Vec::new();
    for line in baseline_text.lines().skip(1) {
        let fields = csv_parse(line);
        if fields.len() < 3 || fields[2].trim().is_empty() {
            continue; // not verified in the baseline
        }
        let id = fields[0].trim();
        // A requirement no longer in the corpus, or retired, has left the
        // comparison: a declared specification change, not a lost test.
        let Some(req) = model.requirements.get(id) else {
            continue;
        };
        if is_retired(&corpus, &req.file) {
            continue;
        }
        if !verified.contains(&id) {
            regressions.push(json!({ "id": id, "file": req.file }));
        }
    }

    let ok = regressions.is_empty();
    match format {
        OutputFormat::Json => emit_json(&json!({
            "schema_version": SCHEMA_VERSION,
            "baseline": path,
            "regressions": regressions,
            "ok": ok,
        })),
        OutputFormat::Text => {
            for r in &regressions {
                println!(
                    "error: RAT-001: coverage regression: {} was verified in the baseline but no active test verifies it ({})",
                    r["id"].as_str().unwrap_or("?"),
                    r["file"].as_str().unwrap_or("?"),
                );
            }
            if ok {
                println!("ratchet: ok (baseline {path})");
            } else {
                println!(
                    "ratchet: {} regression(s) against {path}",
                    regressions.len()
                );
            }
        }
    }
    if ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

/// One CSV line into fields, honouring the writer's quoting (csv_field):
/// quoted fields may contain commas, doubled quotes are literal quotes.
fn csv_parse(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut chars = line.chars().peekable();
    let mut quoted = false;
    while let Some(c) = chars.next() {
        match c {
            '"' if quoted => {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    field.push('"');
                } else {
                    quoted = false;
                }
            }
            '"' if field.is_empty() => quoted = true,
            ',' if !quoted => fields.push(std::mem::take(&mut field)),
            c => field.push(c),
        }
    }
    fields.push(field);
    fields
}

/// Whether the document at `file` declares `lifecycle-status: retired`
/// (ADR-0010: retired documents leave progress denominators).
fn is_retired(corpus: &[(String, String)], file: &str) -> bool {
    corpus
        .iter()
        .find(|(path, _)| path == file)
        .is_some_and(|(_, text)| {
            text.lines()
                .any(|line| line.trim() == "lifecycle-status: retired")
        })
}

fn coverage_text(report: &Value) -> String {
    let mut lines = Vec::new();
    if let Some(diags) = report["diagnostics"].as_array() {
        for d in diags {
            lines.push(format!(
                "{}: {}: {}",
                d["file"].as_str().unwrap_or("?"),
                d["code"].as_str().unwrap_or("?"),
                d["message"].as_str().unwrap_or(""),
            ));
        }
    }
    lines.push("| requirement | kind | verified by | planned by | implemented by |".to_string());
    lines.push("| --- | --- | --- | --- | --- |".to_string());
    if let Some(rows) = report["requirements"].as_array() {
        for row in rows {
            lines.push(format!(
                "| {} | {} | {} | {} | {} |",
                row["id"].as_str().unwrap_or("?"),
                row["kind"].as_str().unwrap_or("?"),
                dash(&row["verified_by"]),
                dash(&row["planned_by"]),
                dash(&row["implemented_by"]),
            ));
        }
    }
    if let Some(summary) = report["summary"].as_object() {
        for (kind, s) in summary {
            lines.push(format!(
                "{kind}: {} verified, {} planned, {} uncovered (of {})",
                s["verified"], s["planned"], s["uncovered"], s["total"],
            ));
        }
    }
    let diags = report["diagnostics"].as_array();
    let errors = diags.map_or(0, |d| d.iter().filter(|x| x["severity"] == "error").count());
    let warnings = diags.map_or(0, |d| {
        d.iter().filter(|x| x["severity"] == "warning").count()
    });
    lines.push(format!(
        "coverage: {errors} error(s), {warnings} warning(s)"
    ));
    lines.join("\n") + "\n"
}

fn dash(value: &Value) -> String {
    let joined = value
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str())
                .collect::<Vec<_>>()
                .join("; ")
        })
        .unwrap_or_default();
    if joined.is_empty() {
        "—".to_string()
    } else {
        joined
    }
}

// arqix:implements REQ-03-01-11-01
// arqix:implements REQ-03-01-11-02
/// A marker's staleness against version history: possibly stale when its
/// target requirement's document was committed after the marker's own file
/// (US-03-01-11, ADR-0015). The decision is pure over injected commit
/// timestamps; `git_last_change` is the only impurity, so unit tests inject
/// a closure and never touch git. A marker or requirement without reachable
/// history degrades to fresh (REQ-03-01-11-02). The comparison is against the
/// requirement document only — the contract the marker verifies; the owning
/// story is a grouping layer whose churn is not a staleness signal (ADR-0015).
fn freshness(model: &Model, last_change: &dyn Fn(&str) -> Option<i64>) -> (Value, ExitCode) {
    // Many markers share a requirement file; cache the injected lookups.
    let mut cache: HashMap<String, Option<i64>> = HashMap::new();
    let mut touched = |path: &str| -> Option<i64> {
        if let Some(v) = cache.get(path) {
            return *v;
        }
        let v = last_change(path);
        cache.insert(path.to_string(), v);
        v
    };

    let mut stale = Vec::new();
    let mut diagnostics = Vec::new();
    let mut evaluated: u64 = 0;

    for e in &model.edges {
        // An active verifying/implementing claim is the only kind that can go
        // stale; a `plans` marker or an ignored skeleton makes no claim yet.
        if e.ignored || (e.kind != "verifies" && e.kind != "implements") {
            continue;
        }
        evaluated += 1;
        // Both sides need reachable history; otherwise degrade to fresh.
        let Some(marker_ts) = touched(&e.from) else {
            continue;
        };
        let Some(req_file) = model.requirements.get(&e.to).map(|r| r.file.clone()) else {
            continue;
        };
        let Some(req_ts) = touched(&req_file) else {
            continue;
        };
        if req_ts <= marker_ts {
            continue;
        }

        let marker = e.location();
        diagnostics.push(diag_json(
            "warning",
            "TRC-FRESH-001",
            format!(
                "{} was committed after {marker}; its verification may be stale",
                e.to
            ),
            &e.to,
            &e.from,
        ));
        stale.push(json!({
            "marker": marker,
            "kind": e.kind,
            "requirement": e.to,
            "requirement_file": req_file,
            "marker_committed": marker_ts,
            "requirement_committed": req_ts,
        }));
    }

    stale.sort_by(|a, b| a["marker"].as_str().cmp(&b["marker"].as_str()));
    diagnostics.sort_by(|a, b| {
        let key = |d: &Value| {
            (
                d["file"].as_str().unwrap_or("").to_string(),
                d["requirement"].as_str().unwrap_or("").to_string(),
            )
        };
        key(a).cmp(&key(b))
    });

    let stale_count = stale.len() as u64;
    let report = json!({
        "schema_version": SCHEMA_VERSION,
        "diagnostics": diagnostics,
        "stale": stale,
        "summary": { "evaluated": evaluated, "stale": stale_count },
    });
    let code = if stale_count == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    };
    (report, code)
}

/// The last commit that touched `path`, in unix committer seconds — the one
/// git shell-out in the binary (ADR-0015). None when the path has no
/// reachable history (untracked, or no `.git`); freshness treats that as
/// fresh (REQ-03-01-11-02).
fn git_last_change(path: &str) -> Option<i64> {
    let output = std::process::Command::new("git")
        .args(["log", "-1", "--format=%ct", "--", path])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8(output.stdout)
        .ok()?
        .trim()
        .parse::<i64>()
        .ok()
}

fn freshness_text(report: &Value) -> String {
    let mut lines = Vec::new();
    if let Some(stale) = report["stale"].as_array() {
        for s in stale {
            lines.push(format!(
                "{}: TRC-FRESH-001: {} was committed after the marker (possibly stale)",
                s["marker"].as_str().unwrap_or("?"),
                s["requirement"].as_str().unwrap_or("?"),
            ));
        }
    }
    let evaluated = report["summary"]["evaluated"].as_u64().unwrap_or(0);
    let stale = report["summary"]["stale"].as_u64().unwrap_or(0);
    lines.push(format!(
        "freshness: {stale} possibly stale (of {evaluated} active marker(s))"
    ));
    lines.join("\n") + "\n"
}

/// `arqix trace freshness`
pub fn freshness_command(format: OutputFormat) -> ExitCode {
    let model = build_model(&read_corpus());
    let (report, code) = freshness(&model, &git_last_change);
    match format {
        OutputFormat::Json => emit_json(&report),
        OutputFormat::Text => print!("{}", freshness_text(&report)),
    }
    code
}

// --- Marker gate (the Rust port of scripts/check_trace_markers.py) --------
//
// The TDD marker gate: every test function carries a `verifies`/`plans`
// marker or an explicit no-requirement annotation (TRC-002/005), markers
// resolve to existing requirements (TRC-001/004), ignored tests name a known
// owning story (TRC-003), and derived-from/has-requirement backlinks stay
// symmetric (TRC-006). The Python checker remains the conformance oracle for
// the grace period, so `arqix trace markers --format json` must be
// JSON-value-equal to `check_trace_markers.py --json` on the corpus.

const MARKER_REQ_DIR: &str = "docs/en/architecture/req";
const MARKER_STORY_DIR: &str = "docs/en/architecture/stories";

// Whole-line markers only (a trailing comment on a code line is no marker):
// the payload is a single `\S+` token with only trailing whitespace.
static GATE_VERIFIES_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^//\s*arqix:verifies\s+(\S+)\s*$").expect("static regex"));
static GATE_PLANS_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^//\s*arqix:plans\s+(\S+)\s*$").expect("static regex"));
static GATE_NO_REQ_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^//\s*arqix:no-requirement\b").expect("static regex"));
static GATE_TEST_ATTR_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*#\[test\]").expect("static regex"));
static GATE_IGNORE_ATTR_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^\s*#\[ignore(?:\s*=\s*"([^"]*)")?\]"#).expect("static regex"));
// Every fn qualifier combination rustc compiles: visibility, then
// default/const/async/unsafe, then an extern ABI.
static GATE_FN_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^\s*(?:pub(?:\s*\([^)]*\))?\s+)?(?:(?:default|const|async|unsafe)\s+)*(?:extern\s+(?:"[^"]*"\s+)?)?fn\s+\w+"#,
    )
    .expect("static regex")
});
static GATE_IGNORE_REASON_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(US-\d{2}-\d{2}-\d{2}): .+").expect("static regex"));
static GATE_COMMENT_OR_ATTR_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*(?://|#\[)").expect("static regex"));
static GATE_REQ_FILENAME_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^REQ-\d{2}-\d{2}-\d{2}-\d{2}").expect("static regex"));
static GATE_US_FILENAME_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^US-\d{2}-\d{2}-\d{2}").expect("static regex"));
static GATE_KIND_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"arqix:classes/(functional-requirement|quality-requirement|constraint)")
        .expect("static regex")
});

/// A requirement's kind for the marker gate, keyed by its filename-derived
/// ID (the oracle reads the req directory directly, not the trace model).
struct GateReqKind {
    kind: String,
    declared: bool,
    file: String,
}

/// One gate finding: (file, line, rule, message). Sorted as a tuple, exactly
/// like the oracle's `sorted(findings)`.
type GateFinding = (String, usize, String, String);

fn kind_short(class: &str) -> String {
    match class {
        "functional-requirement" => "functional",
        "quality-requirement" => "quality",
        _ => "constraint",
    }
    .to_string()
}

/// Map requirement ID -> kind, mirroring `check_trace_markers.py`'s
/// `known_requirement_kinds`: glob `docs/en/architecture/req/REQ-*.md`, key
/// by the filename's `REQ-XX-YY-ZZ-NN`, and take the first
/// `arqix:classes/<kind>` found anywhere in the file (functional by default,
/// then reported as a TRC-KIND-001 warning).
fn gate_requirement_kinds() -> BTreeMap<String, GateReqKind> {
    let mut out = BTreeMap::new();
    let mut paths: Vec<std::path::PathBuf> = match std::fs::read_dir(MARKER_REQ_DIR) {
        Ok(rd) => rd.flatten().map(|e| e.path()).collect(),
        Err(_) => return out,
    };
    paths.sort();
    for path in paths {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if !name.starts_with("REQ-") || !name.ends_with(".md") {
            continue;
        }
        let Some(m) = GATE_REQ_FILENAME_RE.find(name) else {
            continue;
        };
        let id = m.as_str().to_string();
        let text = std::fs::read_to_string(&path).unwrap_or_default();
        let (kind, declared) = match GATE_KIND_RE.captures(&text) {
            Some(c) => (kind_short(&c[1]), true),
            None => ("functional".to_string(), false),
        };
        out.insert(
            id,
            GateReqKind {
                kind,
                declared,
                file: format!("{MARKER_REQ_DIR}/{name}"),
            },
        );
    }
    out
}

/// The known story IDs, mirroring the oracle's `known_story_ids`: glob
/// `docs/en/architecture/stories/US-*.md`, keyed by the filename's
/// `US-XX-YY-ZZ`.
fn gate_story_ids() -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let paths = match std::fs::read_dir(MARKER_STORY_DIR) {
        Ok(rd) => rd,
        Err(_) => return out,
    };
    for entry in paths.flatten() {
        let path = entry.path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if !name.starts_with("US-") || !name.ends_with(".md") {
            continue;
        }
        if let Some(m) = GATE_US_FILENAME_RE.find(name) {
            out.insert(m.as_str().to_string());
        }
    }
    out
}

/// Python `repr()` of an optional ignore reason, so the TRC-003 message is
/// byte-identical to the oracle's `{reason!r}`.
fn py_repr(reason: Option<&str>) -> String {
    let Some(s) = reason else {
        return "None".to_string();
    };
    let quote = if s.contains('\'') && !s.contains('"') {
        '"'
    } else {
        '\''
    };
    let mut out = String::new();
    out.push(quote);
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c == quote => {
                out.push('\\');
                out.push(c);
            }
            c if (c as u32) < 0x20 || (c as u32) == 0x7f => {
                out.push_str(&format!("\\x{:02x}", c as u32));
            }
            c => out.push(c),
        }
    }
    out.push(quote);
    out
}

/// Scan one Rust source for TRC-002/003/005 over its test functions,
/// mirroring the oracle's `check_test_file` line scanner exactly. Payload
/// validity (TRC-001/004) is a separate, global pass (`check_marker_targets`).
fn gate_check_test_file(
    text: &str,
    known_stories: &BTreeSet<String>,
    path: &str,
) -> Vec<GateFinding> {
    let mut findings = Vec::new();
    let mut markers: Vec<(usize, String)> = Vec::new();
    let mut no_req_lines: Vec<usize> = Vec::new();
    let mut is_test = false;
    let mut ignore: Option<(usize, Option<String>)> = None;

    macro_rules! reset {
        () => {{
            markers.clear();
            no_req_lines.clear();
            is_test = false;
            ignore = None;
        }};
    }

    for (idx, line) in parser::py_splitlines(text).iter().enumerate() {
        let line_no = idx + 1;
        let stripped = line.trim();
        if stripped.is_empty() {
            reset!();
            continue;
        }
        if let Some(c) = GATE_VERIFIES_RE.captures(stripped) {
            markers.push((line_no, c[1].to_string()));
            continue;
        }
        if let Some(c) = GATE_PLANS_RE.captures(stripped) {
            markers.push((line_no, c[1].to_string()));
            continue;
        }
        if GATE_NO_REQ_RE.is_match(stripped) {
            no_req_lines.push(line_no);
            continue;
        }
        if GATE_TEST_ATTR_RE.is_match(line) {
            is_test = true;
            continue;
        }
        if let Some(c) = GATE_IGNORE_ATTR_RE.captures(line) {
            ignore = Some((line_no, c.get(1).map(|m| m.as_str().to_string())));
            continue;
        }
        if GATE_COMMENT_OR_ATTR_RE.is_match(line) {
            continue;
        }
        if GATE_FN_RE.is_match(line) {
            if is_test {
                if markers.is_empty() && no_req_lines.is_empty() {
                    findings.push((
                        path.to_string(),
                        line_no,
                        "TRC-002".to_string(),
                        "test has neither a verifies/plans marker nor an \
                         arqix:no-requirement annotation"
                            .to_string(),
                    ));
                }
                if !markers.is_empty() && !no_req_lines.is_empty() {
                    findings.push((
                        path.to_string(),
                        line_no,
                        "TRC-005".to_string(),
                        "test carries both a verifies marker and arqix:no-requirement".to_string(),
                    ));
                }
                if let Some((ignore_line, reason)) = &ignore {
                    match GATE_IGNORE_REASON_RE.captures(reason.as_deref().unwrap_or("")) {
                        None => findings.push((
                            path.to_string(),
                            *ignore_line,
                            "TRC-003".to_string(),
                            format!(
                                "ignore reason must be 'US-XX-YY-ZZ: <text>', got {}",
                                py_repr(reason.as_deref())
                            ),
                        )),
                        Some(c) => {
                            let story = c[1].to_string();
                            if !known_stories.contains(&story) {
                                findings.push((
                                    path.to_string(),
                                    *ignore_line,
                                    "TRC-003".to_string(),
                                    format!("ignore reason names unknown story {story}"),
                                ));
                            }
                        }
                    }
                }
            }
            reset!();
        } else {
            reset!();
        }
    }

    findings.sort();
    findings
}

/// Validate every verifies/plans-marker payload in a Rust source
/// (TRC-001/004), attached to a test or not — the oracle's
/// `check_marker_targets`.
fn gate_check_marker_targets(
    text: &str,
    known_reqs: &BTreeSet<String>,
    path: &str,
) -> Vec<GateFinding> {
    let mut findings = Vec::new();
    for (idx, line) in parser::py_splitlines(text).iter().enumerate() {
        let line_no = idx + 1;
        let stripped = line.trim();
        let payload = GATE_VERIFIES_RE
            .captures(stripped)
            .or_else(|| GATE_PLANS_RE.captures(stripped))
            .map(|c| c[1].to_string());
        if let Some(payload) = payload {
            if !is_requirement_id(&payload) {
                findings.push((
                    path.to_string(),
                    line_no,
                    "TRC-004".to_string(),
                    format!("malformed marker payload '{payload}' (expected REQ-XX-YY-ZZ-NN)"),
                ));
            } else if !known_reqs.contains(&payload) {
                findings.push((
                    path.to_string(),
                    line_no,
                    "TRC-001".to_string(),
                    format!("marker references unknown requirement {payload}"),
                ));
            }
        }
    }
    findings.sort();
    findings
}

/// TRC-006: derived-from (REQ -> US) and has-requirement (US -> REQ) are
/// double bookkeeping that must stay symmetric; the oracle's `check_backlinks`
/// reports the missing counterpart at the existing edge's location.
fn gate_check_backlinks(edges: &[Edge]) -> Vec<GateFinding> {
    // Keyed by the (REQ, US) pair; a later edge overwrites an earlier one,
    // matching the oracle's dict comprehension (last wins).
    let mut derived: BTreeMap<(String, String), (String, usize)> = BTreeMap::new();
    let mut backlinks: BTreeMap<(String, String), (String, usize)> = BTreeMap::new();
    for e in edges {
        if e.kind == "derived-from" && e.to.starts_with("US-") {
            derived.insert(
                (e.from.clone(), e.to.clone()),
                (e.file.clone().unwrap_or_default(), e.line),
            );
        }
    }
    for e in edges {
        if e.kind == "has-requirement" && e.from.starts_with("US-") {
            backlinks.insert(
                (e.to.clone(), e.from.clone()),
                (e.file.clone().unwrap_or_default(), e.line),
            );
        }
    }
    let mut findings = Vec::new();
    for (pair, (file, line)) in &derived {
        if !backlinks.contains_key(pair) {
            findings.push((
                file.clone(),
                *line,
                "TRC-006".to_string(),
                format!(
                    "{} is derived-from {}, but the story has no has-requirement backlink",
                    pair.0, pair.1
                ),
            ));
        }
    }
    for (pair, (file, line)) in &backlinks {
        if !derived.contains_key(pair) {
            findings.push((
                file.clone(),
                *line,
                "TRC-006".to_string(),
                format!(
                    "{} lists {} via has-requirement, but the requirement has no derived-from counterpart",
                    pair.1, pair.0
                ),
            ));
        }
    }
    findings.sort();
    findings
}

/// The distinct known requirement IDs referenced by a `verifies` marker (not
/// `plans`) in the test files — the referenced side of coverage-by-kind.
fn gate_collect_referenced(test_files: &[(String, String)]) -> BTreeSet<String> {
    let mut refs = BTreeSet::new();
    for (_, text) in test_files {
        for line in parser::py_splitlines(text) {
            if let Some(c) = GATE_VERIFIES_RE.captures(line.trim())
                && is_requirement_id(&c[1])
            {
                refs.insert(c[1].to_string());
            }
        }
    }
    refs
}

// arqix:implements REQ-03-01-06-04
/// `arqix trace markers` — the ported TDD marker gate. Exit 1 on any finding,
/// else 0; warnings never affect the exit code.
pub fn markers_command(format: OutputFormat) -> ExitCode {
    let kinds = gate_requirement_kinds();
    let known_reqs: BTreeSet<String> = kinds.keys().cloned().collect();
    let known_stories = gate_story_ids();

    let corpus = read_corpus();
    // Every Rust file under tests/ (fixtures already excluded by read_corpus)
    // and src/ is subject to the same rules; the file paths are cwd-relative
    // posix, exactly as the oracle reports them.
    let test_files: Vec<(String, String)> = corpus
        .iter()
        .filter(|(p, _)| p.starts_with("tests/") && p.ends_with(".rs"))
        .cloned()
        .collect();
    let src_files: Vec<(String, String)> = corpus
        .iter()
        .filter(|(p, _)| p.starts_with("src/") && p.ends_with(".rs"))
        .cloned()
        .collect();

    let mut findings: Vec<GateFinding> = Vec::new();
    for (path, text) in test_files.iter().chain(src_files.iter()) {
        findings.extend(gate_check_test_file(text, &known_stories, path));
        findings.extend(gate_check_marker_targets(text, &known_reqs, path));
    }
    let model = build_model(&corpus);
    findings.extend(gate_check_backlinks(&model.edges));
    findings.sort();

    let referenced: BTreeSet<String> = gate_collect_referenced(&test_files)
        .intersection(&known_reqs)
        .cloned()
        .collect();

    let mut coverage = Map::new();
    let mut coverage_text_parts = Vec::new();
    for kind in ["functional", "quality", "constraint"] {
        let total = kinds.values().filter(|v| v.kind == kind).count();
        let hit = referenced
            .iter()
            .filter(|r| kinds.get(*r).is_some_and(|v| v.kind == kind))
            .count();
        coverage.insert(
            kind.to_string(),
            json!({ "total": total, "referenced": hit }),
        );
        coverage_text_parts.push(format!("{kind} {hit}/{total}"));
    }

    // Warnings in requirement-id order (the oracle's `sorted(kinds.items())`).
    let warnings: Vec<(String, String, String)> = kinds
        .iter()
        .filter(|(_, info)| !info.declared)
        .map(|(req_id, info)| {
            (
                info.file.clone(),
                "TRC-KIND-001".to_string(),
                format!("requirement {req_id} declares no kind; treated as functional"),
            )
        })
        .collect();

    match format {
        OutputFormat::Json => {
            let payload = json!({
                "findings": findings.iter().map(|(f, l, r, m)| json!({
                    "file": f, "line": l, "rule": r, "message": m,
                })).collect::<Vec<_>>(),
                "warnings": warnings.iter().map(|(f, r, m)| json!({
                    "file": f, "rule": r, "message": m,
                })).collect::<Vec<_>>(),
                "tests_files": test_files.len(),
                "coverage_by_kind": Value::Object(coverage),
            });
            emit_json(&payload);
        }
        OutputFormat::Text => {
            let mut out = String::new();
            for (f, l, r, m) in &findings {
                out.push_str(&format!("{f}:{l}: {r}: {m}\n"));
            }
            for (f, r, m) in &warnings {
                out.push_str(&format!("{f}: {r}: warning: {m}\n"));
            }
            out.push_str(&format!(
                "checked: {} error(s), {} warning(s) — referenced by verifies markers: {}\n",
                findings.len(),
                warnings.len(),
                coverage_text_parts.join(", "),
            ));
            print!("{out}");
        }
    }

    if findings.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rs_verifies(req: &str, ignored: bool) -> String {
        if ignored {
            format!("// arqix:{} {req}\n#[ignore]\nfn t() {{}}\n", "verifies")
        } else {
            format!("// arqix:{} {req}\nfn t() {{}}\n", "verifies")
        }
    }

    fn req_doc(id: &str) -> String {
        format!("---\nid: {id}\n---\nbody\n")
    }

    fn req_doc_iri(id: &str, iri: &str) -> String {
        format!("---\nid: {id}\niri: {iri}\n---\nbody\n")
    }

    // A user story whose frontmatter has-requirements the target, so the model
    // carries a story->requirement edge a story-comparing regression could
    // follow.
    fn story_doc(id: &str, req_iri: &str) -> String {
        format!(
            "---\nid: {id}\nrdf:\n  type:\n    - arqix:classes/user-story\n\
             triples:\n  - predicate: arqix:properties/has-requirement\n    \
             object:\n      - {req_iri}\n---\nbody\n"
        )
    }

    // arqix:verifies REQ-03-01-11-01
    #[test]
    fn freshness_flags_a_marker_whose_requirement_is_newer() {
        let corpus = vec![
            ("docs/req.md".to_string(), req_doc("REQ-99-99-99-01")),
            ("t.rs".to_string(), rs_verifies("REQ-99-99-99-01", false)),
        ];
        let model = build_model(&corpus);
        // Requirement committed after the marker's own file -> possibly stale.
        let touched: HashMap<&str, i64> =
            [("t.rs", 100), ("docs/req.md", 200)].into_iter().collect();
        let (report, _) = freshness(&model, &|p| touched.get(p).copied());
        let stale = report["stale"].as_array().expect("stale array");
        assert_eq!(stale.len(), 1, "the marker is stale: {report}");
        assert_eq!(stale[0]["marker"], "t.rs:1");
        assert_eq!(stale[0]["requirement"], "REQ-99-99-99-01");
        assert_eq!(report["summary"]["evaluated"], 1);
        assert_eq!(report["summary"]["stale"], 1);
    }

    // arqix:verifies REQ-03-01-11-01
    #[test]
    fn freshness_is_silent_when_the_marker_is_newer() {
        let corpus = vec![
            ("docs/req.md".to_string(), req_doc("REQ-99-99-99-01")),
            ("t.rs".to_string(), rs_verifies("REQ-99-99-99-01", false)),
        ];
        let model = build_model(&corpus);
        // Marker's file committed after the requirement -> current, not stale.
        let touched: HashMap<&str, i64> =
            [("t.rs", 200), ("docs/req.md", 100)].into_iter().collect();
        let (report, _) = freshness(&model, &|p| touched.get(p).copied());
        assert_eq!(
            report["summary"]["evaluated"], 1,
            "the active marker was evaluated: {report}"
        );
        assert_eq!(report["stale"].as_array().expect("stale array").len(), 0);
    }

    // arqix:verifies REQ-03-01-11-01
    #[test]
    fn freshness_ignores_owning_story_churn() {
        // ADR-0015: staleness is measured against the requirement document
        // only; the owning story is a grouping layer whose churn is not a
        // signal. The requirement here is OLDER than the marker (so, fresh),
        // but the owning story is the newest file in the corpus. An
        // implementation that also compared the marker against the
        // requirement's story would flag this stale; freshness must stay silent.
        let corpus = vec![
            (
                "docs/story.md".to_string(),
                story_doc("US-99-99-99", "arqix:requirements/req-99-99-99-01"),
            ),
            (
                "docs/req.md".to_string(),
                req_doc_iri("REQ-99-99-99-01", "arqix:requirements/req-99-99-99-01"),
            ),
            ("t.rs".to_string(), rs_verifies("REQ-99-99-99-01", false)),
        ];
        let model = build_model(&corpus);
        // Guard the fixture: the story must actually link to the requirement,
        // otherwise a story-comparing regression would have no edge to follow
        // and the test would pass vacuously.
        assert!(
            model.edges.iter().any(|e| e.from == "US-99-99-99"
                && e.to == "REQ-99-99-99-01"
                && e.kind == "has-requirement"),
            "fixture must link the story to the requirement"
        );
        let touched: HashMap<&str, i64> =
            [("t.rs", 200), ("docs/req.md", 100), ("docs/story.md", 300)]
                .into_iter()
                .collect();
        let (report, code) = freshness(&model, &|p| touched.get(p).copied());
        assert_eq!(
            report["summary"]["evaluated"], 1,
            "the active marker was evaluated: {report}"
        );
        assert_eq!(
            report["stale"].as_array().expect("stale array").len(),
            0,
            "the newer owning story must not make the marker stale: {report}"
        );
        assert_eq!(code, ExitCode::SUCCESS);
    }

    // arqix:verifies REQ-03-01-11-01
    #[test]
    fn freshness_excludes_ignored_skeleton_markers() {
        // One active marker (fresh) plus one ignored marker whose requirement
        // is newer: only the active marker is evaluated, so nothing is stale —
        // an implementation that evaluated ignored markers would report one.
        let corpus = vec![
            ("docs/req1.md".to_string(), req_doc("REQ-99-99-99-01")),
            ("docs/req2.md".to_string(), req_doc("REQ-99-99-99-02")),
            (
                "active.rs".to_string(),
                rs_verifies("REQ-99-99-99-01", false),
            ),
            (
                "skeleton.rs".to_string(),
                rs_verifies("REQ-99-99-99-02", true),
            ),
        ];
        let model = build_model(&corpus);
        let touched: HashMap<&str, i64> = [
            ("active.rs", 200),
            ("docs/req1.md", 100),
            ("skeleton.rs", 100),
            ("docs/req2.md", 200),
        ]
        .into_iter()
        .collect();
        let (report, _) = freshness(&model, &|p| touched.get(p).copied());
        assert_eq!(
            report["summary"]["evaluated"], 1,
            "only the active marker is evaluated: {report}"
        );
        assert_eq!(
            report["stale"].as_array().expect("stale array").len(),
            0,
            "the ignored marker is skipped even though its requirement is newer: {report}"
        );
    }

    // arqix:verifies REQ-03-01-11-02
    #[test]
    fn freshness_treats_missing_history_as_fresh() {
        let corpus = vec![
            ("docs/req.md".to_string(), req_doc("REQ-99-99-99-01")),
            ("t.rs".to_string(), rs_verifies("REQ-99-99-99-01", false)),
        ];
        let model = build_model(&corpus);
        // No timestamps for any path -> considered but degraded to fresh.
        let (report, _) = freshness(&model, &|_| None);
        assert_eq!(
            report["summary"]["evaluated"], 1,
            "the marker is still considered: {report}"
        );
        assert_eq!(report["stale"].as_array().expect("stale array").len(), 0);
    }

    // arqix:no-requirement
    #[test]
    fn story_of_matches_the_oracle_and_never_panics() {
        // Canonical requirement id.
        assert_eq!(story_of("REQ-01-01-08-01"), json!("US-01-01-08"));
        // Cross-cutting foundation domain has no owning story.
        assert_eq!(story_of("REQ-00-00-00-04"), Value::Null);
        // Short and multibyte ids that valid_id admits must not panic and
        // must match the oracle's lenient `req_id[4:12]` codepoint slice.
        assert_eq!(story_of("REQ"), json!("US-"));
        assert_eq!(story_of("PII"), json!("US-"));
        assert_eq!(story_of(""), json!("US-"));
        assert_eq!(story_of("äöüßabcdefgh"), json!("US-abcdefgh"));
    }

    // arqix:no-requirement
    #[test]
    fn markers_are_found_across_python_line_boundaries() {
        // Python splitlines also breaks on form feed, so the oracle sees the
        // marker as line 2 of its own; the engine must number it the same.
        let corpus = vec![
            (
                "docs/r.md".to_string(),
                "---\nid: REQ-99-99-99-01\n---\nbody\n".to_string(),
            ),
            (
                "t.rs".to_string(),
                // Assembled from pieces so the marker gate never reads this
                // literal itself as a marker line of this file.
                format!("\x0c// arqix:{} REQ-99-99-99-01\nfn t() {{}}\n", "verifies"),
            ),
        ];
        let model = build_model(&corpus);
        let edge = model
            .edges
            .iter()
            .find(|e| e.kind == "verifies")
            .expect("marker edge");
        assert_eq!(edge.line, 2, "marker line must use Python line boundaries");
    }

    // arqix:no-requirement
    #[test]
    fn bare_reference_target_is_rejected_like_the_oracle() {
        // MD_REF_MARKER_RE captures `(arqix:\S+)` — at least one character
        // after the colon; a bare `arqix:` is not a target.
        assert_eq!(
            md_reference_marker("<!-- arqix:references-artefact arqix: -->"),
            None
        );
    }

    // arqix:no-requirement
    #[test]
    fn matrix_csv_quotes_fields_like_the_oracle() {
        // The oracle writes the matrix through csv.writer, which quotes any
        // field containing a comma (and doubles embedded quotes).
        let corpus = vec![
            (
                "docs/a,b.md".to_string(),
                "<!-- arqix:verifies REQ-99-99-99-01 -->\n".to_string(),
            ),
            (
                "docs/req.md".to_string(),
                "---\nid: REQ-99-99-99-01\n---\nbody\n".to_string(),
            ),
        ];
        let model = build_model(&corpus);
        let csv = matrix_csv(&model, "req-test");
        assert!(
            csv.contains("\"docs/a,b.md:1\""),
            "a field containing a comma must be quoted: {csv}"
        );
    }

    // arqix:no-requirement
    #[test]
    fn body_reference_marker_becomes_a_resolved_edge() {
        // A `<!-- arqix:references-artefact <iri> -->` body marker is the
        // paragraph-anchored analogue of a frontmatter references-artefact
        // triple: an edge from the enclosing document to the resolved target,
        // carrying the body line and the document file.
        let target = "---\nid: ADR-0005\niri: arqix:adrs/adr-0005\n---\n## t\n";
        let src = "---\nid: unit-icd-01\niri: arqix:units/unit-icd-01\n---\n## t\n\ntext\n<!-- arqix:references-artefact arqix:adrs/adr-0005 -->\n";
        let corpus = vec![
            ("docs/adr.md".to_string(), target.to_string()),
            ("docs/unit.md".to_string(), src.to_string()),
        ];
        let model = build_model(&corpus);
        let edge = model
            .edges
            .iter()
            .find(|e| e.kind == "references-artefact")
            .expect("a references-artefact edge is emitted");
        assert_eq!(edge.from, "unit-icd-01");
        assert_eq!(edge.to, "ADR-0005"); // resolved via iri_map
        assert_eq!(edge.file.as_deref(), Some("docs/unit.md"));
        assert_eq!(edge.line, 8); // the body line of the marker
    }

    // arqix:verifies REQ-03-01-06-04
    #[test]
    fn marker_gate_matches_the_oracle_selftest_cases() {
        // The oracle's SELFTEST_CASES (check_trace_markers.py), ported verbatim.
        // Each source is a single-line literal (with `\n` escapes) so no
        // physical line of this file is itself a whole-line marker the gate
        // would read out of its own source.
        let known_reqs: BTreeSet<String> = ["REQ-01-01-16-01", "REQ-01-01-16-02"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let known_stories: BTreeSet<String> =
            ["US-01-01-16"].iter().map(|s| s.to_string()).collect();
        let cases: Vec<(&str, &str, Vec<&str>)> = vec![
            (
                "clean marked test",
                "// arqix:verifies REQ-01-01-16-01\n#[test]\n#[ignore = \"US-01-01-16: not implemented\"]\nfn a() {\n",
                vec![],
            ),
            (
                "clean no-requirement test",
                "// arqix:no-requirement\n#[test]\nfn a() {\n",
                vec![],
            ),
            (
                "unknown requirement",
                "// arqix:verifies REQ-99-99-99-99\n#[test]\nfn a() {\n",
                vec!["TRC-001"],
            ),
            ("missing marker", "#[test]\nfn a() {\n", vec!["TRC-002"]),
            (
                "bad ignore reason",
                "// arqix:verifies REQ-01-01-16-01\n#[test]\n#[ignore = \"todo\"]\nfn a() {\n",
                vec!["TRC-003"],
            ),
            (
                "unknown story in ignore reason",
                "// arqix:verifies REQ-01-01-16-01\n#[test]\n#[ignore = \"US-99-99-99: not implemented\"]\nfn a() {\n",
                vec!["TRC-003"],
            ),
            (
                "ignore without reason",
                "// arqix:verifies REQ-01-01-16-01\n#[test]\n#[ignore]\nfn a() {\n",
                vec!["TRC-003"],
            ),
            (
                "malformed marker payload",
                "// arqix:verifies REQ-1-2-3\n#[test]\nfn a() {\n",
                vec!["TRC-004"],
            ),
            (
                "contradictory annotations",
                "// arqix:verifies REQ-01-01-16-01\n// arqix:no-requirement\n#[test]\nfn a() {\n",
                vec!["TRC-005"],
            ),
            (
                "marker separated by blank line does not attach",
                "// arqix:verifies REQ-01-01-16-01\n\n#[test]\nfn a() {\n",
                vec!["TRC-002"],
            ),
            (
                "two markers on one test",
                "// arqix:verifies REQ-01-01-16-01\n// arqix:verifies REQ-01-01-16-02\n#[test]\nfn a() {\n",
                vec![],
            ),
            ("helper fn is not a test", "fn helper() {\n", vec![]),
            (
                "trailing marker on the fn line is no marker",
                "#[test]\nfn a() { // arqix:verifies REQ-01-01-16-01\n",
                vec!["TRC-002"],
            ),
            (
                "pub(crate) test fn is still a test",
                "#[test]\npub(crate) fn a() {\n",
                vec!["TRC-002"],
            ),
            (
                "async test fn is still a test",
                "#[test]\nasync fn a() {\n",
                vec!["TRC-002"],
            ),
            (
                "marker above a helper fn is still validated",
                "// arqix:verifies REQ-99-99-99-99\nfn helper() {\n",
                vec!["TRC-001"],
            ),
        ];
        for (name, text, expected) in cases {
            // Mirror run_checks: attachment rules plus global payload validation.
            let mut findings = gate_check_test_file(text, &known_stories, "t.rs");
            findings.extend(gate_check_marker_targets(text, &known_reqs, "t.rs"));
            findings.sort();
            let rules: Vec<&str> = findings.iter().map(|(_, _, r, _)| r.as_str()).collect();
            assert_eq!(rules, expected, "case {name:?}: rules mismatch");
        }
    }

    // arqix:verifies REQ-03-01-06-04
    #[test]
    fn marker_gate_matches_the_oracle_backlink_cases() {
        // The oracle's BACKLINK_CASES: derived-from and has-requirement are
        // double bookkeeping that must stay symmetric.
        let derived_edge = || Edge {
            from: "REQ-01-01-16-01".to_string(),
            to: "US-01-01-16".to_string(),
            kind: "derived-from".to_string(),
            line: 5,
            ignored: false,
            test: None,
            file: Some("r.md".to_string()),
        };
        let backlink_edge = || Edge {
            from: "US-01-01-16".to_string(),
            to: "REQ-01-01-16-01".to_string(),
            kind: "has-requirement".to_string(),
            line: 7,
            ignored: false,
            test: None,
            file: Some("s.md".to_string()),
        };
        let cases: Vec<(&str, Vec<Edge>, Vec<&str>)> = vec![
            (
                "symmetric backlinks are clean",
                vec![derived_edge(), backlink_edge()],
                vec![],
            ),
            (
                "missing has-requirement backlink",
                vec![derived_edge()],
                vec!["TRC-006"],
            ),
            (
                "missing derived-from counterpart",
                vec![backlink_edge()],
                vec!["TRC-006"],
            ),
        ];
        for (name, edges, expected) in cases {
            let findings = gate_check_backlinks(&edges);
            let rules: Vec<&str> = findings.iter().map(|(_, _, r, _)| r.as_str()).collect();
            assert_eq!(rules, expected, "case {name:?}: rules mismatch");
        }
    }

    // The block below ports the remaining behaviours the retired Python
    // oracle's selftest pinned (scripts/arqix_trace.py, removed with the
    // checker retirement) that had no Rust mirror yet, so the executable
    // specification survives the retirement. Marker strings inside fixtures
    // are assembled in pieces so the marker gate never reads them as real
    // markers, following the existing fixture idiom above.

    /// A requirement doc with an explicit rdf type class.
    fn req_doc_kind(id: &str, class: &str) -> String {
        format!("---\nid: {id}\nrdf:\n  type:\n    - arqix:classes/{class}\n---\nbody\n")
    }

    // arqix:no-requirement
    #[test]
    fn md_implements_marker_becomes_an_edge() {
        let marker = format!("<!-- arqix:{} REQ-99-99-99-01 -->\n", "implements");
        let corpus = vec![
            ("docs/req.md".to_string(), req_doc("REQ-99-99-99-01")),
            ("docs/impl.md".to_string(), marker),
        ];
        let model = build_model(&corpus);
        let edges: Vec<_> = model
            .edges
            .iter()
            .filter(|e| e.kind == "implements")
            .collect();
        assert_eq!(edges.len(), 1, "one implements edge");
        assert_eq!(edges[0].to, "REQ-99-99-99-01");
    }

    // arqix:no-requirement
    #[test]
    fn rs_marker_inside_a_string_literal_is_ignored() {
        // The .rs scan requires a `//` comment prefix after trimming; a marker
        // quoted inside a string literal is source text, not a claim.
        let line = format!("    \"// arqix:{} REQ-11-11-11-01\",\n", "verifies");
        let corpus = vec![("t.rs".to_string(), line)];
        let model = build_model(&corpus);
        assert!(model.edges.is_empty(), "no edge from a quoted marker");
    }

    // arqix:no-requirement
    #[test]
    fn body_reference_to_an_unknown_target_stays_visible() {
        // The unresolved analogue of the resolved body-reference test above:
        // the edge keeps the raw IRI instead of disappearing.
        let marker = format!(
            "<!-- arqix:{} arqix:adrs/adr-9999 -->\n",
            "references-artefact"
        );
        let src =
            format!("---\nid: unit-icd-01\niri: arqix:units/unit-icd-01\n---\n## t\n\n{marker}");
        let corpus = vec![("docs/unit.md".to_string(), src)];
        let model = build_model(&corpus);
        let edges: Vec<_> = model
            .edges
            .iter()
            .filter(|e| e.kind == "references-artefact")
            .collect();
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].to, "arqix:adrs/adr-9999", "raw IRI survives");
    }

    // arqix:no-requirement
    #[test]
    fn requirement_discovery_reads_kind_from_frontmatter() {
        let corpus = vec![
            (
                "docs/f.md".to_string(),
                req_doc_kind("REQ-11-11-11-01", "functional-requirement"),
            ),
            (
                "docs/q.md".to_string(),
                req_doc_kind("REQ-11-11-11-02", "quality-requirement"),
            ),
        ];
        let model = build_model(&corpus);
        assert_eq!(model.requirements["REQ-11-11-11-01"].kind, "functional");
        assert_eq!(model.requirements["REQ-11-11-11-02"].kind, "quality");
        assert!(model.requirements["REQ-11-11-11-02"].kind_declared);
    }

    // arqix:no-requirement
    #[test]
    fn unresolved_reference_emits_an_unresolved_requirement_node() {
        let corpus = vec![("t.rs".to_string(), rs_verifies("REQ-99-88-77-66", false))];
        let model = build_model(&corpus);
        let graph = graph(&model);
        let nodes = graph["nodes"].as_array().expect("nodes");
        let unresolved: Vec<_> = nodes
            .iter()
            .filter(|n| n["unresolved"].as_bool() == Some(true))
            .collect();
        assert_eq!(unresolved.len(), 1, "one unresolved node: {graph}");
        assert_eq!(unresolved[0]["id"], "REQ-99-88-77-66");
        assert_eq!(unresolved[0]["type"], "requirement");
    }

    // arqix:no-requirement
    #[test]
    fn uncovered_functional_requirement_is_a_trc_cov_001_error() {
        let corpus = vec![(
            "docs/req.md".to_string(),
            req_doc_kind("REQ-11-11-11-01", "functional-requirement"),
        )];
        let model = build_model(&corpus);
        let (report, code) = coverage(&model, None);
        assert_eq!(code, ExitCode::from(1));
        let diags = report["diagnostics"].as_array().expect("diagnostics");
        assert_eq!(diags.len(), 1, "{report}");
        assert_eq!(diags[0]["code"], "TRC-COV-001");
        assert_eq!(diags[0]["severity"], "error");
        assert_eq!(diags[0]["file"], "docs/req.md");
    }

    // arqix:no-requirement
    #[test]
    fn covered_requirement_without_kind_warns_trc_kind_001() {
        let corpus = vec![
            ("docs/req.md".to_string(), req_doc("REQ-11-11-11-01")),
            ("t.rs".to_string(), rs_verifies("REQ-11-11-11-01", false)),
        ];
        let model = build_model(&corpus);
        let (report, code) = coverage(&model, None);
        // A warning is not an error: the gate stays green.
        assert_eq!(code, ExitCode::SUCCESS);
        let diags = report["diagnostics"].as_array().expect("diagnostics");
        assert_eq!(diags.len(), 1, "{report}");
        assert_eq!(diags[0]["code"], "TRC-KIND-001");
        assert_eq!(diags[0]["severity"], "warning");
    }

    // arqix:no-requirement
    #[test]
    fn trace_json_outputs_carry_the_schema_version() {
        let corpus = vec![
            ("docs/req.md".to_string(), req_doc("REQ-11-11-11-01")),
            ("t.rs".to_string(), rs_verifies("REQ-11-11-11-01", false)),
        ];
        let model = build_model(&corpus);
        assert_eq!(graph(&model)["schema_version"], SCHEMA_VERSION);
        assert_eq!(coverage(&model, None).0["schema_version"], SCHEMA_VERSION);
        assert_eq!(
            check(&model, "REQ-11-11-11-01").0["schema_version"],
            SCHEMA_VERSION
        );
    }

    // arqix:no-requirement
    #[test]
    fn verified_functional_requirement_passes_coverage_with_counts() {
        let corpus = vec![
            (
                "docs/req.md".to_string(),
                req_doc_kind("REQ-11-11-11-01", "functional-requirement"),
            ),
            ("t.rs".to_string(), rs_verifies("REQ-11-11-11-01", false)),
        ];
        let model = build_model(&corpus);
        let (report, code) = coverage(&model, None);
        assert_eq!(code, ExitCode::SUCCESS);
        assert_eq!(
            report["summary"]["functional"],
            serde_json::json!({ "total": 1, "verified": 1, "planned": 0, "uncovered": 0 })
        );
    }

    // arqix:no-requirement
    #[test]
    fn uncovered_quality_requirement_is_not_a_finding() {
        let corpus = vec![(
            "docs/q.md".to_string(),
            req_doc_kind("REQ-11-11-11-02", "quality-requirement"),
        )];
        let model = build_model(&corpus);
        let (report, code) = coverage(&model, None);
        assert_eq!(code, ExitCode::SUCCESS);
        assert!(
            report["diagnostics"]
                .as_array()
                .expect("diagnostics")
                .is_empty(),
            "coverage findings fire for functional requirements only: {report}"
        );
    }

    // arqix:no-requirement
    #[test]
    fn ignored_marker_plans_instead_of_verifying() {
        let corpus = vec![
            (
                "docs/req.md".to_string(),
                req_doc_kind("REQ-11-11-11-01", "functional-requirement"),
            ),
            ("t.rs".to_string(), rs_verifies("REQ-11-11-11-01", true)),
        ];
        let model = build_model(&corpus);
        let (report, code) = coverage(&model, None);
        assert_eq!(code, ExitCode::SUCCESS, "planned is not an error");
        assert_eq!(
            report["summary"]["functional"],
            serde_json::json!({ "total": 1, "verified": 0, "planned": 1, "uncovered": 0 })
        );
        let diags = report["diagnostics"].as_array().expect("diagnostics");
        assert_eq!(diags[0]["code"], "TRC-COV-002");
        assert_eq!(diags[0]["severity"], "warning");
    }

    // arqix:no-requirement
    #[test]
    fn check_reports_locations_and_derivation_for_a_requirement() {
        let req = "---\nid: REQ-11-11-11-01\ntriples:\n  - predicate: arqix:properties/derived-from\n    object: arqix:user-stories/us-11-11-11\n---\nbody\n";
        let story = "---\nid: US-11-11-11\niri: arqix:user-stories/us-11-11-11\n---\nbody\n";
        let corpus = vec![
            ("docs/req.md".to_string(), req.to_string()),
            ("docs/story.md".to_string(), story.to_string()),
            ("t.rs".to_string(), rs_verifies("REQ-11-11-11-01", false)),
        ];
        let model = build_model(&corpus);
        let (report, code) = check(&model, "REQ-11-11-11-01");
        assert_eq!(code, ExitCode::SUCCESS);
        assert_eq!(
            report["verifies"],
            serde_json::json!([{ "file": "t.rs", "line": 1, "ignored": false, "test": "t" }])
        );
        assert_eq!(report["implements"], serde_json::json!([]));
        assert_eq!(report["derived_from"], serde_json::json!(["US-11-11-11"]));
    }

    // arqix:no-requirement
    #[test]
    fn check_on_unknown_requirement_is_a_finding() {
        let corpus = vec![("docs/req.md".to_string(), req_doc("REQ-11-11-11-01"))];
        let model = build_model(&corpus);
        let (report, code) = check(&model, "REQ-00-11-22-33");
        assert_eq!(code, ExitCode::from(1));
        assert_eq!(report["error"], "unknown requirement");
    }

    // arqix:no-requirement
    #[test]
    fn coverage_story_falls_back_to_the_requirement_id() {
        // Without a derived-from triple the owning story derives from the id.
        let corpus = vec![("docs/req.md".to_string(), req_doc("REQ-11-11-11-01"))];
        let model = build_model(&corpus);
        let (report, _) = coverage(&model, None);
        assert_eq!(report["requirements"][0]["story"], "US-11-11-11");
    }

    // arqix:no-requirement
    #[test]
    fn inline_object_and_unresolved_workflow_iri_stay_visible() {
        // An inline (non-list) triple object becomes an edge keeping the raw
        // unresolvable IRI, and the graph shows it as an unknown-typed
        // unresolved node.
        let story = "---\nid: US-22-22-22\nrdf:\n  type:\n    - arqix:classes/user-story\ntriples:\n  - predicate: arqix:properties/is-part-of-workflow\n    object: arqix:workflows/wf-22-22\n---\nbody\n";
        let corpus = vec![("docs/story.md".to_string(), story.to_string())];
        let model = build_model(&corpus);
        assert!(
            model
                .edges
                .iter()
                .any(|e| e.kind == "is-part-of-workflow" && e.to == "arqix:workflows/wf-22-22"),
            "the inline object keeps its raw IRI"
        );
        let graph = graph(&model);
        let unresolved: Vec<_> = graph["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .filter(|n| n["unresolved"].as_bool() == Some(true))
            .cloned()
            .collect();
        assert_eq!(
            unresolved,
            vec![serde_json::json!({
                "id": "arqix:workflows/wf-22-22",
                "type": "unknown",
                "unresolved": true,
            })]
        );
    }

    // arqix:no-requirement
    #[test]
    fn us_req_matrix_lists_derived_pairs() {
        let req = "---\nid: REQ-11-11-11-01\ntriples:\n  - predicate: arqix:properties/derived-from\n    object: arqix:user-stories/us-11-11-11\n---\nbody\n";
        let story = "---\nid: US-11-11-11\niri: arqix:user-stories/us-11-11-11\n---\nbody\n";
        let corpus = vec![
            ("docs/req.md".to_string(), req.to_string()),
            ("docs/story.md".to_string(), story.to_string()),
        ];
        let model = build_model(&corpus);
        let csv = matrix_csv(&model, "us-req");
        let lines: Vec<&str> = csv.lines().collect();
        assert_eq!(lines[0], "story,requirement");
        assert_eq!(lines[1], "US-11-11-11,REQ-11-11-11-01");
    }

    // arqix:no-requirement
    #[test]
    fn story_document_becomes_a_typed_node() {
        let story = "---\nid: US-22-22-22\ntitle: Example Story\nrdf:\n  type:\n    - arqix:classes/user-story\n---\nbody\n";
        let corpus = vec![("docs/story.md".to_string(), story.to_string())];
        let model = build_model(&corpus);
        let graph = graph(&model);
        let node = graph["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .find(|n| n["id"] == "US-22-22-22")
            .expect("story node")
            .clone();
        assert_eq!(node["type"], "user-story");
    }

    // arqix:no-requirement
    #[test]
    fn artefact_nodes_use_the_path_identity() {
        // ADR-0007: an artefact node's id IS its path, with `file` repeating it.
        let corpus = vec![
            ("docs/req.md".to_string(), req_doc("REQ-11-11-11-01")),
            ("a.rs".to_string(), rs_verifies("REQ-11-11-11-01", false)),
        ];
        let model = build_model(&corpus);
        let graph = graph(&model);
        let artefacts: Vec<_> = graph["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .filter(|n| n["type"] == "artefact")
            .cloned()
            .collect();
        assert_eq!(
            artefacts,
            vec![serde_json::json!({ "id": "a.rs", "type": "artefact", "file": "a.rs" })]
        );
    }

    // arqix:no-requirement
    #[test]
    fn model_is_deterministic_regardless_of_input_order() {
        let corpus = vec![
            ("docs/req.md".to_string(), req_doc("REQ-11-11-11-01")),
            (
                "docs/story.md".to_string(),
                story_doc("US-11-11-11", "arqix:requirements/req-11-11-11-01"),
            ),
            ("t.rs".to_string(), rs_verifies("REQ-11-11-11-01", false)),
        ];
        let reversed: Vec<_> = corpus.iter().rev().cloned().collect();
        let a = build_model(&corpus);
        let b = build_model(&reversed);
        assert_eq!(graph(&a), graph(&b), "graph is input-order independent");
        assert_eq!(
            coverage(&a, None).0,
            coverage(&b, None).0,
            "coverage is input-order independent"
        );
    }
}
