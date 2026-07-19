//! Frontmatter checker (`arqix lint frontmatter`): the Rust port of the
//! reference oracle `scripts/check_frontmatter.py` (retired 2026-07-15 after
//! conformance; see git history). It validates the
//! architecture documents (stories, requirements, personas, workflows, ADRs,
//! arc42/ICD units and pages) and the ontology documents (classes, properties,
//! individuals, index) for frontmatter consistency, canonical formatting, and
//! correct use of the ontology vocabulary. It complements the sibling
//! requirements checker (`arqix lint requirements`), which covers the US/REQ
//! link semantics and the EARS sentence rules.
//!
//! The port is behaviour-faithful to the oracle: same rule IDs (FMT-*, FM-*,
//! ONT-*), same message strings, the same `(path, rule, message)` finding
//! order, and the same exit codes (0 no findings, 1 findings — errors OR
//! warnings, 2 usage/I-O error). The retired script's selftest fixtures are
//! mirrored in this module's tests, which own the specification (arc42
//! chapter 8, oracle policy). It has its own frontmatter reader (the order-preserving `Doc`
//! parser, not the shared store parser), so this module reproduces that reader
//! exactly rather than reusing the store's.

use crate::OutputFormat;
use crate::checkers::shared::{
    Finding, basename, collect_md, py_list_repr, py_repr, read_universal, repr_opt,
};
use crate::date::is_calendar_date;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::process::ExitCode;
use std::sync::OnceLock;

// --- controlled vocabularies (faithful to the oracle's module tables) ---

const REQUIRED_META: [&str; 6] = [
    "lifecycle-status",
    "owner",
    "created",
    "updated",
    "lang",
    "generated",
];

const NON_ARQIX_TYPES: [&str; 2] = ["rdfs:Class", "rdf:Property"];

/// Controlled vocabulary for `properties.section-kind` (FM-007).
const SECTION_KINDS: [&str; 9] = [
    "arc42-chapter",
    "icd-command-surface",
    "icd-exit-codes",
    "icd-diagnostics",
    "icd-wire-schemas",
    "icd-input-grammars",
    "icd-forward-contracts",
    "icd-page",
    "manual-chapter",
];

// arqix:implements REQ-08-01-29-01
/// The effective section-kind vocabulary: the configured
/// `[frontmatter].section-kinds`, or the built-in list.
fn effective_section_kinds() -> &'static [String] {
    static VOCAB: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();
    VOCAB.get_or_init(|| {
        crate::config::frontmatter_vocab(Path::new("."))
            .section_kinds
            .unwrap_or_else(|| SECTION_KINDS.iter().map(|s| s.to_string()).collect())
    })
}

// arqix:implements REQ-08-01-29-02
/// The effective external-type vocabulary: the configured
/// `[frontmatter].allowed-external-types`, or the built-in list.
fn effective_external_types() -> &'static [String] {
    static VOCAB: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();
    VOCAB.get_or_init(|| {
        crate::config::frontmatter_vocab(Path::new("."))
            .allowed_external_types
            .unwrap_or_else(|| NON_ARQIX_TYPES.iter().map(|s| s.to_string()).collect())
    })
}

/// The canonical top-level key order and required keys shared by the
/// architecture families.
const ARCH_ORDER: [&str; 10] = [
    "id",
    "title",
    "slug",
    "iri",
    "rdf",
    "triples",
    "derived-triples",
    "properties",
    "external-references",
    "meta",
];
const ARCH_REQUIRED: [&str; 4] = ["id", "title", "slug", "iri"];

/// Architecture family -> (id prefix, iri namespace).
fn arch_ns(family: &str) -> Option<(&'static str, &'static str)> {
    Some(match family {
        "story" => ("US-", "arqix:user-stories/"),
        "req" => ("REQ-", "arqix:requirements/"),
        "persona" => ("PER-", "arqix:personas/"),
        "workflow" => ("WF-", "arqix:workflows/"),
        "adr" => ("ADR-", "arqix:adrs/"),
        "arc42-unit" => ("unit-arc42-", "arqix:units/"),
        "arc42-page" => ("page-", "arqix:pages/"),
        "icd-unit" => ("unit-icd-", "arqix:units/"),
        "icd-page" => ("page-icd", "arqix:pages/"),
        _ => return None,
    })
}

/// Ontology family -> id prefix.
fn ont_id_prefix(family: &str) -> Option<&'static str> {
    match family {
        "ont-class" => Some("class-"),
        "ont-property" => Some("property-"),
        "ont-individual" => Some("individual-"),
        _ => None,
    }
}

/// Ontology family -> iri namespace.
fn ont_ns(family: &str) -> Option<&'static str> {
    match family {
        "ont-class" => Some("arqix:classes/"),
        "ont-property" => Some("arqix:properties/"),
        "ont-individual" => Some("arqix:individuals/"),
        _ => None,
    }
}

/// Controlled lifecycle vocabulary per document nature (ADR-0010, FM-008).
/// Every family not listed follows the prose model.
fn lifecycle_vocab(family: &str) -> &'static [&'static str] {
    match family {
        "story" => &["draft", "specified", "in-implementation", "done", "retired"],
        "req" => &["active", "retired"],
        _ => &["draft", "final", "retired"],
    }
}

fn strs(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| s.to_string()).collect()
}

// --- compiled patterns (once) -------------------------------------------

struct Patterns {
    top_key: Regex,
    triple_pred: Regex,
    triple_obj: Regex,
    triple_item: Regex,
    iso_date: Regex,
    index_entry: Regex,
}

fn patterns() -> &'static Patterns {
    static P: OnceLock<Patterns> = OnceLock::new();
    P.get_or_init(|| Patterns {
        top_key: Regex::new(r"^([A-Za-z0-9_-]+):\s*(.*)$").unwrap(),
        triple_pred: Regex::new(r"^-\s*predicate:\s*(.*)$").unwrap(),
        triple_obj: Regex::new(r"^object:\s*(.*)$").unwrap(),
        triple_item: Regex::new(r"^-\s*(\S.*)$").unwrap(),
        iso_date: Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap(),
        index_entry: Regex::new(r"(?m)^- ([a-z0-9-]+)$").unwrap(),
    })
}

// --- configured family contract (US-01-01-19, ADR-0011/0012) ------------

struct FamilyDef {
    name: String,
    dir: String,
    key_order: Vec<String>,
    required: Vec<String>,
}

/// The family tables the checker reads, seeded with the built-in defaults and
/// merged with the `[kinds.<family>]` entries of `arqix.toml` — the one-source
/// rule (ADR-0011): the formatter and this checker read the same declared
/// contract.
struct Contract {
    families: Vec<FamilyDef>,
    family_meta: HashMap<String, Vec<String>>,
    family_patterns: HashMap<String, String>,
    /// Declared vocabularies for named `properties` fields per family
    /// (REQ-08-01-29-03): the domain-state axis next to the guarded
    /// lifecycle.
    family_vocab: HashMap<String, Vec<(String, Vec<String>)>>,
    /// Declared placement templates per family (REQ-08-01-30-04): the
    /// checker-side direction of the kind's dir-template.
    family_dir_templates: HashMap<String, String>,
}

impl Contract {
    fn defaults() -> Contract {
        let arch = |name: &str, dir: &str| FamilyDef {
            name: name.to_string(),
            dir: dir.to_string(),
            key_order: strs(&ARCH_ORDER),
            required: strs(&ARCH_REQUIRED),
        };
        let families = vec![
            arch("story", "docs/en/architecture/stories"),
            arch("req", "docs/en/architecture/req"),
            arch("persona", "docs/en/architecture/personas"),
            arch("workflow", "docs/en/architecture/workflows"),
            arch("adr", "docs/en/architecture/adr"),
            arch("arc42-unit", "docs/en/architecture/arc42/units"),
            arch("arc42-page", "docs/en/architecture/arc42"),
            arch("icd-unit", "docs/en/architecture/icd/units"),
            arch("icd-page", "docs/en/architecture/icd"),
            FamilyDef {
                name: "ont-class".to_string(),
                dir: "docs/ontology/classes".to_string(),
                key_order: strs(&[
                    "id",
                    "label",
                    "iri",
                    "rdf",
                    "rdfs",
                    "triples",
                    "derived-triples",
                    "properties",
                    "external-references",
                    "owl",
                    "meta",
                ]),
                required: strs(&["id", "label", "iri"]),
            },
            FamilyDef {
                name: "ont-property".to_string(),
                dir: "docs/ontology/properties".to_string(),
                key_order: strs(&[
                    "id",
                    "label",
                    "iri",
                    "rdf",
                    "rdfs",
                    "owl",
                    "triples",
                    "derived-triples",
                    "properties",
                    "external-references",
                    "meta",
                ]),
                required: strs(&["id", "label", "iri"]),
            },
            FamilyDef {
                name: "ont-individual".to_string(),
                dir: "docs/ontology/individuals".to_string(),
                key_order: strs(&[
                    "id",
                    "label",
                    "iri",
                    "rdf",
                    "triples",
                    "derived-triples",
                    "properties",
                    "external-references",
                    "meta",
                ]),
                required: strs(&["id", "label", "iri"]),
            },
        ];
        Contract {
            families,
            family_meta: HashMap::new(),
            family_patterns: HashMap::new(),
            family_vocab: HashMap::new(),
            family_dir_templates: HashMap::new(),
        }
    }

    fn family_opt(&self, name: &str) -> Option<&FamilyDef> {
        self.families.iter().find(|f| f.name == name)
    }

