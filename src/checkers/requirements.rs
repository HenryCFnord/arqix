//! Requirements checker (`arqix lint requirements`): the Rust port of the
//! reference oracle `scripts/check_requirements.py` (retired 2026-07-15 after
//! conformance; see git history). It validates requirement
//! documents under `docs/en/architecture/req/` against the user stories and
//! the authoring rules of `docs/en/processes/requirements-style-guide.md`
//! (RFC 2119 subset + EARS sentence patterns).
//!
//! The port is behaviour-faithful to the oracle: same rule IDs, same message
//! strings, the same `(path, rule, message)` finding order, and the same exit
//! codes (0 no findings, 1 findings — errors OR warnings, 2 usage/I-O error).
//! The retired script's selftest fixtures are mirrored in this module's tests,
//! which own the specification (arc42 chapter 8, oracle policy). It has its
//! own frontmatter reader (not the shared parser, which ports a different oracle), so this
//! module reproduces that reader exactly rather than reusing the store's.

use crate::OutputFormat;
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::Path;
use std::process::ExitCode;
use std::sync::OnceLock;

const REQ_DIR: &str = "docs/en/architecture/req";
const STORY_DIR: &str = "docs/en/architecture/stories";

/// The three requirement subclasses and their coarse kind.
const KIND_CLASSES: [(&str, &str); 3] = [
    ("arqix:classes/functional-requirement", "functional"),
    ("arqix:classes/quality-requirement", "quality"),
    ("arqix:classes/constraint", "constraint"),
];

const ALLOWED_KEYWORDS: [&str; 5] = ["SHALL NOT", "SHALL", "SHOULD NOT", "SHOULD", "MAY"];
const FORBIDDEN_KEYWORDS: [&str; 6] = [
    "MUST NOT",
    "MUST",
    "REQUIRED",
    "NOT RECOMMENDED",
    "RECOMMENDED",
    "OPTIONAL",
];
const REQUIRED_META: [&str; 6] = [
    "lifecycle-status",
    "owner",
    "created",
    "updated",
    "lang",
    "generated",
];

/// Keyword subset expected per kind; a mismatch is the EARS-005 warning.
fn kind_keywords(kind: &str) -> Option<&'static [&'static str]> {
    match kind {
        "functional" => Some(&["SHALL", "SHALL NOT", "MAY"]),
        "quality" => Some(&["SHOULD", "SHOULD NOT", "SHALL", "MAY"]),
        "constraint" => Some(&["SHALL", "SHALL NOT", "MAY"]),
        _ => None,
    }
}

struct Finding {
    path: String,
    rule: &'static str,
    level: &'static str,
    message: String,
}

impl Finding {
    fn error(path: &str, rule: &'static str, message: String) -> Self {
        Finding {
            path: path.to_string(),
            rule,
            level: "error",
            message,
        }
    }

    fn warning(path: &str, rule: &'static str, message: String) -> Self {
        Finding {
            path: path.to_string(),
            rule,
            level: "warning",
            message,
        }
    }
}

// --- compiled patterns (once) -------------------------------------------

struct Patterns {
    req_id: Regex,
    us_id: Regex,
    top_key: Regex,
    triple_pred: Regex,
    triple_obj: Regex,
    triple_obj_item: Regex,
    keyword_search: Regex,
    lowercase_kw: Regex,
    arqix_subject: Regex,
    ears: Vec<(&'static str, Regex)>,
    allowed_word: Vec<Regex>,
    forbidden_word: Vec<Regex>,
}

fn patterns() -> &'static Patterns {
    static P: OnceLock<Patterns> = OnceLock::new();
    P.get_or_init(|| {
        // The EARS core clause: "the <system> <KEYWORD> <response>". A leading
        // article is optional so bare system names ("arqix SHALL ...") work.
        let core = r"(?:[Tt]he\s+)?\S.*?\s(?:SHALL NOT|SHALL|SHOULD NOT|SHOULD|MAY)\s\S.*";
        let ears = vec![
            (
                "unwanted-behaviour",
                Regex::new(&(r"^If\s+.+?,\s+then\s+".to_string() + core + r"\.$")).unwrap(),
            ),
            (
                "event-driven",
                Regex::new(&(r"^When\s+.+?,\s+".to_string() + core + r"\.$")).unwrap(),
            ),
            (
                "state-driven",
                Regex::new(&(r"^While\s+.+?,\s+".to_string() + core + r"\.$")).unwrap(),
            ),
            (
                "optional-feature",
                Regex::new(&(r"^Where\s+.+?,\s+".to_string() + core + r"\.$")).unwrap(),
            ),
            (
                "complex",
                Regex::new(
                    &(r"^(?:(?:While|When|Where)\s+.+?,\s+|If\s+.+?,\s+then\s+){2,}".to_string()
                        + core
                        + r"\.$"),
                )
                .unwrap(),
            ),
            (
                "ubiquitous",
                Regex::new(&(r"^".to_string() + core + r"\.$")).unwrap(),
            ),
        ];
        Patterns {
            req_id: Regex::new(r"^REQ-(\d{2})-(\d{2})-(\d{2})-(\d{2})$").unwrap(),
            us_id: Regex::new(r"^US-(\d{2})-(\d{2})-(\d{2})$").unwrap(),
            top_key: Regex::new(r"^([A-Za-z0-9_-]+):\s*(.*)$").unwrap(),
            triple_pred: Regex::new(r"^-\s*predicate:\s*(.*)$").unwrap(),
            triple_obj: Regex::new(r"^object:\s*(.*)$").unwrap(),
            triple_obj_item: Regex::new(r"^-\s*(arqix:.*)$").unwrap(),
            keyword_search: Regex::new(r"\b(?:SHALL NOT|SHALL|SHOULD NOT|SHOULD|MAY)\b").unwrap(),
            lowercase_kw: Regex::new(r"\b(shall|should|may)\b").unwrap(),
            arqix_subject: Regex::new(r"^`arqix[^`]*`$").unwrap(),
            ears,
            allowed_word: ALLOWED_KEYWORDS
                .iter()
                .map(|&kw| Regex::new(&format!(r"\b{kw}\b")).unwrap())
                .collect(),
            forbidden_word: FORBIDDEN_KEYWORDS
                .iter()
                .map(|&kw| Regex::new(&format!(r"\b{kw}\b")).unwrap())
                .collect(),
        }
    })
}

