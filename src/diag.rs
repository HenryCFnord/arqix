//! Shared machine-readable diagnostics (REQ-00-00-00-03, ADR-0006 layer 2):
//! severity, stable code, message, optional source location. Every command
//! that reports findings emits this one shape, so agents and CI parse a
//! single format across the whole tool.

use crate::OutputFormat;
use serde_json::{Value, json};
use std::process::ExitCode;

pub const SCHEMA_VERSION: u64 = 1;

#[derive(Clone)]
pub struct Diagnostic {
    pub severity: &'static str,
    pub code: &'static str,
    pub message: String,
    pub file: Option<String>,
    pub line: Option<usize>,
}

impl Diagnostic {
    pub fn error(code: &'static str, message: impl Into<String>) -> Self {
        Diagnostic {
            severity: "error",
            code,
            message: message.into(),
            file: None,
            line: None,
        }
    }

    pub fn warning(code: &'static str, message: impl Into<String>) -> Self {
        Diagnostic {
            severity: "warning",
            code,
            message: message.into(),
            file: None,
            line: None,
        }
    }

    pub fn at(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }

    // Used by the linter and assembler slices (file:line findings).
    #[allow(dead_code)]
    pub fn at_line(mut self, file: impl Into<String>, line: usize) -> Self {
        self.file = Some(file.into());
        self.line = Some(line);
        self
    }

    pub fn to_json(&self) -> Value {
        let mut obj = json!({
            "severity": self.severity,
            "code": self.code,
            "message": self.message,
        });
        if let Some(f) = &self.file {
            obj["file"] = json!(f);
        }
        if let Some(l) = self.line {
            obj["line"] = json!(l);
        }
        obj
    }
}

pub fn has_errors(diags: &[Diagnostic]) -> bool {
    diags.iter().any(|d| d.severity == "error")
}

pub fn exit_code(diags: &[Diagnostic]) -> ExitCode {
    if has_errors(diags) {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

/// The JSON diagnostics payload (ADR-0006 layer 2).
pub fn to_report(diags: &[Diagnostic]) -> Value {
    json!({
        "schema_version": SCHEMA_VERSION,
        "diagnostics": diags.iter().map(Diagnostic::to_json).collect::<Vec<_>>(),
    })
}

/// Emit a diagnostics report in the requested format.
pub fn emit(diags: &[Diagnostic], format: OutputFormat) {
    match format {
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&to_report(diags)).expect("valid JSON")
            );
        }
        OutputFormat::Text => {
            for d in diags {
                let loc = match (&d.file, d.line) {
                    (Some(f), Some(l)) => format!("{f}:{l}: "),
                    (Some(f), None) => format!("{f}: "),
                    _ => String::new(),
                };
                println!("{loc}{}: {}: {}", d.code, d.severity, d.message);
            }
        }
    }
}