    /// The contract for a document's family. Every loaded document comes from a
    /// family directory, so the lookup always resolves.
    fn family(&self, name: &str) -> &FamilyDef {
        self.family_opt(name)
            .expect("family is loaded from the contract")
    }

    fn upsert(&mut self, name: String, dir: String, key_order: Vec<String>, required: Vec<String>) {
        if let Some(existing) = self.families.iter_mut().find(|f| f.name == name) {
            existing.dir = dir;
            existing.key_order = key_order;
            existing.required = required;
        } else {
            self.families.push(FamilyDef {
                name,
                dir,
                key_order,
                required,
            });
        }
    }
}

fn toml_string_array(value: Option<&toml::Value>) -> Option<Vec<String>> {
    let arr = value?.as_array()?;
    Some(
        arr.iter()
            .filter_map(|x| x.as_str().map(str::to_string))
            .collect(),
    )
}

/// Merge the `[kinds.<family>]` contract entries into the family tables. Entries
/// without a string `dir` cannot be matched to files and are skipped.
fn apply_config(contract: &mut Contract, config: &toml::Table) {
    let kinds = match config.get("kinds").and_then(|v| v.as_table()) {
        Some(t) => t,
        None => return,
    };
    for (family, entry) in kinds {
        let directory = match entry.get("dir").and_then(|v| v.as_str()) {
            Some(d) if !d.is_empty() => d,
            _ => continue,
        };
        let (def_order, def_required) = match contract.family_opt(family) {
            Some(f) => (f.key_order.clone(), f.required.clone()),
            None => (strs(&ARCH_ORDER), strs(&ARCH_REQUIRED)),
        };
        let dir = directory.trim_end_matches('/').to_string();
        let key_order = toml_string_array(entry.get("key-order")).unwrap_or(def_order);
        let required = toml_string_array(entry.get("required")).unwrap_or(def_required);
        contract.upsert(family.clone(), dir, key_order, required);
        if let Some(required_meta) = toml_string_array(entry.get("required-meta")) {
            contract.family_meta.insert(family.clone(), required_meta);
        }
        if let Some(pattern) = entry.get("id-pattern").and_then(|v| v.as_str()) {
            contract
                .family_patterns
                .insert(family.clone(), pattern.to_string());
        }
        // arqix:implements REQ-08-01-30-04
        if let Some(template) = entry.get("dir-template").and_then(|v| v.as_str()) {
            contract
                .family_dir_templates
                .insert(family.clone(), template.to_string());
        }
        // arqix:implements REQ-08-01-29-03
        if let Some(vocab) = entry.get("vocab").and_then(|v| v.as_table()) {
            let declared: Vec<(String, Vec<String>)> = vocab
                .iter()
                .filter_map(|(field, values)| {
                    Some((field.clone(), toml_string_array(Some(values))?))
                })
                .collect();
            contract.family_vocab.insert(family.clone(), declared);
        }
    }
}

/// The effective contract: defaults merged with `arqix.toml` `[kinds]`. A
/// malformed configuration falls back to the defaults (config validate owns the
/// malformed-file finding), exactly like the oracle's `load_config`.
fn load_contract() -> Contract {
    let mut contract = Contract::defaults();
    if Path::new("arqix.toml").is_file()
        && let Ok(text) = std::fs::read_to_string("arqix.toml")
        && let Ok(table) = text.parse::<toml::Table>()
    {
        apply_config(&mut contract, &table);
    }
    contract
}

/// Emulate Python's `re.match(pattern, s)`: a match anchored at the start of
/// the string (the pattern's own `$`, if any, still binds the end).
fn py_re_match(pattern: &str, s: &str) -> bool {
    match Regex::new(&format!("^(?:{pattern})")) {
        Ok(re) => re.is_match(s),
        Err(_) => false,
    }
}

// --- frontmatter reader (faithful to the oracle's `Doc`) ----------------

/// A parsed arqix document with enough raw detail for the format checks.
struct Doc {
    path: String,
    family: String,
    lines: Vec<String>,
    text: String,
    top_keys: Vec<String>,
    scalars: HashMap<String, String>,
    rdf_types: Vec<String>,
    triples: Vec<(String, Vec<String>)>,
    rdfs: HashMap<String, Vec<String>>,
    owl: HashMap<String, String>,
    meta: HashMap<String, String>,
    properties: HashMap<String, String>,
    body: String,
    fm_ok: bool,
    fm_lines: Vec<String>,
}

impl Doc {
    fn new(path: String, text: String, family: String) -> Doc {
        let lines: Vec<String> = text.split('\n').map(str::to_string).collect();
        let mut doc = Doc {
            path,
            family,
            lines,
            text,
            top_keys: Vec::new(),
            scalars: HashMap::new(),
            rdf_types: Vec::new(),
            triples: Vec::new(),
            rdfs: HashMap::new(),
            owl: HashMap::new(),
            meta: HashMap::new(),
            properties: HashMap::new(),
            body: String::new(),
            fm_ok: false,
            fm_lines: Vec::new(),
        };
        doc.parse();
        doc
    }

    fn parse(&mut self) {
        let p = patterns();
        if self.lines.first().map(|l| l.trim()) != Some("---") {
            return;
        }
        let mut end = 1;
        while end < self.lines.len() && self.lines[end].trim() != "---" {
            end += 1;
        }
        if end >= self.lines.len() {
            return;
        }
        self.fm_lines = self.lines[1..end].to_vec();
        self.body = self.lines[end + 1..].join("\n");
        self.fm_ok = true;

        let mut section: Option<String> = None;
        let mut subsection: Option<String> = None;
        let mut current_triple: Option<usize> = None;

        for raw in &self.fm_lines {
            if raw.trim().is_empty() || raw.trim_start().starts_with('#') {
                continue;
            }
            let indent = raw.len() - raw.trim_start_matches(' ').len();
            let line = raw.trim();

            if indent == 0 {
                current_triple = None;
                subsection = None;
                let caps = match p.top_key.captures(line) {
                    Some(c) => c,
                    None => continue,
                };
                let key = caps.get(1).unwrap().as_str().to_string();
                let mut value = caps.get(2).unwrap().as_str().trim().to_string();
                if value.len() >= 2 && value.starts_with('"') && value.ends_with('"') {
                    value = value[1..value.len() - 1].to_string();
                }
                section = Some(key.clone());
                self.top_keys.push(key.clone());
                if value != "{}" && value != "[]" && !value.is_empty() {
                    self.scalars.insert(key, value);
                }
            } else if section.as_deref() == Some("rdf") && line.starts_with("- ") {
                self.rdf_types.push(line[2..].trim().to_string());
            } else if section.as_deref() == Some("rdfs") {
                let matched = p.top_key.captures(line);
                if let Some(caps) = &matched
                    && indent == 2
                {
                    let sub = caps.get(1).unwrap().as_str().to_string();
                    let value = caps.get(2).unwrap().as_str().trim();
                    let entry = self.rdfs.entry(sub.clone()).or_default();
                    if !value.is_empty() {
                        entry.push(value.to_string());
                    }
                    subsection = Some(sub);
                } else if line.starts_with("- ")
                    && let Some(sub) = subsection.clone()
                {
                    self.rdfs
                        .entry(sub)
                        .or_default()
                        .push(line[2..].trim().to_string());
                }
            } else if section.as_deref() == Some("owl") {
                if let Some(caps) = p.top_key.captures(line) {
                    let value = caps.get(2).unwrap().as_str().trim();
                    if !value.is_empty() {
                        self.owl
                            .insert(caps.get(1).unwrap().as_str().to_string(), value.to_string());
                    }
                }
            } else if section.as_deref() == Some("triples")
                || section.as_deref() == Some("derived-triples")
            {
                if let Some(caps) = p.triple_pred.captures(line) {
                    let predicate = caps.get(1).unwrap().as_str().trim().to_string();
                    self.triples.push((predicate, Vec::new()));
                    current_triple = Some(self.triples.len() - 1);
                } else if let Some(caps) = p.triple_obj.captures(line) {
                    if let Some(ti) = current_triple {
                        let value = caps.get(1).unwrap().as_str().trim();
                        if !value.is_empty() {
                            self.triples[ti].1.push(value.to_string());
                        }
                    }
                } else if let Some(caps) = p.triple_item.captures(line)
                    && let Some(ti) = current_triple
                {
                    self.triples[ti]
                        .1
                        .push(caps.get(1).unwrap().as_str().trim().to_string());
                }
            } else if section.as_deref() == Some("meta") {
                if let Some(caps) = p.top_key.captures(line) {
                    let value = caps.get(2).unwrap().as_str().trim();
                    if !value.is_empty() {
                        self.meta
                            .insert(caps.get(1).unwrap().as_str().to_string(), value.to_string());
                    }
                }
            } else if section.as_deref() == Some("properties")
                && let Some(caps) = p.top_key.captures(line)
            {
                let value = caps.get(2).unwrap().as_str().trim();
                if !value.is_empty() {
                    self.properties
                        .insert(caps.get(1).unwrap().as_str().to_string(), value.to_string());
                }
            }
        }
    }

    fn first_heading(&self) -> Option<String> {
        for line in self.body.split('\n') {
            if let Some(rest) = line.strip_prefix("## ") {
                return Some(rest.trim().to_string());
            }
            if let Some(rest) = line.strip_prefix("# ") {
                return Some(rest.trim().to_string());
            }
        }
        None
    }
}

/// True when `observed` is a subsequence of `canonical`.
fn is_subsequence(observed: &[String], canonical: &[String]) -> bool {
    let mut it = canonical.iter();
    for key in observed {
        let mut found = false;
        for candidate in it.by_ref() {
            if candidate == key {
                found = true;
                break;
            }
        }
        if !found {
            return false;
        }
    }
    true
}