// --- Python-compatible formatting helpers -------------------------------

/// Reproduce CPython's `repr()` for a string: single quotes unless the value
/// contains a single quote and no double quote, with `\`, the quote, and the
/// control characters escaped. The corpus is ASCII, so the non-ASCII branch
/// (CPython's printable/`\x`/`\u` handling) is not exercised in practice.
fn py_repr(s: &str) -> String {
    let has_single = s.contains('\'');
    let has_double = s.contains('"');
    let quote = if has_single && !has_double { '"' } else { '\'' };
    let mut out = String::with_capacity(s.len() + 2);
    out.push(quote);
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
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

/// `%r` of an optional value: `None` when absent (CPython `repr(None)`).
fn repr_opt(value: Option<&str>) -> String {
    match value {
        Some(s) => py_repr(s),
        None => "None".to_string(),
    }
}

/// `%s` of a Python list of strings: `['a', 'b']` (each element `repr`'d).
fn py_list_repr(items: &[String]) -> String {
    let inner: Vec<String> = items.iter().map(|s| py_repr(s)).collect();
    format!("[{}]", inner.join(", "))
}

/// `%s` of a Python list of ints: `[1, 2, 3]`.
fn py_int_list_repr(items: &[i64]) -> String {
    let inner: Vec<String> = items.iter().map(|n| n.to_string()).collect();
    format!("[{}]", inner.join(", "))
}

// --- frontmatter reader (faithful to the oracle's parse_frontmatter) ----

#[derive(Default)]
struct Triple {
    predicate: String,
    objects: Vec<String>,
}

#[derive(Default)]
struct Fields {
    top: HashMap<String, String>,
    meta: HashMap<String, String>,
    rdf_types: Vec<String>,
    triples: Vec<Triple>,
}

/// Parse the leading `---` block into fields and the body. Returns `None`
/// when no frontmatter block is found (the oracle's `(None, text)` case).
fn parse_frontmatter(text: &str) -> Option<(Fields, String)> {
    let lines: Vec<&str> = text.split('\n').collect();
    let mut idx = 0;
    while idx < lines.len() && lines[idx].trim().is_empty() {
        idx += 1;
    }
    if idx >= lines.len() || lines[idx].trim() != "---" {
        return None;
    }
    let mut end = idx + 1;
    while end < lines.len() && lines[end].trim() != "---" {
        end += 1;
    }
    if end >= lines.len() {
        return None;
    }

    let fm_lines = &lines[idx + 1..end];
    let body = lines[end + 1..].join("\n");

    let p = patterns();
    let mut fields = Fields::default();
    let mut section: Option<String> = None;
    let mut current_triple: Option<usize> = None;

    for raw in fm_lines {
        if raw.trim().is_empty() || raw.trim_start().starts_with('#') {
            continue;
        }
        let indent = raw.len() - raw.trim_start_matches(' ').len();
        let line = raw.trim();

        if indent == 0 {
            current_triple = None;
            let caps = match p.top_key.captures(line) {
                Some(c) => c,
                None => continue,
            };
            let key = caps.get(1).unwrap().as_str().to_string();
            let value = caps.get(2).unwrap().as_str().trim().to_string();
            section = Some(key.clone());
            if value != "{}" && value != "[]" && !value.is_empty() {
                fields.top.insert(key, value);
            }
        } else if section.as_deref() == Some("rdf") && line.starts_with("- ") {
            fields.rdf_types.push(line[2..].trim().to_string());
        } else if section.as_deref() == Some("triples") {
            if let Some(caps) = p.triple_pred.captures(line) {
                let predicate = caps.get(1).unwrap().as_str().trim().to_string();
                fields.triples.push(Triple {
                    predicate,
                    objects: Vec::new(),
                });
                current_triple = Some(fields.triples.len() - 1);
                continue;
            }
            if let Some(caps) = p.triple_obj.captures(line) {
                let value = caps.get(1).unwrap().as_str().trim().to_string();
                if let Some(ti) = current_triple
                    && !value.is_empty()
                {
                    fields.triples[ti].objects.push(value);
                }
                continue;
            }
            if let Some(caps) = p.triple_obj_item.captures(line) {
                let value = caps.get(1).unwrap().as_str().trim().to_string();
                if let Some(ti) = current_triple {
                    fields.triples[ti].objects.push(value);
                }
            }
        } else if section.as_deref() == Some("meta")
            && let Some(caps) = p.top_key.captures(line)
        {
            let value = caps.get(2).unwrap().as_str().trim();
            if !value.is_empty() {
                fields
                    .meta
                    .insert(caps.get(1).unwrap().as_str().to_string(), value.to_string());
            }
        }
    }
    Some((fields, body))
}

fn triple_objects(fields: &Fields, predicate: &str) -> Vec<String> {
    let mut objs = Vec::new();
    for t in &fields.triples {
        if t.predicate == predicate {
            objs.extend(t.objects.iter().cloned());
        }
    }
    objs
}

/// The non-comment content lines of the `## Requirement` section.
fn normative_sentences(body: &str) -> Vec<String> {
    let mut collected = Vec::new();
    let mut in_section = false;
    for line in body.split('\n') {
        let stripped = line.trim();
        if stripped.starts_with("## ") || stripped.starts_with("### ") {
            in_section = stripped == "## Requirement";
            continue;
        }
        if in_section && !stripped.is_empty() && !stripped.starts_with("<!--") {
            collected.push(stripped.to_string());
        }
    }
    collected
}

/// The first matching EARS pattern name, or `None`.
fn classify_sentence(sentence: &str) -> Option<&'static str> {
    for (name, re) in &patterns().ears {
        if re.is_match(sentence) {
            return Some(name);
        }
    }
    None
}

