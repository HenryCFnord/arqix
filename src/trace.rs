//! Trace Engine: the Rust port of the Python oracle (scripts/arqix_trace.py).
//! Builds the canonical trace graph from markers and frontmatter triples,
//! and projects coverage, per-requirement checks, and matrices (ADR-0006
//! layers, ADR-0007 node identity). The oracle remains the conformance
//! reference: `arqix trace …` must be JSON-value-equal to it on the corpus.

use crate::OutputFormat;
use crate::parser::{self, is_requirement_id};
use serde_json::{Map, Value, json};
use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use std::process::ExitCode;

const SCHEMA_VERSION: u64 = 1;
const SKIP_DIRS: [&str; 5] = [".git", "target", "node_modules", "__pycache__", "fixtures"];
const REQ_KIND_CLASSES: [&str; 3] = [
    "functional-requirement",
    "quality-requirement",
    "constraint",
];

struct Edge {
    from: String,
    to: String,
    kind: String,
    line: usize,
    ignored: bool,
    test: Option<String>,
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

struct DocInfo {
    file: String,
    doctype: String,
}

pub(crate) struct Model {
    requirements: BTreeMap<String, Requirement>,
    documents: BTreeMap<String, DocInfo>,
    edges: Vec<Edge>,
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
            out.push((rel.to_string_lossy().replace('\\', "/"), text));
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

/// Parse `arqix:(verifies|implements)\s+<token>` with only trailing space.
fn marker_body(rest: &str) -> Option<(String, String)> {
    let rest = rest.strip_prefix("arqix:")?;
    let (kind, after) = if let Some(r) = rest.strip_prefix("verifies") {
        ("verifies", r)
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

/// The coverage report as JSON, for downstream consumers.
pub(crate) fn coverage_report(model: &Model) -> (Value, ExitCode) {
    coverage(model)
}

fn coverage(model: &Model) -> (Value, ExitCode) {
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
    for e in &model.edges {
        if (e.kind == "verifies" || e.kind == "implements")
            && let Some(l) = links.get_mut(&e.to)
        {
            if e.kind == "implements" {
                l.implemented.push(e.location());
            } else if e.ignored {
                l.planned.push(e.location());
            } else {
                l.verified.push(e.location());
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
        rows.push(json!({
            "id": id,
            "kind": req.kind,
            "story": owner_story(id, &model.edges),
            "verified_by": l.verified,
            "planned_by": l.planned,
            "implemented_by": l.implemented,
        }));

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
                        "functional requirement {id} is only planned: all verifies markers sit on ignored tests"
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

fn matrix_csv(model: &Model, matrix_type: &str) -> String {
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
    let (report, _) = coverage(model);
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
pub fn coverage_command(format: OutputFormat) -> ExitCode {
    let model = build_model(&read_corpus());
    let (report, code) = coverage(&model);
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