// --- checks -------------------------------------------------------------

fn check_format(doc: &Doc, contract: &Contract, findings: &mut Vec<Finding>) {
    let path = &doc.path;
    if !doc.fm_ok {
        findings.push(Finding::error(
            path,
            "FMT-001",
            "frontmatter must open with '---' on line 1 and be closed".to_string(),
        ));
        return;
    }
    if doc.fm_lines.first().is_some_and(|l| l.trim().is_empty()) {
        findings.push(Finding::error(
            path,
            "FMT-002",
            "blank line directly after the opening '---'".to_string(),
        ));
    }
    let order = &contract.family(&doc.family).key_order;
    let known: Vec<String> = doc
        .top_keys
        .iter()
        .filter(|k| order.iter().any(|o| o == *k))
        .cloned()
        .collect();
    let unknown: Vec<String> = doc
        .top_keys
        .iter()
        .filter(|k| !order.iter().any(|o| o == *k))
        .cloned()
        .collect();
    for key in &unknown {
        findings.push(Finding::error(
            path,
            "FMT-003",
            format!(
                "unknown top-level key {} for family {}",
                py_repr(key),
                doc.family
            ),
        ));
    }
    if !is_subsequence(&known, order) {
        findings.push(Finding::error(
            path,
            "FMT-003",
            format!(
                "top-level keys out of canonical order: {} (expected order {})",
                py_list_repr(&known),
                py_list_repr(order)
            ),
        ));
    }
    for (i, raw) in doc.lines.iter().enumerate() {
        if raw.as_str() != raw.trim_end() {
            findings.push(Finding::error(
                path,
                "FMT-004",
                format!("trailing whitespace on line {}", i + 1),
            ));
            break;
        }
    }
    if !doc.text.ends_with('\n') || doc.text.ends_with("\n\n") {
        findings.push(Finding::error(
            path,
            "FMT-004",
            "file must end with exactly one newline".to_string(),
        ));
    }
    let created = doc.meta.get("created").map(String::as_str);
    let updated = doc.meta.get("updated").map(String::as_str);
    let iso = &patterns().iso_date;
    for (key, value) in [("created", created), ("updated", updated)] {
        if let Some(v) = value {
            if !iso.is_match(v) {
                findings.push(Finding::error(
                    path,
                    "FMT-005",
                    format!("meta.{} {} is not ISO YYYY-MM-DD", key, py_repr(v)),
                ));
            } else if !is_calendar_date(v) {
                findings.push(Finding::error(
                    path,
                    "FMT-005",
                    format!("meta.{} {} is not a real calendar date", key, py_repr(v)),
                ));
            }
        }
    }
    if let (Some(created), Some(updated)) = (created, updated)
        && iso.is_match(created)
        && iso.is_match(updated)
        && created > updated
    {
        findings.push(Finding::error(
            path,
            "FMT-005",
            format!("meta.created {created} is after meta.updated {updated}"),
        ));
    }
    if let Some(lang) = doc.meta.get("lang")
        && lang != "en"
    {
        findings.push(Finding::error(
            path,
            "FMT-006",
            format!("meta.lang {}, expected 'en'", py_repr(lang)),
        ));
    }
}

fn check_frontmatter(doc: &Doc, contract: &Contract, findings: &mut Vec<Finding>) {
    let path = &doc.path;
    let fam = contract.family(&doc.family);
    for key in &fam.required {
        if !doc.scalars.contains_key(key) {
            findings.push(Finding::error(
                path,
                "FM-001",
                format!("required key {} missing or empty", py_repr(key)),
            ));
        }
    }
    if doc.rdf_types.is_empty() {
        findings.push(Finding::error(
            path,
            "FM-001",
            "rdf.type missing or empty".to_string(),
        ));
    }
    let default_meta = strs(&REQUIRED_META);
    let meta_keys = contract
        .family_meta
        .get(&doc.family)
        .unwrap_or(&default_meta);
    for key in meta_keys {
        if !doc.meta.contains_key(key) {
            findings.push(Finding::error(
                path,
                "FM-001",
                format!("meta.{key} missing or empty"),
            ));
        }
    }

    let filename = basename(path);
    let doc_id = doc.scalars.get("id").cloned().unwrap_or_default();
    let iri = doc.scalars.get("iri").cloned().unwrap_or_default();

    let expected_heading: String;
    if let Some((prefix, ns)) = arch_ns(&doc.family) {
        if !doc_id.is_empty() {
            let id_dash = format!("{doc_id}-");
            if !doc_id.starts_with(prefix) || !filename.starts_with(&id_dash) {
                findings.push(Finding::error(
                    path,
                    "FM-002",
                    format!(
                        "id {} is not a {}* prefix of the filename",
                        py_repr(&doc_id),
                        prefix
                    ),
                ));
            }
            let expected_iri = format!("{}{}", ns, doc_id.to_lowercase());
            if !iri.is_empty() && iri != expected_iri {
                findings.push(Finding::error(
                    path,
                    "FM-003",
                    format!("iri {}, expected {}", py_repr(&iri), py_repr(&expected_iri)),
                ));
            }
            let slug = doc.scalars.get("slug").cloned().unwrap_or_default();
            if !slug.is_empty() && filename.starts_with(&id_dash) {
                let start = doc_id.len() + 1;
                let stop = filename.len().saturating_sub(3);
                let tail = if start <= stop {
                    &filename[start..stop]
                } else {
                    ""
                };
                if slug != tail {
                    findings.push(Finding::error(
                        path,
                        "FM-004",
                        format!(
                            "slug {} does not match filename tail {}",
                            py_repr(&slug),
                            py_repr(tail)
                        ),
                    ));
                }
            }
        }
        expected_heading = if doc.family == "req" {
            "Requirement".to_string()
        } else {
            doc.scalars.get("title").cloned().unwrap_or_default()
        };
    } else if ont_id_prefix(&doc.family).is_none() {
        // A configured family carries no built-in id/iri shape — its shape is
        // the configured id-pattern (US-01-01-18, ADR-0012).
        if let Some(pattern) = contract.family_patterns.get(&doc.family)
            && !doc_id.is_empty()
            && !py_re_match(pattern, &doc_id)
        {
            findings.push(Finding::error(
                path,
                "FM-002",
                format!(
                    "id {} does not match the configured id-pattern {}",
                    py_repr(&doc_id),
                    py_repr(pattern)
                ),
            ));
        }
        expected_heading = doc.scalars.get("title").cloned().unwrap_or_default();
    } else {
        let label = doc.scalars.get("label").cloned().unwrap_or_default();
        let id_prefix = ont_id_prefix(&doc.family).unwrap();
        if !label.is_empty() {
            let expected_name = format!("{label}.md");
            if filename != expected_name {
                findings.push(Finding::error(
                    path,
                    "FM-002",
                    format!(
                        "filename {}, expected {}",
                        py_repr(&filename),
                        py_repr(&expected_name)
                    ),
                ));
            }
            let expected_id = format!("{id_prefix}{label}");
            if doc_id != expected_id {
                findings.push(Finding::error(
                    path,
                    "FM-002",
                    format!(
                        "id {}, expected {}",
                        py_repr(&doc_id),
                        py_repr(&expected_id)
                    ),
                ));
            }
            let expected_iri = format!("{}{}", ont_ns(&doc.family).unwrap(), label);
            if !iri.is_empty() && iri != expected_iri {
                findings.push(Finding::error(
                    path,
                    "FM-003",
                    format!("iri {}, expected {}", py_repr(&iri), py_repr(&expected_iri)),
                ));
            }
        }
        expected_heading = label;
    }

    let heading = doc.first_heading();
    if !expected_heading.is_empty() {
        let mismatch = match &heading {
            None => true,
            Some(h) => h.to_lowercase() != expected_heading.to_lowercase(),
        };
        if mismatch {
            findings.push(Finding::error(
                path,
                "FM-005",
                format!(
                    "first heading {}, expected {}",
                    repr_opt(heading.as_deref()),
                    py_repr(&expected_heading)
                ),
            ));
        }
    }

    if let Some(section_kind) = doc.properties.get("section-kind")
        && !effective_section_kinds().iter().any(|k| k == section_kind)
    {
        findings.push(Finding::error(
            path,
            "FM-007",
            format!(
                "properties.section-kind {} is not an allowed value",
                py_repr(section_kind)
            ),
        ));
    }

    // arqix:implements REQ-08-01-30-04
    // The placement contract: the parent directory must equal the kind's
    // dir-template rendered from the document's own properties.
    if let Some(template) = contract.family_dir_templates.get(&doc.family) {
        let mut rendered = template.clone();
        for (key, value) in &doc.properties {
            rendered = rendered.replace(&format!("{{{key}}}"), value);
        }
        if let Some(slug) = doc.scalars.get("slug") {
            rendered = rendered.replace("{slug}", slug);
        }
        rendered = rendered.replace("{kind}", &doc.family);
        if let Some(open) = rendered.find('{') {
            let placeholder: String = rendered[open + 1..]
                .chars()
                .take_while(|c| *c != '}')
                .collect();
            findings.push(Finding::error(
                path,
                "FM-010",
                format!(
                    "dir-template placeholder '{{{placeholder}}}' has no value in this document"
                ),
            ));
        } else {
            let parent = Path::new(path)
                .parent()
                .map(|p| p.to_string_lossy().replace('\\', "/"))
                .unwrap_or_default();
            if parent != rendered {
                findings.push(Finding::error(
                    path,
                    "FM-010",
                    format!(
                        "document lies in {}, dir-template renders {}",
                        py_repr(&parent),
                        py_repr(&rendered)
                    ),
                ));
            }
        }
    }

    // arqix:implements REQ-08-01-29-03
    if let Some(declared) = contract.family_vocab.get(&doc.family) {
        for (field, allowed) in declared {
            if let Some(value) = doc.properties.get(field)
                && !allowed.contains(value)
            {
                findings.push(Finding::error(
                    path,
                    "FM-009",
                    format!(
                        "properties.{field} {} is not in the vocabulary {}",
                        py_repr(value),
                        py_list_repr(allowed)
                    ),
                ));
            }
        }
    }

    if let Some(lifecycle) = doc.meta.get("lifecycle-status") {
        let vocab = lifecycle_vocab(&doc.family);
        if !vocab.contains(&lifecycle.as_str()) {
            let mut sorted: Vec<String> = vocab.iter().map(|s| s.to_string()).collect();
            sorted.sort();
            findings.push(Finding::error(
                path,
                "FM-008",
                format!(
                    "meta.lifecycle-status {} is not in the vocabulary {}",
                    py_repr(lifecycle),
                    py_list_repr(&sorted)
                ),
            ));
        }
    }
}