/// Allowed keywords found in the sentence and the residue after removing
/// them (NOT-forms matched first so `SHALL NOT` is not double-counted).
fn keywords_in(sentence: &str) -> (Vec<&'static str>, String) {
    let p = patterns();
    let mut found: Vec<&'static str> = Vec::new();
    let mut remaining = sentence.to_string();
    for (i, kw) in ALLOWED_KEYWORDS.iter().enumerate() {
        let re = &p.allowed_word[i];
        let count = re.find_iter(&remaining).count();
        if count > 0 {
            for _ in 0..count {
                found.push(kw);
            }
            remaining = re.replace_all(&remaining, " ").into_owned();
        }
    }
    (found, remaining)
}

/// The core-clause subject: text between the last clause comma (or sentence
/// start) and the normative keyword, without a leading `then`.
fn core_subject(sentence: &str) -> Option<String> {
    let m = patterns().keyword_search.find(sentence)?;
    let prefix = &sentence[..m.start()];
    let prefix = match prefix.rsplit_once(',') {
        Some((_, after)) => after,
        None => prefix,
    };
    let mut subject = prefix.trim().to_string();
    if let Some(rest) = subject.strip_prefix("then ") {
        subject = rest.trim().to_string();
    }
    Some(subject)
}

fn check_sentence(
    path: &str,
    sentence: &str,
    kind: Option<&str>,
    findings: &mut Vec<Finding>,
) -> Option<&'static str> {
    let p = patterns();
    let pattern = classify_sentence(sentence);
    if pattern.is_none() {
        findings.push(Finding::error(
            path,
            "EARS-002",
            format!(
                "sentence does not match any EARS pattern: {}",
                py_repr(sentence)
            ),
        ));
    }

    for (i, kw) in FORBIDDEN_KEYWORDS.iter().enumerate() {
        if p.forbidden_word[i].is_match(sentence) {
            findings.push(Finding::error(
                path,
                "EARS-003",
                format!("forbidden keyword '{kw}'; use the SHALL/SHOULD/MAY subset"),
            ));
            break;
        }
    }

    let (found, remaining) = keywords_in(sentence);
    if p.lowercase_kw.is_match(&remaining) {
        findings.push(Finding::error(
            path,
            "EARS-003",
            "lowercase normative keyword in the requirement sentence".to_string(),
        ));
    }
    if found.len() != 1 {
        let list = if found.is_empty() {
            "none".to_string()
        } else {
            found.join(", ")
        };
        findings.push(Finding::error(
            path,
            "EARS-004",
            format!(
                "expected exactly one normative keyword, found {} ({})",
                found.len(),
                list
            ),
        ));
    } else {
        if let Some(kw_set) = kind.and_then(kind_keywords)
            && !kw_set.contains(&found[0])
        {
            findings.push(Finding::warning(
                path,
                "EARS-005",
                format!(
                    "keyword '{}' unusual for kind '{}' (see style guide matrix)",
                    found[0],
                    kind.unwrap()
                ),
            ));
        }
        if let Some(subject) = core_subject(sentence)
            && subject.to_lowercase().contains("arqix")
        {
            let allowed = subject == "arqix"
                || subject == "The arqix CLI"
                || p.arqix_subject.is_match(&subject);
            if !allowed {
                findings.push(Finding::warning(
                    path,
                    "EARS-006",
                    format!(
                        "subject {} is not an allowed arqix subject form \
                         ('arqix', 'The arqix CLI', or a backticked command)",
                        py_repr(&subject)
                    ),
                ));
            }
        }
    }
    pattern
}

