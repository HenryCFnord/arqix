//! Config Resolver: the effective configuration from defaults plus
//! arqix.toml overrides (REQ-00-00-00-06), with schema validation and the
//! `config show` rendering (REQ-01-01-16-*). Schema v1 is documented in
//! docs/en/processes/configuration.md.
//!
//! Validation walks the parsed TOML table manually so every diagnostic can
//! name the failing key precisely (REQ-01-01-16-03) — no derive layer
//! between the file and the finding.

use crate::OutputFormat;
use crate::diag::{self, Diagnostic};
use serde_json::{Map, Value};
use std::path::Path;
use std::process::ExitCode;

const CONFIG_FILE: &str = "arqix.toml";
const SCHEMA_VERSION: u64 = 1;

/// Known optional table sections accepted (and not yet validated) in v1.
const KNOWN_SECTIONS: [&str; 4] = ["kinds", "templates", "policies", "i18n"];

// arqix:implements REQ-01-01-17-02
/// Directories document discovery never descends into unless overridden by
/// a `skip-dirs` config entry. The trace corpus walk keeps its own fixed
/// copy of this set (src/trace.rs) for oracle conformance.
const DEFAULT_SKIP_DIRS: [&str; 5] = [".git", "target", "node_modules", "__pycache__", "fixtures"];

/// The effective configuration: schema-v1 defaults merged with overrides.
pub struct EffectiveConfig {
    pub roots: Vec<String>,
    pub skip_dirs: Vec<String>,
    pub sections: Map<String, Value>,
}

impl Default for EffectiveConfig {
    fn default() -> Self {
        EffectiveConfig {
            roots: vec!["docs".to_string()],
            skip_dirs: DEFAULT_SKIP_DIRS.iter().map(|s| s.to_string()).collect(),
            sections: Map::new(),
        }
    }
}

// arqix:implements REQ-00-00-00-06
// arqix:implements REQ-01-01-16-01
// arqix:implements REQ-01-01-16-03
/// Resolve arqix.toml from `dir` into the effective configuration plus
/// diagnostics. A missing file is valid: it means no overrides.
fn resolve(dir: &Path) -> (EffectiveConfig, Vec<Diagnostic>) {
    let mut config = EffectiveConfig::default();
    let mut diagnostics = Vec::new();

    let text = match std::fs::read_to_string(dir.join(CONFIG_FILE)) {
        Ok(text) => text,
        Err(_) => return (config, diagnostics),
    };

    let table: toml::Table = match text.parse() {
        Ok(table) => table,
        Err(err) => {
            diagnostics.push(
                Diagnostic::error("CFG-001", format!("not parseable as TOML: {err}"))
                    .at(CONFIG_FILE),
            );
            return (config, diagnostics);
        }
    };

    for (key, value) in &table {
        match key.as_str() {
            "roots" => match string_array(value) {
                Ok(roots) => config.roots = roots,
                Err(found) => diagnostics.push(
                    Diagnostic::error(
                        "CFG-001",
                        format!("roots: expected an array of strings, found {found}"),
                    )
                    .at(CONFIG_FILE),
                ),
            },
            // arqix:implements REQ-01-01-17-01
            "skip-dirs" => match string_array(value) {
                Ok(dirs) => config.skip_dirs = dirs,
                Err(found) => diagnostics.push(
                    Diagnostic::error(
                        "CFG-001",
                        format!("skip-dirs: expected an array of strings, found {found}"),
                    )
                    .at(CONFIG_FILE),
                ),
            },
            key if KNOWN_SECTIONS.contains(&key) => {
                if value.is_table() {
                    config.sections.insert(key.to_string(), toml_to_json(value));
                } else {
                    diagnostics.push(
                        Diagnostic::error(
                            "CFG-001",
                            format!("{key}: expected a table, found {}", value.type_str()),
                        )
                        .at(CONFIG_FILE),
                    );
                }
            }
            unknown => diagnostics.push(
                Diagnostic::warning(
                    "CFG-002",
                    format!("{unknown}: unknown key, ignored (schema v1)"),
                )
                .at(CONFIG_FILE),
            ),
        }
    }

    (config, diagnostics)
}

/// The effective document roots for discovery — defaults plus overrides.
/// Validation is `config validate`'s job; discovery falls back to defaults
/// for a malformed file rather than failing.
pub fn roots(dir: &Path) -> Vec<String> {
    resolve(dir).0.roots
}

/// The verify loop's effective policy: which sub-steps run, in which order,
/// and which of them are informational (findings reported, exit untouched).
pub struct VerifyPolicy {
    pub steps: Vec<String>,
    pub informational: Vec<String>,
    /// The ratchet's baseline file (C17, REQ-04-01-16-01); None means the
    /// built-in default snapshot location.
    pub ratchet_baseline: Option<String>,
}