/// The provenance fields a finalised source record must carry; the local
/// copy and its digest are optional but travel as a pair.
const SOURCE_REQUIRED: [&str; 2] = ["uri", "accessed"];

/// The provenance contract of `arqix:classes/source` (the SRC rule family),
/// keyed on the document's rdf.type rather than a family directory: any
/// repository that types a document as a source gets the checks.
/// Completeness (SRC-002) applies once the record leaves draft; a present
/// but malformed value (SRC-003..005) is a finding in every lifecycle state.
// arqix:implements REQ-08-01-28-01
// arqix:implements REQ-08-01-28-02
// arqix:implements REQ-08-01-28-03
fn check_source(doc: &Doc, roots: &[String], findings: &mut Vec<Finding>) {
    let path = &doc.path;

    let doc_id = doc.scalars.get("id").cloned().unwrap_or_default();
    let iri = doc.scalars.get("iri").cloned().unwrap_or_default();
    if !doc_id.is_empty() {
        let expected_iri = format!("arqix:sources/{}", doc_id.to_lowercase());
        if !iri.is_empty() && iri != expected_iri {
            findings.push(Finding::error(
                path,
                "SRC-001",
                format!("iri {}, expected {}", py_repr(&iri), py_repr(&expected_iri)),
            ));
        }
    }

    let draft = doc.meta.get("lifecycle-status").map(String::as_str) == Some("draft");
    if !draft {
        for key in SOURCE_REQUIRED {
            if !doc.properties.contains_key(key) {
                findings.push(Finding::error(
                    path,
                    "SRC-002",
                    format!("finalised source is missing properties.{key}"),
                ));
            }
        }
    }
    // A copy without its digest (or the reverse) pretends more than it
    // pins; a record without either is a source that holds no copy.
    if doc.properties.contains_key("local-copy") != doc.properties.contains_key("sha256") {
        findings.push(Finding::error(
            path,
            "SRC-002",
            "properties.local-copy and properties.sha256 must be given together".to_string(),
        ));
    }

    if let Some(accessed) = doc.properties.get("accessed")
        && !is_calendar_date(accessed)
    {
        findings.push(Finding::error(
            path,
            "SRC-003",
            format!(
                "properties.accessed {} is not a calendar date",
                py_repr(accessed)
            ),
        ));
    }

    if let Some(digest) = doc.properties.get("sha256")
        && !(digest.len() == 64
            && digest
                .bytes()
                .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase()))
    {
        findings.push(Finding::error(
            path,
            "SRC-004",
            format!(
                "properties.sha256 {} is not a 64-character lowercase hex digest",
                py_repr(digest)
            ),
        ));
    }

    if let Some(copy) = doc.properties.get("local-copy") {
        let p = Path::new(copy);
        let escapes = p.is_absolute()
            || p.components()
                .any(|c| matches!(c, std::path::Component::ParentDir));
        let in_corpus = roots
            .iter()
            .any(|root| copy == root || copy.starts_with(&format!("{root}/")));
        if escapes || in_corpus {
            findings.push(Finding::error(
                path,
                "SRC-005",
                format!(
                    "properties.local-copy {} escapes the repository or lies inside a documentation root",
                    py_repr(copy)
                ),
            ));
        }
    }

    // arqix:implements REQ-08-01-28-04
    // The digest pins what was read (SRC-006): with a clean path shape
    // (SRC-005) and a well-formed digest (SRC-004), the copy must exist and
    // its bytes must hash to the recorded value — one cause, one finding.
    if let (Some(copy), Some(digest)) = (
        doc.properties.get("local-copy"),
        doc.properties.get("sha256"),
    ) {
        let digest_ok = digest.len() == 64
            && digest
                .bytes()
                .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase());
        let p = Path::new(copy);
        let escapes = p.is_absolute()
            || p.components()
                .any(|c| matches!(c, std::path::Component::ParentDir));
        let in_corpus = roots
            .iter()
            .any(|root| copy == root || copy.starts_with(&format!("{root}/")));
        if digest_ok && !escapes && !in_corpus {
            match std::fs::read(p) {
                Ok(bytes) => {
                    use sha2::Digest;
                    let actual = sha2::Sha256::digest(&bytes)
                        .iter()
                        .map(|b| format!("{b:02x}"))
                        .collect::<String>();
                    if actual != *digest {
                        findings.push(Finding::error(
                            path,
                            "SRC-006",
                            format!(
                                "properties.local-copy {} hashes to {actual}, not the recorded sha256",
                                py_repr(copy)
                            ),
                        ));
                    }
                }
                Err(_) => {
                    findings.push(Finding::error(
                        path,
                        "SRC-006",
                        format!("properties.local-copy {} does not exist", py_repr(copy)),
                    ));
                }
            }
        }
    }
}

struct Vocab {
    classes: HashSet<String>,
    properties: HashSet<String>,
    all_iris: HashSet<String>,
}

fn check_vocabulary(
    doc: &Doc,
    vocab: &Vocab,
    findings: &mut Vec<Finding>,
    allow_undefined_inverse: bool,
) {
    let path = &doc.path;
    for (predicate, objects) in &doc.triples {
        if predicate.starts_with("arqix:") && !vocab.properties.contains(predicate) {
            findings.push(Finding::error(
                path,
                "ONT-001",
                format!("predicate {predicate} is not a defined ontology property"),
            ));
        }
        for obj in objects {
            if obj.starts_with("arqix:") && !vocab.all_iris.contains(obj) {
                findings.push(Finding::error(
                    path,
                    "ONT-003",
                    format!("triple object {obj} does not resolve to a scanned document"),
                ));
            }
        }
    }
    for rdf_type in &doc.rdf_types {
        if rdf_type.starts_with("arqix:") {
            if !vocab.classes.contains(rdf_type) {
                findings.push(Finding::error(
                    path,
                    "ONT-002",
                    format!("rdf.type {rdf_type} is not a defined ontology class"),
                ));
            }
        } else if !effective_external_types().iter().any(|t| t == rdf_type) {
            findings.push(Finding::error(
                path,
                "ONT-002",
                format!(
                    "rdf.type {rdf_type} is neither an arqix class nor an allowed external type"
                ),
            ));
        }
    }
    for subkey in ["sub-class-of", "domain", "range"] {
        if let Some(targets) = doc.rdfs.get(subkey) {
            for target in targets {
                if target.starts_with("arqix:") && !vocab.classes.contains(target) {
                    findings.push(Finding::error(
                        path,
                        "ONT-004",
                        format!("rdfs.{subkey} target {target} is not a defined class"),
                    ));
                }
            }
        }
    }
    if let Some(inverse) = doc.owl.get("inverse-of")
        && inverse.starts_with("arqix:")
        && !vocab.properties.contains(inverse)
        && !allow_undefined_inverse
    {
        findings.push(Finding::warning(
            path,
            "ONT-005",
            format!("owl.inverse-of {inverse} has no property document"),
        ));
    }
}

fn check_index(classes_by_label: &HashSet<String>, findings: &mut Vec<Finding>) {
    let path = "docs/ontology/index.md";
    if !Path::new(path).is_file() {
        return;
    }
    let text = read_universal(path);
    let body = text.splitn(3, "---\n").last().unwrap_or("");
    for caps in patterns().index_entry.captures_iter(body) {
        let name = caps.get(1).unwrap().as_str();
        if !classes_by_label.contains(name) {
            findings.push(Finding::error(
                path,
                "ONT-006",
                format!("index lists class {} which is not defined", py_repr(name)),
            ));
        }
    }
}

// --- corpus walk --------------------------------------------------------

/// Load every family document recursively; nested family directories (arc42/
/// units under arc42, icd/units under icd) win by longest path so each file
/// keeps its most specific family. `index.md` is navigation prose, not a
/// document family member.
fn load_docs(contract: &Contract) -> Vec<Doc> {
    let mut by_depth: Vec<&FamilyDef> = contract.families.iter().collect();
    by_depth.sort_by_key(|f| std::cmp::Reverse(f.dir.len()));
    let mut seen: HashSet<String> = HashSet::new();
    let mut docs: Vec<Doc> = Vec::new();
    for fam in by_depth {
        if !Path::new(&fam.dir).is_dir() {
            continue;
        }
        let mut files = Vec::new();
        collect_md(Path::new(&fam.dir), &mut files);
        files.sort();
        for path in files {
            if seen.contains(&path) {
                continue;
            }
            seen.insert(path.clone());
            if basename(&path) == "index.md" {
                continue;
            }
            let text = read_universal(&path);
            docs.push(Doc::new(path, text, fam.name.clone()));
        }
    }
    docs.sort_by(|a, b| a.path.cmp(&b.path));
    docs
}