/// Structural + sentence checks for one requirement document. Returns
/// `(req_id, derived_from_objects)` for the cross-file checks, or `None`.
fn check_requirement_file(
    path: &str,
    filename: &str,
    text: &str,
    findings: &mut Vec<Finding>,
) -> Option<(String, Vec<String>)> {
    let (fields, body) = match parse_frontmatter(text) {
        Some(x) => x,
        None => {
            findings.push(Finding::error(
                path,
                "REQ-ID-001",
                "missing frontmatter".to_string(),
            ));
            return None;
        }
    };
    let req_id = fields.top.get("id").cloned().unwrap_or_default();

    let caps = match patterns().req_id.captures(&req_id) {
        Some(c) => c,
        None => {
            findings.push(Finding::error(
                path,
                "REQ-ID-002",
                format!("id {} does not match REQ-XX-YY-ZZ-NN", py_repr(&req_id)),
            ));
            return None;
        }
    };
    let (g1, g2, g3) = (
        caps.get(1).unwrap().as_str().to_string(),
        caps.get(2).unwrap().as_str().to_string(),
        caps.get(3).unwrap().as_str().to_string(),
    );

    let prefix = format!("{req_id}-");
    if !filename.starts_with(&prefix) {
        findings.push(Finding::error(
            path,
            "REQ-ID-001",
            format!("id {req_id} is not the filename prefix"),
        ));
    }
    let expected_iri = format!("arqix:requirements/{}", req_id.to_lowercase());
    let top_iri = fields.top.get("iri").map(String::as_str);
    if top_iri != Some(expected_iri.as_str()) {
        findings.push(Finding::error(
            path,
            "REQ-ID-003",
            format!(
                "iri {}, expected {}",
                repr_opt(top_iri),
                py_repr(&expected_iri)
            ),
        ));
    }
    let slug = fields.top.get("slug").cloned().unwrap_or_default();
    if filename.starts_with(&prefix) {
        let start = req_id.len() + 1;
        let end = filename.len().saturating_sub(3);
        let tail = if start <= end {
            &filename[start..end]
        } else {
            ""
        };
        if slug != tail {
            findings.push(Finding::error(
                path,
                "REQ-ID-004",
                format!(
                    "slug {} does not match filename tail {}",
                    py_repr(&slug),
                    py_repr(tail)
                ),
            ));
        }
    }

    let kinds: Vec<&'static str> = fields
        .rdf_types
        .iter()
        .filter_map(|t| KIND_CLASSES.iter().find(|(k, _)| k == t).map(|(_, v)| *v))
        .collect();
    let kind: Option<&str> = if kinds.len() == 1 {
        Some(kinds[0])
    } else {
        None
    };
    if kinds.len() != 1 {
        let found = if fields.rdf_types.is_empty() {
            "none".to_string()
        } else {
            fields.rdf_types.join(", ")
        };
        findings.push(Finding::error(
            path,
            "REQ-KIND-001",
            format!("rdf.type must be exactly one requirement subclass, found: {found}"),
        ));
    }

    for key in REQUIRED_META {
        if !fields.meta.contains_key(key) {
            findings.push(Finding::error(
                path,
                "REQ-META-001",
                format!("meta.{key} missing or empty"),
            ));
        }
    }

    let derived = triple_objects(&fields, "arqix:properties/derived-from");
    if (g1.as_str(), g2.as_str(), g3.as_str()) == ("00", "00", "00") {
        // Distinct objects: listing the same story twice is one link.
        let distinct: BTreeSet<&String> = derived.iter().collect();
        if distinct.len() < 2 {
            findings.push(Finding::error(
                path,
                "REQ-LNK-002",
                format!(
                    "cross-cutting requirement needs >= 2 distinct derived-from objects, found {}",
                    distinct.len()
                ),
            ));
        }
    } else {
        let owner_iri = format!("arqix:user-stories/us-{g1}-{g2}-{g3}");
        if derived.is_empty() || derived[0] != owner_iri {
            let found = if derived.is_empty() {
                "none".to_string()
            } else {
                py_list_repr(&derived)
            };
            findings.push(Finding::error(
                path,
                "REQ-LNK-001",
                format!(
                    "story-bound requirement must list its owning story {} as the \
                     first derived-from object, found {}",
                    py_repr(&owner_iri),
                    found
                ),
            ));
        }
    }

    let sentences = normative_sentences(&body);
    if sentences.len() != 1 {
        findings.push(Finding::error(
            path,
            "EARS-001",
            format!(
                "'## Requirement' must contain exactly one normative sentence, found {}",
                sentences.len()
            ),
        ));
    }
    if let Some(sentence) = sentences.first() {
        check_sentence(path, sentence, kind, findings);
    }

    Some((req_id, derived))
}

/// One user story's coupling-relevant frontmatter.
struct StoryRecord {
    path: String,
    id: String,
    personas: Vec<String>,
    workflows: Vec<String>,
}

// arqix:implements REQ-01-01-11-08
// arqix:implements REQ-01-01-11-09
fn story_workflow_checks(
    _stories: &BTreeMap<String, StoryRecord>,
    _workflows: &BTreeMap<String, BTreeSet<String>>,
    _consolidation: &BTreeSet<String>,
    _findings: &mut Vec<Finding>,
) {
}

fn load_stories(
    story_dir: &str,
    findings: &mut Vec<Finding>,
) -> BTreeMap<String, (String, Vec<String>)> {
    let mut stories = BTreeMap::new();
    for path in sorted_md_files(story_dir) {
        let text = read_universal(&path);
        let fields = match parse_frontmatter(&text) {
            Some((f, _)) => f,
            None => continue,
        };
        let story_id = fields.top.get("id").cloned().unwrap_or_default();
        if !patterns().us_id.is_match(&story_id) {
            findings.push(Finding::error(
                &path,
                "US-ID-001",
                format!(
                    "story id {} does not match the US-XX-YY-ZZ scheme",
                    py_repr(&story_id)
                ),
            ));
            continue;
        }
        let iri = format!("arqix:user-stories/{}", story_id.to_lowercase());
        let has_req = triple_objects(&fields, "arqix:properties/has-requirement");
        stories.insert(iri, (path, has_req));
    }
    stories
}

fn cross_file_checks(
    requirements: &BTreeMap<String, (String, Vec<String>)>,
    stories: &BTreeMap<String, (String, Vec<String>)>,
    allow_unlinked: bool,
    findings: &mut Vec<Finding>,
) {
    for (req_iri, (path, derived)) in requirements {
        for story_iri in derived {
            if !stories.contains_key(story_iri) {
                findings.push(Finding::error(
                    path,
                    "REQ-LNK-003",
                    format!("derived-from references missing story {story_iri}"),
                ));
            } else if !stories[story_iri].1.contains(req_iri) {
                findings.push(Finding::error(
                    path,
                    "REQ-LNK-005",
                    format!(
                        "derived-from {story_iri} has no matching has-requirement in the story"
                    ),
                ));
            }
        }
    }

    for (story_iri, (path, has_req)) in stories {
        for req_iri in has_req {
            if !requirements.contains_key(req_iri) {
                findings.push(Finding::error(
                    path,
                    "REQ-LNK-004",
                    format!("has-requirement references missing requirement {req_iri}"),
                ));
            } else if !requirements[req_iri].1.contains(story_iri) {
                findings.push(Finding::error(
                    path,
                    "REQ-LNK-005",
                    format!("has-requirement {req_iri} has no matching derived-from"),
                ));
            }
        }
        if has_req.is_empty() && !allow_unlinked {
            findings.push(Finding::warning(
                path,
                "REQ-LNK-006",
                "story has no has-requirement link".to_string(),
            ));
        }
    }
}