impl Default for VerifyPolicy {
    fn default() -> Self {
        VerifyPolicy {
            // Coverage measures project progress, so it must never gate a
            // change by default (ADR-0010 discussion, REQ-04-01-14-03);
            // what gates instead is the regression ratchet.
            steps: ["format", "lint", "trace-scan", "coverage", "ratchet"]
                .map(str::to_string)
                .to_vec(),
            informational: vec!["coverage".to_string()],
            ratchet_baseline: None,
        }
    }
}

// arqix:implements REQ-04-01-14-01
// arqix:implements REQ-04-01-14-03
/// The effective `[policies.verify]` table — defaults unless overridden.
pub fn verify_policy(dir: &Path) -> VerifyPolicy {
    let mut policy = VerifyPolicy::default();
    let (config, _) = resolve(dir);
    let Some(verify) = config
        .sections
        .get("policies")
        .and_then(|p| p.get("verify"))
    else {
        return policy;
    };
    if let Some(steps) = json_string_array(verify.get("steps")) {
        policy.steps = steps;
    }
    if let Some(informational) = json_string_array(verify.get("informational")) {
        policy.informational = informational;
    }
    // arqix:implements REQ-04-01-16-01
    policy.ratchet_baseline = verify
        .get("ratchet-baseline")
        .and_then(Value::as_str)
        .map(str::to_string);
    policy
}

fn json_string_array(value: Option<&Value>) -> Option<Vec<String>> {
    let items = value?.as_array()?;
    items
        .iter()
        .map(|item| item.as_str().map(str::to_string))
        .collect()
}

/// The effective skip list for document discovery — the default set unless
/// a `skip-dirs` override replaces it (REQ-01-01-17-01/-02).
pub fn skip_dirs(dir: &Path) -> Vec<String> {
    resolve(dir).0.skip_dirs
}

/// The publish policy: where artefact-ready inputs are staged, how the site
/// is stitched, and the toolchain that renders it (there is deliberately no
/// built-in renderer).
pub struct PublishPolicy {
    pub staging_dir: String,
    pub stitching: String,
    pub site_command: Option<String>,
    /// Language-root-relative path prefixes that never stage (the publish
    /// scope; the ADR-0010 final-filter is the lifecycle-based successor).
    pub exclude: Vec<String>,
    /// Repository-root-relative files or directories copied verbatim into
    /// the staging dir (logo, favicon — the toolchain can only reference
    /// what reaches staging).
    pub assets: Vec<String>,
}

// arqix:implements REQ-04-01-03-03
/// The effective `[policies.publish]` table — defaults unless overridden.
pub fn publish_policy(dir: &Path) -> PublishPolicy {
    let (config, _) = resolve(dir);
    let publish = config
        .sections
        .get("policies")
        .and_then(|p| p.get("publish"));
    PublishPolicy {
        staging_dir: publish
            .and_then(|p| p.get("staging-dir"))
            .and_then(Value::as_str)
            .unwrap_or("site-src")
            .to_string(),
        stitching: publish
            .and_then(|p| p.get("stitching"))
            .and_then(Value::as_str)
            .unwrap_or("single-page")
            .to_string(),
        site_command: publish
            .and_then(|p| p.get("site-command"))
            .and_then(Value::as_str)
            .map(str::to_string),
        exclude: publish
            .and_then(|p| json_string_array(p.get("exclude")))
            .unwrap_or_default(),
        assets: publish
            .and_then(|p| json_string_array(p.get("assets")))
            .unwrap_or_default(),
    }
}

/// The assemble policy: who owns section headings in a stitched corpus
/// (ADR-0013). `child` (default) — fragments own their headings and a bare
/// include behaves as `level=+1`; `parent` — the page declares the outline
/// and a bare include inlines verbatim.
// arqix:implements REQ-02-01-12-04
pub fn heading_ownership(dir: &Path) -> String {
    let (config, _) = resolve(dir);
    config
        .sections
        .get("policies")
        .and_then(|p| p.get("assemble"))
        .and_then(|a| a.get("heading-ownership"))
        .and_then(Value::as_str)
        .unwrap_or("child")
        .to_string()
}

// arqix:implements REQ-01-01-20-01
/// The configured template directory (`[templates] dir`), if any — the
/// engine falls back to the package-local scaffold and the embedded
/// default when it is absent (US-01-01-20).
pub fn templates_dir(dir: &Path) -> Option<String> {
    let (config, _) = resolve(dir);
    config
        .sections
        .get("templates")?
        .get("dir")?
        .as_str()
        .map(str::to_string)
}