/// The semantic identity of an ontology definition, as ONT-009 compares it:
/// declared types, subclass parents, domain, and range, order-insensitive.
type OntSemantics = (Vec<String>, Vec<String>, Vec<String>, Vec<String>);

fn ont_semantics(doc: &Doc) -> OntSemantics {
    let mut types = doc.rdf_types.clone();
    types.sort();
    let sorted = |key: &str| {
        let mut values = doc.rdfs.get(key).cloned().unwrap_or_default();
        values.sort();
        values
    };
    (
        types,
        sorted("sub-class-of"),
        sorted("domain"),
        sorted("range"),
    )
}

/// The reflexive-transitive sub-class-of closure of one class.
fn class_closure(start: &str, subclass: &HashMap<String, Vec<String>>) -> HashSet<String> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut stack = vec![start.to_string()];
    while let Some(class) = stack.pop() {
        if !seen.insert(class.clone()) {
            continue;
        }
        if let Some(parents) = subclass.get(&class) {
            stack.extend(parents.iter().cloned());
        }
    }
    seen
}

/// Whether a class reaches itself through sub-class-of edges other than its
/// own root self-reference (REQ-08-01-30-03).
fn in_subclass_cycle(start: &str, subclass: &HashMap<String, Vec<String>>) -> bool {
    let mut seen: HashSet<String> = HashSet::new();
    let mut stack: Vec<String> = subclass
        .get(start)
        .map(|parents| parents.iter().filter(|p| *p != start).cloned().collect())
        .unwrap_or_default();
    while let Some(class) = stack.pop() {
        if class == start {
            return true;
        }
        if !seen.insert(class.clone()) {
            continue;
        }
        if let Some(parents) = subclass.get(&class) {
            stack.extend(parents.iter().filter(|p| *p != &class).cloned());
        }
    }
    false
}

// arqix:implements REQ-08-01-30-02
/// ONT-007: a triple whose predicate declares a domain or range must keep
/// its subject and every resolvable object inside the declared classes,
/// subclass closure included; declaring opts the property in.
fn check_graph_contract(
    doc: &Doc,
    subclass: &HashMap<String, Vec<String>>,
    domains: &HashMap<String, Vec<String>>,
    ranges: &HashMap<String, Vec<String>>,
    types_by_iri: &HashMap<String, Vec<String>>,
    findings: &mut Vec<Finding>,
) {
    let satisfies = |types: &[String], declared: &[String]| {
        types.iter().any(|t| {
            class_closure(t, subclass)
                .iter()
                .any(|c| declared.contains(c))
        })
    };
    for (predicate, objects) in &doc.triples {
        if let Some(domain) = domains.get(predicate)
            && !doc.rdf_types.is_empty()
            && !satisfies(&doc.rdf_types, domain)
        {
            findings.push(Finding::error(
                &doc.path,
                "ONT-007",
                format!(
                    "triple predicate {predicate}: subject types {} lie outside the declared domain {}",
                    py_list_repr(&doc.rdf_types),
                    py_list_repr(domain)
                ),
            ));
        }
        if let Some(range) = ranges.get(predicate) {
            for object in objects {
                if let Some(object_types) = types_by_iri.get(object)
                    && !object_types.is_empty()
                    && !satisfies(object_types, range)
                {
                    findings.push(Finding::error(
                        &doc.path,
                        "ONT-007",
                        format!(
                            "triple {predicate} object {object}: types {} lie outside the declared range {}",
                            py_list_repr(object_types),
                            py_list_repr(range)
                        ),
                    ));
                }
            }
        }
    }
}

/// The built-in confidence vocabulary for claim markers.
const CLAIM_CONFIDENCE: [&str; 3] = ["high", "inferred", "estimated"];

/// The built-in review vocabulary of the provenance layers (ADR-0019).
const CLAIM_REVIEW: [&str; 3] = ["unreviewed", "confirmed", "rejected"];

/// The claim marker's attribute vocabulary: the anchor attributes plus the
/// shared provenance keys every carrier understands (ADR-0019).
const CLAIM_KEYS: [&str; 11] = [
    "supported-by",
    "confidence",
    "anchor",
    "record",
    "agent",
    "activity",
    "reviewed-by",
    "reviewed",
    "review-status",
    "representation",
    "representation-sha256",
];

// arqix:implements REQ-08-01-40-06
/// The effective review vocabulary: the configured
/// `[frontmatter].claim-review-status`, or the built-in default.
fn effective_claim_review() -> &'static [String] {
    static VOCAB: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();
    VOCAB.get_or_init(|| {
        crate::config::frontmatter_vocab(Path::new("."))
            .claim_review
            .unwrap_or_else(|| CLAIM_REVIEW.iter().map(|s| s.to_string()).collect())
    })
}

// arqix:implements REQ-08-01-40-01
/// The effective claim-confidence vocabulary: the configured
/// `[frontmatter].claim-confidence`, or the built-in default.
fn effective_claim_confidence() -> &'static [String] {
    static VOCAB: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();
    VOCAB.get_or_init(|| {
        crate::config::frontmatter_vocab(Path::new("."))
            .claim_confidence
            .unwrap_or_else(|| CLAIM_CONFIDENCE.iter().map(|s| s.to_string()).collect())
    })
}