fn sequence_checks(req_ids: &[(String, String)], findings: &mut Vec<Finding>, req_dir: &str) {
    let mut groups: BTreeMap<(String, String, String), Vec<i64>> = BTreeMap::new();
    let mut seen: HashMap<String, String> = HashMap::new();
    for (req_id, path) in req_ids {
        if let Some(first_name) = seen.get(req_id) {
            findings.push(Finding::error(
                path,
                "REQ-ID-005",
                format!("duplicate id {req_id} (also in {first_name})"),
            ));
            continue;
        }
        seen.insert(req_id.clone(), basename(path));
        let caps = patterns().req_id.captures(req_id).unwrap();
        let key = (
            caps.get(1).unwrap().as_str().to_string(),
            caps.get(2).unwrap().as_str().to_string(),
            caps.get(3).unwrap().as_str().to_string(),
        );
        let nn: i64 = caps.get(4).unwrap().as_str().parse().unwrap();
        groups.entry(key).or_default().push(nn);
    }
    for (domain, numbers) in &groups {
        let mut sorted_nums = numbers.clone();
        sorted_nums.sort_unstable();
        let max = *sorted_nums.iter().max().unwrap();
        let expected: Vec<i64> = (1..=max).collect();
        if sorted_nums != expected {
            findings.push(Finding::error(
                req_dir,
                "REQ-ID-006",
                format!(
                    "domain {}-{}-{}: NN sequence {} is not contiguous from 01",
                    domain.0,
                    domain.1,
                    domain.2,
                    py_int_list_repr(&sorted_nums)
                ),
            ));
        }
    }
}

fn run_checks(allow_unlinked: bool) -> Option<Vec<Finding>> {
    let mut findings = Vec::new();
    if !Path::new(STORY_DIR).is_dir() {
        eprintln!("error: story directory not found: {STORY_DIR}");
        return None;
    }

    let stories = load_stories(STORY_DIR, &mut findings);

    let mut requirements: BTreeMap<String, (String, Vec<String>)> = BTreeMap::new();
    let mut req_ids: Vec<(String, String)> = Vec::new();
    if Path::new(REQ_DIR).is_dir() {
        for path in sorted_md_files(REQ_DIR) {
            let text = read_universal(&path);
            let name = basename(&path);
            if let Some((req_id, derived)) =
                check_requirement_file(&path, &name, &text, &mut findings)
            {
                req_ids.push((req_id.clone(), path.clone()));
                requirements.insert(
                    format!("arqix:requirements/{}", req_id.to_lowercase()),
                    (path, derived),
                );
            }
        }
        sequence_checks(&req_ids, &mut findings, REQ_DIR);
    }

    cross_file_checks(&requirements, &stories, allow_unlinked, &mut findings);
    findings.sort_by(|a, b| {
        (a.path.as_str(), a.rule, a.message.as_str()).cmp(&(
            b.path.as_str(),
            b.rule,
            b.message.as_str(),
        ))
    });
    Some(findings)
}

fn report(findings: &[Finding], format: OutputFormat) -> ExitCode {
    let errors = findings.iter().filter(|f| f.level == "error").count();
    let warnings = findings.len() - errors;
    match format {
        OutputFormat::Json => emit_json(findings, errors, warnings),
        OutputFormat::Text => {
            for f in findings {
                println!("{}: [{}] {}: {}", f.level, f.rule, f.path, f.message);
            }
            println!("checked: {errors} error(s), {warnings} warning(s)");
        }
    }
    // Any finding — error OR warning — is a non-zero exit, exactly like the
    // oracle's `return 1 if findings else 0`.
    if findings.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

/// Emit the report in the oracle's `json.dumps(..., indent=2)` shape (keys in
/// insertion order: findings → [path, rule, level, message], then summary),
/// so the output is byte-identical for the ASCII corpus.
fn emit_json(findings: &[Finding], errors: usize, warnings: usize) {
    let mut out = String::from("{\n  \"findings\": ");
    if findings.is_empty() {
        out.push_str("[]");
    } else {
        out.push_str("[\n");
        for (i, f) in findings.iter().enumerate() {
            out.push_str("    {\n");
            out.push_str(&format!("      \"path\": {},\n", json_string(&f.path)));
            out.push_str(&format!("      \"rule\": {},\n", json_string(f.rule)));
            out.push_str(&format!("      \"level\": {},\n", json_string(f.level)));
            out.push_str(&format!("      \"message\": {}\n", json_string(&f.message)));
            out.push_str("    }");
            out.push_str(if i + 1 < findings.len() { ",\n" } else { "\n" });
        }
        out.push_str("  ]");
    }
    out.push_str(&format!(
        ",\n  \"summary\": {{\n    \"errors\": {errors},\n    \"warnings\": {warnings}\n  }}\n}}"
    ));
    println!("{out}");
}

/// A JSON string literal with the standard escapes (`serde_json` matches
/// CPython's `json.dumps` for ASCII input, which the corpus is).
fn json_string(s: &str) -> String {
    serde_json::to_string(s).expect("string serialises")
}

// arqix:implements REQ-01-01-11-06
/// `arqix lint requirements`.
pub fn lint(format: OutputFormat, allow_unlinked_stories: bool) -> ExitCode {
    match run_checks(allow_unlinked_stories) {
        Some(findings) => report(&findings, format),
        None => ExitCode::from(2),
    }
}

// --- filesystem helpers -------------------------------------------------

/// The `*.md` files under `dir`, recursively, sorted by path string — the
/// oracle's `sorted(dir.rglob("*.md"))`.
fn sorted_md_files(dir: &str) -> Vec<String> {
    let mut files = Vec::new();
    collect_md(Path::new(dir), &mut files);
    files.sort();
    files
}

fn collect_md(dir: &Path, out: &mut Vec<String>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            collect_md(&p, out);
        } else if p.extension().and_then(|e| e.to_str()) == Some("md") {
            out.push(p.to_string_lossy().replace('\\', "/"));
        }
    }
}