/// One configured document-family contract (`[kinds.<family>]`): the
/// directory that selects the family and its canonical frontmatter key
/// order (US-01-01-19). Families without a `dir` cannot be matched to
/// files and are skipped.
pub struct KindContract {
    pub family: String,
    pub dir: String,
    pub key_order: Option<Vec<String>>,
    /// The family's ID pattern (ADR-0012): a regex governing shape,
    /// uniqueness scope, and — through its named groups — generation
    /// (`seq`) and consistency checks (`story`).
    pub id_pattern: Option<String>,
}

// arqix:implements REQ-01-01-19-01
// arqix:implements REQ-01-01-19-02
/// The configured family contracts, longest directory first so the most
/// specific family wins (units under a page directory, for example).
pub fn kind_contracts(base: &Path) -> Vec<KindContract> {
    let (config, _) = resolve(base);
    let mut contracts: Vec<KindContract> = config
        .sections
        .get("kinds")
        .and_then(Value::as_object)
        .map(|kinds| {
            kinds
                .iter()
                .filter_map(|(family, entry)| {
                    Some(KindContract {
                        family: family.clone(),
                        dir: entry
                            .get("dir")?
                            .as_str()?
                            .trim_end_matches('/')
                            .to_string(),
                        key_order: json_string_array(entry.get("key-order")),
                        id_pattern: entry
                            .get("id-pattern")
                            .and_then(Value::as_str)
                            .map(str::to_string),
                    })
                })
                .collect()
        })
        .unwrap_or_default();
    contracts.sort_by_key(|c| std::cmp::Reverse(c.dir.len()));
    contracts
}

// arqix:implements REQ-01-01-18-01
/// The configured ID pattern for one document kind, by family name — the
/// generation side of the ID policy (`doc new`).
pub fn id_pattern_for_kind(base: &Path, kind: &str) -> Option<String> {
    kind_contracts(base)
        .into_iter()
        .find(|c| c.family == kind)
        .and_then(|c| c.id_pattern)
}

/// The corpus default language (`[i18n] default-lang`, default `en`).
pub fn default_lang(dir: &Path) -> String {
    let (config, _) = resolve(dir);
    config
        .sections
        .get("i18n")
        .and_then(|i| i.get("default-lang"))
        .and_then(Value::as_str)
        .unwrap_or("en")
        .to_string()
}

fn string_array(value: &toml::Value) -> Result<Vec<String>, String> {
    let items = value
        .as_array()
        .ok_or_else(|| value.type_str().to_string())?;
    items
        .iter()
        .map(|item| {
            item.as_str()
                .map(str::to_string)
                .ok_or_else(|| item.type_str().to_string())
        })
        .collect()
}

fn toml_to_json(value: &toml::Value) -> Value {
    match value {
        toml::Value::String(s) => Value::from(s.clone()),
        toml::Value::Integer(i) => Value::from(*i),
        toml::Value::Float(f) => Value::from(*f),
        toml::Value::Boolean(b) => Value::from(*b),
        toml::Value::Datetime(d) => Value::from(d.to_string()),
        toml::Value::Array(items) => {
            Value::from(items.iter().map(toml_to_json).collect::<Vec<_>>())
        }
        toml::Value::Table(table) => Value::Object(
            table
                .iter()
                .map(|(k, v)| (k.clone(), toml_to_json(v)))
                .collect(),
        ),
    }
}

// arqix:implements REQ-01-01-16-01
/// `arqix config validate`
pub fn validate(format: OutputFormat) -> ExitCode {
    let (_, diagnostics) = resolve(Path::new("."));
    diag::emit(&diagnostics, format);
    if diagnostics.is_empty() && matches!(format, OutputFormat::Text) {
        println!("configuration ok");
    }
    diag::exit_code(&diagnostics)
}

// arqix:implements REQ-01-01-16-02
/// `arqix config show`
pub fn show(format: OutputFormat) -> ExitCode {
    let (config, diagnostics) = resolve(Path::new("."));
    if diag::has_errors(&diagnostics) {
        diag::emit(&diagnostics, format);
        return ExitCode::from(1);
    }

    let mut effective = Map::new();
    effective.insert("schema_version".to_string(), Value::from(SCHEMA_VERSION));
    effective.insert("roots".to_string(), Value::from(config.roots.clone()));
    effective.insert(
        "skip-dirs".to_string(),
        Value::from(config.skip_dirs.clone()),
    );
    for (key, value) in &config.sections {
        effective.insert(key.clone(), value.clone());
    }

    match format {
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&Value::Object(effective)).expect("valid JSON")
            );
        }
        OutputFormat::Text => {
            println!("roots = {:?}", config.roots);
            println!("skip-dirs = {:?}", config.skip_dirs);
            for key in config.sections.keys() {
                println!("{key} = <table>");
            }
        }
    }
    ExitCode::SUCCESS
}