// arqix:implements REQ-08-01-40-01
/// CLM-001/CLM-002: every claim marker names a supported-by target, carries
/// only known keys, and keeps its confidence inside the effective
/// vocabulary. The marker grammar is validated here; the edge itself is the
/// formatter's derived triple (ADR-0018).
fn check_claim_markers(doc: &Doc, claim_ids: &HashSet<String>, findings: &mut Vec<Finding>) {
    let pair_re = Regex::new(r#"^([a-z-]+)=("[^"]*"|[^\s"]+)\s*"#).expect("static regex");
    for line in doc.text.lines() {
        let trimmed = line.trim_start();
        let Some(rest) = trimmed.strip_prefix("<!-- arqix:claim") else {
            continue;
        };
        let Some(body) = rest.strip_suffix("-->") else {
            findings.push(Finding::error(
                &doc.path,
                "CLM-001",
                format!("claim marker is not closed: {}", py_repr(trimmed)),
            ));
            continue;
        };
        let mut cursor = body.trim_start();
        let mut keys: Vec<(String, String)> = Vec::new();
        let mut malformed = false;
        while !cursor.trim().is_empty() {
            match pair_re.captures(cursor) {
                Some(caps) => {
                    let value = caps[2].trim_matches('"').to_string();
                    keys.push((caps[1].to_string(), value));
                    cursor = &cursor[caps[0].len()..];
                }
                None => {
                    malformed = true;
                    break;
                }
            }
        }
        if malformed {
            findings.push(Finding::error(
                &doc.path,
                "CLM-001",
                format!("claim marker is malformed near {}", py_repr(cursor.trim())),
            ));
            continue;
        }
        for (key, _) in &keys {
            if !CLAIM_KEYS.contains(&key.as_str()) {
                findings.push(Finding::error(
                    &doc.path,
                    "CLM-001",
                    format!("claim marker carries unknown key {}", py_repr(key)),
                ));
            }
        }
        if !keys.iter().any(|(k, _)| k == "supported-by") {
            findings.push(Finding::error(
                &doc.path,
                "CLM-001",
                "claim marker misses supported-by".to_string(),
            ));
        }
        if let Some((_, confidence)) = keys.iter().find(|(k, _)| k == "confidence")
            && !effective_claim_confidence().iter().any(|c| c == confidence)
        {
            findings.push(Finding::error(
                &doc.path,
                "CLM-002",
                format!(
                    "claim confidence {} is not in the vocabulary {}",
                    py_repr(confidence),
                    py_list_repr(effective_claim_confidence())
                ),
            ));
        }
        // arqix:implements REQ-08-01-40-06
        if let Some((_, verdict)) = keys.iter().find(|(k, _)| k == "review-status")
            && !effective_claim_review().iter().any(|v| v == verdict)
        {
            findings.push(Finding::error(
                &doc.path,
                "CLM-003",
                format!(
                    "claim review-status {} is not in the vocabulary {}",
                    py_repr(verdict),
                    py_list_repr(effective_claim_review())
                ),
            ));
        }
        // arqix:implements REQ-08-01-40-07
        if let Some((_, record)) = keys.iter().find(|(k, _)| k == "record")
            && !claim_ids.contains(record)
        {
            findings.push(Finding::error(
                &doc.path,
                "CLM-004",
                format!(
                    "claim record {} does not resolve to a claim document",
                    py_repr(record)
                ),
            ));
        }
    }
}

fn run_checks(contract: &Contract, allow_undefined_inverse: bool) -> Vec<Finding> {
    let mut findings = Vec::new();
    let docs = load_docs(contract);

    // arqix:implements REQ-08-01-31-02
    // The module layers (ADR-0021): the embedded ontology documents of the
    // reserved core and the effective modules join the vocabulary tables
    // below. A corpus definition of the same IRI overrides the embedded one
    // (rising precedence), and embedded documents are definitions only —
    // they never join the per-document checks.
    let corpus_ont_iris: HashSet<String> = docs
        .iter()
        .filter(|d| d.family.starts_with("ont-"))
        .filter_map(|d| d.scalars.get("iri").cloned())
        .collect();
    let embedded: Vec<Doc> =
        crate::modules::embedded_ontology_docs(&crate::config::process_modules(Path::new(".")))
            .into_iter()
            .map(|(path, text)| {
                let family = if path.contains("/classes/") {
                    "ont-class"
                } else if path.contains("/properties/") {
                    "ont-property"
                } else {
                    "ont-individual"
                };
                Doc::new(path.to_string(), text.to_string(), family.to_string())
            })
            .collect();
    // The shipped core semantics, captured before the override filter —
    // ONT-009 compares against them (REQ-08-01-31-03).
    let core_iris = crate::modules::core_iris();
    let core_semantics: HashMap<String, OntSemantics> = embedded
        .iter()
        .filter_map(|d| d.scalars.get("iri").cloned().map(|iri| (iri, d)))
        .filter(|(iri, _)| core_iris.contains(iri))
        .map(|(iri, d)| (iri, ont_semantics(d)))
        .collect();
    let layered: Vec<&Doc> = embedded
        .iter()
        .filter(|d| {
            d.scalars
                .get("iri")
                .is_none_or(|iri| !corpus_ont_iris.contains(iri))
        })
        .collect();

    let mut classes: HashSet<String> = HashSet::new();
    let mut properties: HashSet<String> = HashSet::new();
    let mut all_iris: HashSet<String> = HashSet::new();
    let mut classes_by_label: HashSet<String> = HashSet::new();
    for doc in docs.iter().chain(layered.iter().copied()) {
        if doc.family == "ont-class" {
            if let Some(iri) = doc.scalars.get("iri") {
                classes.insert(iri.clone());
            }
            if let Some(label) = doc.scalars.get("label") {
                classes_by_label.insert(label.clone());
            }
        }
        if doc.family == "ont-property"
            && let Some(iri) = doc.scalars.get("iri")
        {
            properties.insert(iri.clone());
        }
        if let Some(iri) = doc.scalars.get("iri") {
            all_iris.insert(iri.clone());
        }
    }
    let vocab = Vocab {
        classes,
        properties,
        all_iris,
    };

    // arqix:implements REQ-08-01-30-02
    // arqix:implements REQ-08-01-30-03
    // The graph contract (ONT-007/ONT-008): declared domains and ranges,
    // and the sub-class-of hierarchy, validated over every document.
    let mut subclass: HashMap<String, Vec<String>> = HashMap::new();
    let mut domains: HashMap<String, Vec<String>> = HashMap::new();
    let mut ranges: HashMap<String, Vec<String>> = HashMap::new();
    let mut types_by_iri: HashMap<String, Vec<String>> = HashMap::new();
    for doc in docs.iter().chain(layered.iter().copied()) {
        if let Some(iri) = doc.scalars.get("iri") {
            types_by_iri.insert(iri.clone(), doc.rdf_types.clone());
            if doc.family == "ont-class"
                && let Some(parents) = doc.rdfs.get("sub-class-of")
            {
                subclass.insert(iri.clone(), parents.clone());
            }
            if doc.family == "ont-property" {
                if let Some(domain) = doc.rdfs.get("domain") {
                    domains.insert(iri.clone(), domain.clone());
                }
                if let Some(range) = doc.rdfs.get("range") {
                    ranges.insert(iri.clone(), range.clone());
                }
            }
        }
    }
    for doc in &docs {
        if doc.family != "ont-class" || !doc.fm_ok {
            continue;
        }
        let Some(iri) = doc.scalars.get("iri") else {
            continue;
        };
        if in_subclass_cycle(iri, &subclass) {
            findings.push(Finding::error(
                &doc.path,
                "ONT-008",
                format!("sub-class-of cycle through {iri}"),
            ));
        }
    }

    // arqix:implements REQ-08-01-31-03
    // ONT-009: shadowing means changing (ADR-0021) — a corpus re-declaration
    // of a reserved-core IRI is authorship when identical and a finding when
    // its semantics diverge from the shipped definition.
    for doc in &docs {
        if !doc.family.starts_with("ont-") || !doc.fm_ok {
            continue;
        }
        let Some(iri) = doc.scalars.get("iri") else {
            continue;
        };
        if let Some(shipped) = core_semantics.get(iri)
            && *shipped != ont_semantics(doc)
        {
            findings.push(Finding::error(
                &doc.path,
                "ONT-009",
                format!("reserved core IRI {iri} redefined with different semantics"),
            ));
        }
    }

    // The claim documents the record= references resolve against
    // (REQ-08-01-40-07).
    let claim_ids: HashSet<String> = docs
        .iter()
        .filter(|doc| doc.rdf_types.iter().any(|t| t == "arqix:classes/claim"))
        .filter_map(|doc| doc.scalars.get("id").cloned())
        .collect();

    let roots = crate::config::roots(Path::new("."));
    let mut seen_ids: HashMap<String, String> = HashMap::new();
    let mut seen_iris: HashMap<String, String> = HashMap::new();
    for doc in &docs {
        check_format(doc, contract, &mut findings);
        if !doc.fm_ok {
            continue;
        }
        check_frontmatter(doc, contract, &mut findings);
        check_vocabulary(doc, &vocab, &mut findings, allow_undefined_inverse);
        check_graph_contract(
            doc,
            &subclass,
            &domains,
            &ranges,
            &types_by_iri,
            &mut findings,
        );
        check_claim_markers(doc, &claim_ids, &mut findings);
        if doc.rdf_types.iter().any(|t| t == "arqix:classes/source") {
            check_source(doc, &roots, &mut findings);
        }
        for (kind, seen) in [
            ("id", &mut seen_ids as &mut HashMap<String, String>),
            ("iri", &mut seen_iris),
        ] {
            let value = match doc.scalars.get(kind) {
                Some(v) if !v.is_empty() => v,
                _ => continue,
            };
            if let Some(first) = seen.get(value) {
                findings.push(Finding::error(
                    &doc.path,
                    "FM-006",
                    format!("duplicate {} {} (also in {})", kind, py_repr(value), first),
                ));
            } else {
                seen.insert(value.clone(), basename(&doc.path));
            }
        }
    }

    check_index(&classes_by_label, &mut findings);
    findings.sort_by(|a, b| {
        (a.path.as_str(), a.rule, a.message.as_str()).cmp(&(
            b.path.as_str(),
            b.rule,
            b.message.as_str(),
        ))
    });
    findings
}

// --- reporting ----------------------------------------------------------

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
/// insertion order: findings → [path, rule, level, message], then summary), so
/// the output is byte-identical for the ASCII corpus.
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

// arqix:implements REQ-01-01-11-07
/// `arqix lint frontmatter`.
pub fn lint(format: OutputFormat, allow_undefined_inverse: bool) -> ExitCode {
    let contract = load_contract();
    // A corpus without docs/ontology is valid (ADR-0021): the reserved core
    // and the effective module layers carry the vocabulary.
    let findings = run_checks(&contract, allow_undefined_inverse);
    report(&findings, format)
}

#[cfg(test)]
mod tests {
    use super::*;

    // The oracle's GOOD_CLASS selftest fixture, verbatim.
    const GOOD_CLASS: &str = "---\nid: class-widget\nlabel: widget\niri: arqix:classes/widget\n\nrdf:\n  type:\n    - rdfs:Class\n\nrdfs:\n  sub-class-of:\n    - arqix:classes/widget\n\ntriples: []\n\nproperties: {}\n\nexternal-references: []\n\nowl: {}\n\nmeta:\n  lifecycle-status: draft\n  owner: hcf\n  created: 2026-07-02\n  updated: 2026-07-02\n  lang: en\n  generated: false\n---\n\n## Widget\n\nA selftest fixture class.\n";

    // The oracle's GOOD_STORY selftest fixture, verbatim.
    const GOOD_STORY: &str = "---\nid: US-01-01-01\ntitle: Test Story\nslug: test-story\niri: arqix:user-stories/us-01-01-01\n\nrdf:\n  type:\n    - arqix:classes/widget\n\ntriples:\n  - predicate: arqix:properties/points-at\n    object: arqix:classes/widget\n\nproperties:\n  priority: high\n\nexternal-references: []\n\nmeta:\n  lifecycle-status: draft\n  owner: hcf\n  created: 2026-07-01\n  updated: 2026-07-02\n  lang: en\n  generated: false\n---\n\n## Test Story\n\nAs a tester, I want fixtures, so that the selftest is honest.\n";

    // The oracle's `good_note`, assembled from the same fragments.
    const GOOD_NOTE: &str = "---\ntitle: A Note\nid: note-1\nrdf:\n  type:\n    - arqix:classes/widget\nmeta:\n  lang: en\n---\n\n## A Note\n\nBody.\n";

    fn selftest_vocab() -> Vocab {
        Vocab {
            classes: ["arqix:classes/widget"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            properties: ["arqix:properties/points-at"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            all_iris: [
                "arqix:classes/widget",
                "arqix:properties/points-at",
                "arqix:user-stories/us-01-01-01",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        }
    }

    // The oracle's selftest `apply_config({"kinds": {"note": {...}}})`, expressed
    // as the equivalent `[kinds.note]` TOML.
    fn selftest_contract() -> Contract {
        let mut contract = Contract::defaults();
        let source = r#"
[kinds.note]
dir = "docs/notes"
key-order = ["title", "id", "rdf", "meta"]
required = ["title", "id"]
required-meta = ["lang"]
id-pattern = '^note-(?P<seq>\d+)$'
"#;
        let table: toml::Table = source.parse().unwrap();
        apply_config(&mut contract, &table);
        contract
    }

    /// The oracle's selftest `run(name, text, family, expected_rules, mutate)`:
    /// build a document, run the pure checks, and compare the sorted unique
    /// rule set against the expectation.
    fn run_case(
        name: &str,
        text: &str,
        family: &str,
        expected: &[&str],
        contract: &Contract,
        vocab: &Vocab,
    ) {
        let doc = Doc::new(name.to_string(), text.to_string(), family.to_string());
        let mut findings = Vec::new();
        check_format(&doc, contract, &mut findings);
        if doc.fm_ok {
            check_frontmatter(&doc, contract, &mut findings);
            check_vocabulary(&doc, vocab, &mut findings, false);
        }
        let mut rules: Vec<&str> = findings.iter().map(|f| f.rule).collect();
        rules.sort_unstable();
        rules.dedup();
        let mut want: Vec<&str> = expected.to_vec();
        want.sort_unstable();
        assert_eq!(rules, want, "case {name}");
    }

    // arqix:verifies REQ-01-01-11-07
    #[test]
    fn selftest_cases_match_the_oracle() {
        let contract = selftest_contract();
        let vocab = selftest_vocab();
        let run = |name: &str, text: String, family: &str, expected: &[&str]| {
            run_case(name, &text, family, expected, &contract, &vocab);
        };

        run("widget.md", GOOD_CLASS.to_string(), "ont-class", &[]);

        // US-01-01-19: the configured [kinds.note] contract governs key order,
        // required keys, and required meta for its family.
        run("docs/notes/n.md", GOOD_NOTE.to_string(), "note", &[]);
        run(
            "docs/notes/n.md",
            GOOD_NOTE.replace("title: A Note\nid: note-1", "id: note-1\ntitle: A Note"),
            "note",
            &["FMT-003"],
        );
        run(
            "docs/notes/n.md",
            GOOD_NOTE.replace("\n  lang: en", "\n  owner: x"),
            "note",
            &["FM-001"],
        );
        run(
            "docs/notes/n.md",
            GOOD_NOTE.replace("id: note-1", "id: NOTE_X"),
            "note",
            &["FM-002"],
        );

        run(
            "US-01-01-01-test-story.md",
            GOOD_STORY.to_string(),
            "story",
            &[],
        );
        run(
            "US-01-01-01-test-story.md",
            GOOD_STORY.replace("title: Test Story", "title: \"Test Story\""),
            "story",
            &[],
        );
        run(
            "US-01-01-01-test-story.md",
            format!("\n{GOOD_STORY}"),
            "story",
            &["FMT-001"],
        );
        run(
            "US-01-01-01-test-story.md",
            GOOD_STORY.replacen("---\nid:", "---\n\nid:", 1),
            "story",
            &["FMT-002"],
        );
        run(
            "US-01-01-01-test-story.md",
            GOOD_STORY.replacen(
                "title: Test Story\nslug: test-story",
                "slug: test-story\ntitle: Test Story",
                1,
            ),
            "story",
            &["FMT-003"],
        );
        run(
            "US-01-01-01-test-story.md",
            GOOD_STORY.replace("priority: high", "priority: high "),
            "story",
            &["FMT-004"],
        );
        run(
            "US-01-01-01-test-story.md",
            GOOD_STORY.replace("created: 2026-07-01", "created: 2026-07-03"),
            "story",
            &["FMT-005"],
        );
        run(
            "US-01-01-01-test-story.md",
            GOOD_STORY.replace("created: 2026-07-01", "created: 2026-02-30"),
            "story",
            &["FMT-005"],
        );
        run(
            "US-01-01-01-test-story.md",
            GOOD_STORY.replace("lang: en", "lang: de"),
            "story",
            &["FMT-006"],
        );
        run(
            "US-01-01-01-test-story.md",
            GOOD_STORY.replace("slug: test-story", "slug:"),
            "story",
            &["FM-001"],
        );
        run(
            "US-01-01-99-test-story.md",
            GOOD_STORY.to_string(),
            "story",
            &["FM-002"],
        );
        run(
            "US-01-01-01-test-story.md",
            GOOD_STORY.replace(
                "iri: arqix:user-stories/us-01-01-01",
                "iri: arqix:user-stories/wrong",
            ),
            "story",
            &["FM-003"],
        );
        run(
            "US-01-01-01-other-slug.md",
            GOOD_STORY.to_string(),
            "story",
            &["FM-004"],
        );
        run(
            "US-01-01-01-test-story.md",
            GOOD_STORY.replace("## Test Story", "## Something Else"),
            "story",
            &["FM-005"],
        );
        run(
            "US-01-01-01-test-story.md",
            GOOD_STORY.replace("arqix:properties/points-at", "arqix:properties/undefined"),
            "story",
            &["ONT-001"],
        );
        run(
            "US-01-01-01-test-story.md",
            GOOD_STORY.replacen(
                "- arqix:classes/widget\n\ntriples",
                "- arqix:classes/undefined\n\ntriples",
                1,
            ),
            "story",
            &["ONT-002"],
        );
        run(
            "US-01-01-01-test-story.md",
            GOOD_STORY.replace(
                "object: arqix:classes/widget",
                "object: arqix:classes/nowhere",
            ),
            "story",
            &["ONT-003"],
        );

        let unit = GOOD_STORY
            .replace("id: US-01-01-01", "id: unit-arc42-01")
            .replace(
                "iri: arqix:user-stories/us-01-01-01",
                "iri: arqix:units/unit-arc42-01",
            );
        run(
            "unit-arc42-01-test-story.md",
            unit.clone(),
            "arc42-unit",
            &[],
        );
        run(
            "unit-arc42-01-test-story.md",
            unit.replace(
                "iri: arqix:units/unit-arc42-01",
                "iri: arqix:pages/unit-arc42-01",
            ),
            "arc42-unit",
            &["FM-003"],
        );

        run(
            "widget.md",
            GOOD_CLASS.replace(
                "sub-class-of:\n    - arqix:classes/widget",
                "sub-class-of:\n    - arqix:classes/nowhere",
            ),
            "ont-class",
            &["ONT-004"],
        );
        run(
            "widget.md",
            GOOD_CLASS.replace("owl: {}", "owl:\n  inverse-of: arqix:properties/nowhere"),
            "ont-class",
            &["ONT-005"],
        );
        run(
            "wrong-name.md",
            GOOD_CLASS.to_string(),
            "ont-class",
            &["FM-002"],
        );

        run(
            "US-01-01-01-test-story.md",
            GOOD_STORY.replace("priority: high", "section-kind: arc42-chapter"),
            "story",
            &[],
        );
        run(
            "US-01-01-01-test-story.md",
            GOOD_STORY.replace("priority: high", "section-kind: bogus-kind"),
            "story",
            &["FM-007"],
        );
        run(
            "US-01-01-01-test-story.md",
            GOOD_STORY.replace("lifecycle-status: draft", "lifecycle-status: published"),
            "story",
            &["FM-008"],
        );
        run(
            "US-01-01-01-test-story.md",
            GOOD_STORY.replace("lifecycle-status: draft", "lifecycle-status: done"),
            "story",
            &[],
        );
    }

    // The message strings carry Python-`repr` (`%r`) and list-repr formatting;
    // the clean corpus never exercises them, so pin the representative ones.
    // arqix:verifies REQ-01-01-11-07
    #[test]
    fn finding_messages_match_the_oracle_formatting() {
        let contract = selftest_contract();
        let vocab = selftest_vocab();

        let messages = |name: &str, text: &str, family: &str| -> Vec<String> {
            let doc = Doc::new(name.to_string(), text.to_string(), family.to_string());
            let mut findings = Vec::new();
            check_format(&doc, &contract, &mut findings);
            if doc.fm_ok {
                check_frontmatter(&doc, &contract, &mut findings);
                check_vocabulary(&doc, &vocab, &mut findings, false);
            }
            findings.into_iter().map(|f| f.message).collect()
        };

        // FMT-006: `%r` of the language.
        assert!(
            messages(
                "US-01-01-01-test-story.md",
                &GOOD_STORY.replace("lang: en", "lang: de"),
                "story",
            )
            .contains(&"meta.lang 'de', expected 'en'".to_string())
        );

        // FM-003: two `%r` values, the observed and expected iri.
        assert!(
            messages(
                "US-01-01-01-test-story.md",
                &GOOD_STORY.replace(
                    "iri: arqix:user-stories/us-01-01-01",
                    "iri: arqix:user-stories/wrong",
                ),
                "story",
            )
            .contains(
                &"iri 'arqix:user-stories/wrong', expected 'arqix:user-stories/us-01-01-01'"
                    .to_string()
            )
        );

        // FM-005: a missing heading renders `repr(None)` == "None".
        assert!(
            messages(
                "US-01-01-01-test-story.md",
                &GOOD_STORY.replace("## Test Story\n", ""),
                "story",
            )
            .iter()
            .any(|m| m == "first heading None, expected 'Test Story'")
        );

        // FM-008: `%r` of the value and the sorted vocabulary list.
        assert!(
            messages(
                "US-01-01-01-test-story.md",
                &GOOD_STORY.replace("lifecycle-status: draft", "lifecycle-status: published"),
                "story",
            )
            .contains(
                &"meta.lifecycle-status 'published' is not in the vocabulary \
                  ['done', 'draft', 'in-implementation', 'retired', 'specified']"
                    .to_string()
            )
        );

        // FMT-003: an unknown key and the out-of-order list message.
        let reordered = GOOD_STORY.replacen(
            "title: Test Story\nslug: test-story",
            "slug: test-story\ntitle: Test Story",
            1,
        );
        assert!(
            messages("US-01-01-01-test-story.md", &reordered, "story")
                .iter()
                .any(|m| m
                    == "top-level keys out of canonical order: \
                    ['id', 'slug', 'title', 'iri', 'rdf', 'triples', 'properties', \
                    'external-references', 'meta'] (expected order ['id', 'title', 'slug', \
                    'iri', 'rdf', 'triples', 'derived-triples', 'properties', \
                    'external-references', 'meta'])")
        );
    }

    // arqix:no-requirement
    #[test]
    fn py_repr_matches_cpython_quote_selection() {
        assert_eq!(py_repr("plain"), "'plain'");
        assert_eq!(py_repr("it's"), "\"it's\"");
        assert_eq!(py_repr("it's \"x\""), "'it\\'s \"x\"'");
        assert_eq!(py_repr("a\\b"), "'a\\\\b'");
    }

    // arqix:no-requirement
    #[test]
    fn calendar_validity_matches_datetime() {
        assert!(is_calendar_date("2026-07-02"));
        assert!(is_calendar_date("2024-02-29")); // leap year
        assert!(!is_calendar_date("2026-02-30"));
        assert!(!is_calendar_date("2026-13-01"));
        assert!(!is_calendar_date("0000-01-01")); // year 0 is out of range
    }

    // arqix:no-requirement
    #[test]
    fn parser_reads_sections_and_quotes() {
        let doc = Doc::new(
            "widget.md".to_string(),
            GOOD_CLASS.to_string(),
            "ont-class".to_string(),
        );
        assert!(doc.fm_ok);
        assert_eq!(doc.scalars.get("label").map(String::as_str), Some("widget"));
        assert_eq!(doc.rdf_types, vec!["rdfs:Class"]);
        assert_eq!(
            doc.rdfs.get("sub-class-of").unwrap(),
            &vec!["arqix:classes/widget".to_string()]
        );
        assert_eq!(doc.first_heading().as_deref(), Some("Widget"));

        // A quoted top-level scalar is unquoted before it is stored.
        let quoted = Doc::new(
            "s.md".to_string(),
            GOOD_STORY.replace("title: Test Story", "title: \"Test Story\""),
            "story".to_string(),
        );
        assert_eq!(
            quoted.scalars.get("title").map(String::as_str),
            Some("Test Story")
        );

        // A triple's inline object is attached to its predicate.
        let doc = Doc::new(
            "s.md".to_string(),
            GOOD_STORY.to_string(),
            "story".to_string(),
        );
        assert_eq!(doc.triples.len(), 1);
        assert_eq!(doc.triples[0].0, "arqix:properties/points-at");
        assert_eq!(doc.triples[0].1, vec!["arqix:classes/widget".to_string()]);
    }

    // --- the source provenance contract (SRC rule family) -----------------

    const GOOD_SOURCE: &str = "---\nid: SRC-0001\ntitle: Markdownlint Rule Reference\nslug: markdownlint-rule-reference\niri: arqix:sources/src-0001\n\nrdf:\n  type:\n    - arqix:classes/source\n\ntriples: []\n\nproperties:\n  uri: https://raw.githubusercontent.com/markdownlint/markdownlint/main/docs/RULES.md\n  accessed: 2026-07-16\n  local-copy: sources/markdownlint-rules.md\n  sha256: cbed8b0810f7d5fc478b1a1f9949041ac42f122902cc87a27271fbc5a8093070\n  licence: MIT\n\nexternal-references: []\n\nmeta:\n  lifecycle-status: final\n  owner: hcf\n  created: 2026-07-16\n  updated: 2026-07-16\n  lang: en\n  generated: false\n---\n\n## Markdownlint Rule Reference\n\nA selftest fixture source.\n";

    fn source_findings(text: &str) -> Vec<Finding> {
        let doc = Doc::new(
            "SRC-0001-markdownlint-rule-reference.md".to_string(),
            text.to_string(),
            "source".to_string(),
        );
        let mut findings = Vec::new();
        check_source(&doc, &["docs".to_string()], &mut findings);
        findings
    }

    fn src_rules(findings: &[Finding]) -> Vec<&str> {
        findings.iter().map(|f| f.rule).collect()
    }

    // arqix:verifies REQ-08-01-28-01
    // arqix:verifies REQ-08-01-28-02
    // arqix:verifies REQ-08-01-28-03
    #[test]
    fn complete_final_source_is_clean() {
        let findings = source_findings(GOOD_SOURCE);
        assert!(
            findings.is_empty(),
            "unexpected: {:?}",
            src_rules(&findings)
        );
    }

    // arqix:verifies REQ-08-01-28-01
    #[test]
    fn source_iri_outside_the_namespace_is_reported() {
        let bad = GOOD_SOURCE.replace("iri: arqix:sources/src-0001", "iri: arqix:classes/src-0001");
        let findings = source_findings(&bad);
        assert_eq!(src_rules(&findings), vec!["SRC-001"]);
        assert_eq!(
            findings[0].message,
            "iri 'arqix:classes/src-0001', expected 'arqix:sources/src-0001'"
        );
    }

    // arqix:verifies REQ-08-01-28-02
    #[test]
    fn finalised_source_without_provenance_is_reported() {
        let incomplete = GOOD_SOURCE
            .replace("  uri: https://raw.githubusercontent.com/markdownlint/markdownlint/main/docs/RULES.md\n", "")
            .replace("  accessed: 2026-07-16\n", "");
        let findings = source_findings(&incomplete);
        assert_eq!(src_rules(&findings), vec!["SRC-002", "SRC-002"]);
        assert_eq!(
            findings[0].message,
            "finalised source is missing properties.uri"
        );
        assert_eq!(
            findings[1].message,
            "finalised source is missing properties.accessed"
        );
    }

    // A licence that forbids redistribution means no copy at all: uri and
    // access date finalise the record, and no path or digest pretends
    // a copy exists.
    // arqix:verifies REQ-08-01-28-02
    #[test]
    fn finalised_source_without_a_copy_is_clean() {
        let no_copy = GOOD_SOURCE
            .replace("  local-copy: sources/markdownlint-rules.md\n", "")
            .replace(
                "  sha256: cbed8b0810f7d5fc478b1a1f9949041ac42f122902cc87a27271fbc5a8093070\n",
                "",
            );
        let findings = source_findings(&no_copy);
        assert!(
            findings.is_empty(),
            "unexpected: {:?}",
            src_rules(&findings)
        );
    }

    // arqix:verifies REQ-08-01-28-02
    #[test]
    fn a_lone_copy_or_digest_is_reported() {
        for dropped in [
            "  local-copy: sources/markdownlint-rules.md\n",
            "  sha256: cbed8b0810f7d5fc478b1a1f9949041ac42f122902cc87a27271fbc5a8093070\n",
        ] {
            let lone = GOOD_SOURCE.replace(dropped, "");
            let findings = source_findings(&lone);
            assert_eq!(src_rules(&findings), vec!["SRC-002"], "dropped {dropped}");
            assert_eq!(
                findings[0].message,
                "properties.local-copy and properties.sha256 must be given together"
            );
            // The pairing holds in every lifecycle state.
            let draft = lone.replace("lifecycle-status: final", "lifecycle-status: draft");
            assert_eq!(src_rules(&source_findings(&draft)), vec!["SRC-002"]);
        }
    }

    // arqix:verifies REQ-08-01-28-02
    #[test]
    fn draft_source_without_provenance_is_clean() {
        let skeleton = GOOD_SOURCE
            .replace("  uri: https://raw.githubusercontent.com/markdownlint/markdownlint/main/docs/RULES.md\n", "")
            .replace("  accessed: 2026-07-16\n", "")
            .replace("  local-copy: sources/markdownlint-rules.md\n", "")
            .replace("  sha256: cbed8b0810f7d5fc478b1a1f9949041ac42f122902cc87a27271fbc5a8093070\n", "")
            .replace("lifecycle-status: final", "lifecycle-status: draft");
        let findings = source_findings(&skeleton);
        assert!(
            findings.is_empty(),
            "unexpected: {:?}",
            src_rules(&findings)
        );
    }

    // arqix:verifies REQ-08-01-28-03
    #[test]
    fn malformed_provenance_values_are_reported() {
        let malformed = GOOD_SOURCE
            .replace("accessed: 2026-07-16", "accessed: yesterday")
            .replace(
                "sha256: cbed8b0810f7d5fc478b1a1f9949041ac42f122902cc87a27271fbc5a8093070",
                "sha256: BEEF",
            );
        let findings = source_findings(&malformed);
        assert_eq!(src_rules(&findings), vec!["SRC-003", "SRC-004"]);
        assert_eq!(
            findings[0].message,
            "properties.accessed 'yesterday' is not a calendar date"
        );
        assert_eq!(
            findings[1].message,
            "properties.sha256 'BEEF' is not a 64-character lowercase hex digest"
        );
        // Malformed values are findings even in draft (only absence is excused).
        let draft = malformed.replace("lifecycle-status: final", "lifecycle-status: draft");
        assert_eq!(
            src_rules(&source_findings(&draft)),
            vec!["SRC-003", "SRC-004"]
        );
    }

    // arqix:verifies REQ-08-01-28-03
    #[test]
    fn escaping_or_corpus_resident_local_copy_is_reported() {
        for copy in ["../outside.md", "/tmp/outside.md", "docs/en/copy.md"] {
            let bad = GOOD_SOURCE.replace(
                "local-copy: sources/markdownlint-rules.md",
                &format!("local-copy: {copy}"),
            );
            let findings = source_findings(&bad);
            assert_eq!(src_rules(&findings), vec!["SRC-005"], "copy {copy}");
        }
    }
}