/// The basename of a POSIX-style path.
fn basename(path: &str) -> String {
    path.rsplit('/').next().unwrap_or(path).to_string()
}

/// Read a file the way CPython's `read_text` does: UTF-8 with universal
/// newline translation (`\r\n` and lone `\r` become `\n`).
fn read_universal(path: &str) -> String {
    let raw = std::fs::read_to_string(path).unwrap_or_default();
    raw.replace("\r\n", "\n").replace('\r', "\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rules_of(findings: &[Finding]) -> Vec<&str> {
        let mut set: BTreeSet<&str> = BTreeSet::new();
        for f in findings {
            set.insert(f.rule);
        }
        set.into_iter().collect()
    }

    // The oracle's GOOD_REQ selftest fixture, verbatim.
    const GOOD_REQ: &str = "---\nid: REQ-01-01-01-01\ntitle: Test Requirement\nslug: test-requirement\niri: arqix:requirements/req-01-01-01-01\n\nrdf:\n  type:\n    - arqix:classes/functional-requirement\n\ntriples:\n  - predicate: arqix:properties/derived-from\n    object: arqix:user-stories/us-01-01-01\n  - predicate: arqix:properties/has-verification-method\n    object:\n\nproperties:\n  priority: high\n  fit-criterion: deterministic\n\nmeta:\n  lifecycle-status: draft\n  owner: hcf\n  created: 2026-07-02\n  updated: 2026-07-02\n  lang: en\n  generated: false\n---\n\n## Requirement\n\nWhen `arqix doc new` is invoked without `--id`, arqix SHALL generate a unique document ID from the configured policy.\n\n### Notes\n\nSelftest fixture.\n";

    const GOOD_NAME: &str = "REQ-01-01-01-01-test-requirement.md";

    // arqix:verifies REQ-01-01-11-06
    #[test]
    fn sentence_checks_match_the_oracle_selftest_cases() {
        // The oracle's SELFTEST_SENTENCES: (sentence, kind, expected pattern,
        // expected rule IDs), ported verbatim.
        let cases: Vec<(&str, &str, Option<&str>, Vec<&str>)> = vec![
            (
                "The arqix CLI SHALL NOT write outside the declared scope.",
                "constraint",
                Some("ubiquitous"),
                vec![],
            ),
            (
                "When a lint finding occurs, arqix SHALL exit non-zero.",
                "functional",
                Some("event-driven"),
                vec![],
            ),
            (
                "While assembly runs, the logger SHALL append JSONL records.",
                "functional",
                Some("state-driven"),
                vec![],
            ),
            (
                "If an include target is missing, then the assembler SHALL fail.",
                "functional",
                Some("unwanted-behaviour"),
                vec![],
            ),
            (
                "Where MCP is enabled, the server MAY expose a read tool.",
                "functional",
                Some("optional-feature"),
                vec![],
            ),
            (
                "The lint diagnostics SHOULD be actionable without reading source.",
                "quality",
                Some("ubiquitous"),
                vec![],
            ),
            (
                "The tool MUST reject unknown flags.",
                "functional",
                None,
                vec!["EARS-002", "EARS-003", "EARS-004"],
            ),
            (
                "arqix shall do things.",
                "functional",
                None,
                vec!["EARS-002", "EARS-003", "EARS-004"],
            ),
            (
                "The tool SHALL do this and SHOULD do that.",
                "functional",
                Some("ubiquitous"),
                vec!["EARS-004"],
            ),
            (
                "Renders pages quickly.",
                "functional",
                None,
                vec!["EARS-002", "EARS-004"],
            ),
            (
                "The exporter SHOULD emit stable ordering.",
                "constraint",
                Some("ubiquitous"),
                vec!["EARS-005"],
            ),
            (
                "The arqix formatter SHALL NOT emit noise.",
                "constraint",
                Some("ubiquitous"),
                vec!["EARS-006"],
            ),
            (
                "If a value is current, then `arqix finalise` SHALL NOT rewrite it.",
                "functional",
                Some("unwanted-behaviour"),
                vec![],
            ),
            (
                "When `arqix fmt` runs, arqix SHALL sort keys deterministically.",
                "functional",
                Some("event-driven"),
                vec![],
            ),
        ];
        for (sentence, kind, expected_pattern, expected_rules) in cases {
            let mut findings = Vec::new();
            let pattern = check_sentence("selftest", sentence, Some(kind), &mut findings);
            assert_eq!(pattern, expected_pattern, "pattern for {sentence:?}");
            assert_eq!(
                rules_of(&findings),
                expected_rules,
                "rules for {sentence:?}"
            );
        }
    }

    // arqix:verifies REQ-01-01-11-06
    #[test]
    fn good_fixture_parses_clean() {
        let mut findings = Vec::new();
        let result = check_requirement_file("selftest", GOOD_NAME, GOOD_REQ, &mut findings);
        assert!(
            findings.is_empty(),
            "unexpected findings: {:?}",
            rules_of(&findings)
        );
        let (req_id, derived) = result.expect("clean parse");
        assert_eq!(req_id, "REQ-01-01-01-01");
        assert_eq!(derived, vec!["arqix:user-stories/us-01-01-01".to_string()]);
    }

    // arqix:verifies REQ-01-01-11-06
    #[test]
    fn bad_fixture_reports_kind_and_iri() {
        let bad = GOOD_REQ
            .replace(
                "arqix:classes/functional-requirement",
                "arqix:classes/requirement",
            )
            .replace(
                "iri: arqix:requirements/req-01-01-01-01",
                "iri: arqix:requirements/wrong",
            );
        let mut findings = Vec::new();
        check_requirement_file("selftest", GOOD_NAME, &bad, &mut findings);
        assert_eq!(rules_of(&findings), vec!["REQ-ID-003", "REQ-KIND-001"]);
    }

    // arqix:verifies REQ-01-01-11-06
    #[test]
    fn canonical_owner_fixture_is_clean_with_multiple_stories() {
        let multi = GOOD_REQ.replace(
            "object: arqix:user-stories/us-01-01-01",
            "object:\n      - arqix:user-stories/us-01-01-01\n      - arqix:user-stories/us-02-01-01",
        );
        let mut findings = Vec::new();
        check_requirement_file("selftest", GOOD_NAME, &multi, &mut findings);
        assert!(findings.is_empty(), "unexpected: {:?}", rules_of(&findings));
    }

    // arqix:verifies REQ-01-01-11-06
    #[test]
    fn foreign_owner_fixture_reports_lnk_001() {
        let foreign = GOOD_REQ.replace(
            "object: arqix:user-stories/us-01-01-01",
            "object:\n      - arqix:user-stories/us-02-01-01\n      - arqix:user-stories/us-01-01-01",
        );
        let mut findings = Vec::new();
        check_requirement_file("selftest", GOOD_NAME, &foreign, &mut findings);
        assert_eq!(rules_of(&findings), vec!["REQ-LNK-001"]);
    }

    // arqix:verifies REQ-01-01-11-06
    #[test]
    fn missing_generated_meta_reports_meta_001() {
        let missing = GOOD_REQ.replace("  generated: false\n", "");
        let mut findings = Vec::new();
        check_requirement_file("selftest", GOOD_NAME, &missing, &mut findings);
        assert_eq!(rules_of(&findings), vec!["REQ-META-001"]);
    }

    // arqix:verifies REQ-01-01-11-06
    #[test]
    fn missing_frontmatter_reports_id_001() {
        let mut findings = Vec::new();
        let result = check_requirement_file("selftest", "x.md", "no frontmatter\n", &mut findings);
        assert!(result.is_none());
        assert_eq!(rules_of(&findings), vec!["REQ-ID-001"]);
        assert_eq!(findings[0].message, "missing frontmatter");
    }

    // The message strings carry Python-`repr` (`%r`) and list-repr formatting;
    // the clean corpus never exercises them, so pin them here directly.
    // arqix:verifies REQ-01-01-11-06
    #[test]
    fn finding_messages_match_the_oracle_formatting() {
        // REQ-ID-002: `%r` of the offending id.
        let mut findings = Vec::new();
        check_requirement_file("p.md", "p.md", "---\nid: bogus\n---\nbody\n", &mut findings);
        assert!(
            findings.iter().any(|f| f.rule == "REQ-ID-002"
                && f.message == "id 'bogus' does not match REQ-XX-YY-ZZ-NN")
        );

        // REQ-ID-003 with a missing iri renders `repr(None)` == "None".
        let no_iri = "---\nid: REQ-01-01-01-01\nslug: test-requirement\nrdf:\n  type:\n    - arqix:classes/functional-requirement\ntriples:\n  - predicate: arqix:properties/derived-from\n    object: arqix:user-stories/us-01-01-01\nmeta:\n  lifecycle-status: active\n  owner: hcf\n  created: 2026-07-13\n  updated: 2026-07-13\n  lang: en\n  generated: false\n---\n\n## Requirement\n\nWhen `arqix fmt` runs, arqix SHALL sort keys deterministically.\n";
        let mut findings = Vec::new();
        check_requirement_file(
            "REQ-01-01-01-01-test-requirement.md",
            "REQ-01-01-01-01-test-requirement.md",
            no_iri,
            &mut findings,
        );
        assert!(findings.iter().any(|f| f.rule == "REQ-ID-003"
            && f.message == "iri None, expected 'arqix:requirements/req-01-01-01-01'"));

        // REQ-LNK-001 renders the derived list with Python list-repr.
        let foreign = GOOD_REQ.replace(
            "object: arqix:user-stories/us-01-01-01",
            "object:\n      - arqix:user-stories/us-02-01-01\n      - arqix:user-stories/us-01-01-01",
        );
        let mut findings = Vec::new();
        check_requirement_file("selftest", GOOD_NAME, &foreign, &mut findings);
        assert!(findings.iter().any(|f| f.rule == "REQ-LNK-001"
            && f.message == "story-bound requirement must list its owning story 'arqix:user-stories/us-01-01-01' as the first derived-from object, found ['arqix:user-stories/us-02-01-01', 'arqix:user-stories/us-01-01-01']"));
    }

    // arqix:verifies REQ-01-01-11-06
    #[test]
    fn cross_cutting_requirement_needs_two_distinct_stories() {
        // A 00-00-00 requirement listing one story twice is still one link.
        let one = GOOD_REQ
            .replace("id: REQ-01-01-01-01", "id: REQ-00-00-00-01")
            .replace(
                "iri: arqix:requirements/req-01-01-01-01",
                "iri: arqix:requirements/req-00-00-00-01",
            )
            .replace(
                "object: arqix:user-stories/us-01-01-01",
                "object:\n      - arqix:user-stories/us-01-01-01\n      - arqix:user-stories/us-01-01-01",
            );
        let mut findings = Vec::new();
        check_requirement_file(
            "REQ-00-00-00-01-test-requirement.md",
            "REQ-00-00-00-01-test-requirement.md",
            &one,
            &mut findings,
        );
        assert!(findings.iter().any(|f| f.rule == "REQ-LNK-002"));
    }

    // arqix:no-requirement
    #[test]
    fn py_repr_matches_cpython_quote_selection() {
        assert_eq!(py_repr("plain"), "'plain'");
        // A single quote and no double quote switches to double quotes.
        assert_eq!(py_repr("it's"), "\"it's\"");
        // Both present keeps single quotes and escapes the single quote.
        assert_eq!(py_repr("it's \"x\""), "'it\\'s \"x\"'");
        assert_eq!(py_repr("a\\b"), "'a\\\\b'");
    }

    // arqix:no-requirement
    #[test]
    fn frontmatter_reader_handles_inline_and_list_objects() {
        let (fields, _) = parse_frontmatter(GOOD_REQ).expect("frontmatter");
        assert_eq!(
            fields.top.get("id").map(String::as_str),
            Some("REQ-01-01-01-01")
        );
        assert_eq!(
            fields.rdf_types,
            vec!["arqix:classes/functional-requirement"]
        );
        assert_eq!(
            triple_objects(&fields, "arqix:properties/derived-from"),
            vec!["arqix:user-stories/us-01-01-01".to_string()]
        );
        assert!(fields.meta.contains_key("generated"));
        // An empty `{}`/`[]` top value sets the section but is not stored.
        let (f2, _) =
            parse_frontmatter("---\nexternal-references: []\nmeta:\n  lang: en\n---\nbody\n")
                .expect("fm");
        assert!(!f2.top.contains_key("external-references"));
    }

    // --- story-workflow coupling (US-WF-001, US-PER-001) -----------------

    fn story(id: &str, persona: &str, workflow: &str) -> (String, StoryRecord) {
        (
            format!("arqix:user-stories/{}", id.to_lowercase()),
            StoryRecord {
                path: format!("{id}.md"),
                id: id.to_string(),
                personas: vec![format!("arqix:personas/{persona}")],
                workflows: vec![format!("arqix:workflows/{workflow}")],
            },
        )
    }

    fn workflow(id: &str, personas: &[&str]) -> (String, BTreeSet<String>) {
        (
            format!("arqix:workflows/{id}"),
            personas
                .iter()
                .map(|p| format!("arqix:personas/{p}"))
                .collect(),
        )
    }

    // arqix:verifies REQ-01-01-11-08
    #[test]
    fn story_in_a_workflow_its_id_does_not_encode_is_reported() {
        let stories = BTreeMap::from([story("US-01-01-21", "per-01", "wf-08-01")]);
        let workflows = BTreeMap::from([workflow("wf-01-01", &["per-01"])]);
        let mut findings = Vec::new();
        story_workflow_checks(&stories, &workflows, &BTreeSet::new(), &mut findings);
        assert_eq!(rules_of(&findings), vec!["US-WF-001"]);
        assert_eq!(
            findings[0].message,
            "id US-01-01-21 encodes workflow 'arqix:workflows/wf-01-01', \
             is-part-of-workflow names ['arqix:workflows/wf-08-01']"
        );
    }

    // arqix:verifies REQ-01-01-11-08
    #[test]
    fn story_without_a_workflow_is_reported() {
        let (iri, mut record) = story("US-01-01-21", "per-01", "wf-01-01");
        record.workflows.clear();
        let stories = BTreeMap::from([(iri, record)]);
        let mut findings = Vec::new();
        story_workflow_checks(&stories, &BTreeMap::new(), &BTreeSet::new(), &mut findings);
        assert_eq!(rules_of(&findings), vec!["US-WF-001"]);
        assert_eq!(
            findings[0].message,
            "id US-01-01-21 encodes workflow 'arqix:workflows/wf-01-01', \
             is-part-of-workflow names none"
        );
    }

    // arqix:verifies REQ-01-01-11-09
    #[test]
    fn persona_missing_from_the_workflow_is_reported() {
        let stories = BTreeMap::from([story("US-01-01-21", "per-08", "wf-01-01")]);
        let workflows = BTreeMap::from([workflow("wf-01-01", &["per-01"])]);
        let mut findings = Vec::new();
        story_workflow_checks(&stories, &workflows, &BTreeSet::new(), &mut findings);
        assert_eq!(rules_of(&findings), vec!["US-PER-001"]);
        assert_eq!(
            findings[0].message,
            "persona 'arqix:personas/per-08' is not declared on workflow \
             'arqix:workflows/wf-01-01' (has-primary-persona/has-relevant-persona)"
        );
    }

    // arqix:verifies REQ-01-01-11-09
    #[test]
    fn consolidation_persona_attaches_to_any_workflow() {
        let stories = BTreeMap::from([story("US-04-01-17", "per-10", "wf-04-01")]);
        let workflows = BTreeMap::from([workflow("wf-04-01", &["per-09"])]);
        let consolidation = BTreeSet::from(["arqix:personas/per-10".to_string()]);
        let mut findings = Vec::new();
        story_workflow_checks(&stories, &workflows, &consolidation, &mut findings);
        assert!(findings.is_empty(), "unexpected: {:?}", rules_of(&findings));
    }

    // arqix:verifies REQ-01-01-11-08
    // arqix:verifies REQ-01-01-11-09
    #[test]
    fn coupled_story_is_clean_and_relevant_personas_count() {
        // per-08 is only a relevant (not primary) persona of wf-09-01.
        let stories = BTreeMap::from([story("US-09-01-03", "per-08", "wf-09-01")]);
        let workflows = BTreeMap::from([workflow("wf-09-01", &["per-01", "per-08"])]);
        let mut findings = Vec::new();
        story_workflow_checks(&stories, &workflows, &BTreeSet::new(), &mut findings);
        assert!(findings.is_empty(), "unexpected: {:?}", rules_of(&findings));
    }

    // arqix:verifies REQ-01-01-11-09
    #[test]
    fn unresolvable_workflow_reference_skips_the_persona_check() {
        // Graph resolution is a linter concern; the coupling check only reads
        // workflows that exist.
        let stories = BTreeMap::from([story("US-01-01-21", "per-08", "wf-01-01")]);
        let mut findings = Vec::new();
        story_workflow_checks(&stories, &BTreeMap::new(), &BTreeSet::new(), &mut findings);
        assert!(findings.is_empty(), "unexpected: {:?}", rules_of(&findings));
    }
}
