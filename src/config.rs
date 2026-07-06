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

/// The effective configuration: schema-v1 defaults merged with overrides.
pub struct EffectiveConfig {
    pub roots: Vec<String>,
    pub sections: Map<String, Value>,
}

impl Default for EffectiveConfig {
    fn default() -> Self {
        EffectiveConfig {
            roots: vec!["docs".to_string()],
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
            "roots" => match roots_from(value) {
                Ok(roots) => config.roots = roots,
                Err(found) => diagnostics.push(
                    Diagnostic::error(
                        "CFG-001",
                        format!("roots: expected an array of strings, found {found}"),
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

fn roots_from(value: &toml::Value) -> Result<Vec<String>, String> {
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
    let code = diag::exit_code(&diagnostics);
    diag::emit(&diagnostics, format);
    if diagnostics.is_empty() && matches!(format, OutputFormat::Text) {
        println!("configuration ok");
    }
    code
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
            for key in config.sections.keys() {
                println!("{key} = <table>");
            }
        }
    }
    ExitCode::SUCCESS
}
