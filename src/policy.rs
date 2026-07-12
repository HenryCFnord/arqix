//! Policy Checker: changed files versus the declared change scope
//! (US-01-01-07, arc42 chapter 5). The changed-file list is external —
//! positional arguments, e.g. from `git diff --name-only` — and the policy
//! comes from `[policies.change]` in arqix.toml via the Config Resolver.
//!
//! The scope grammar is deliberately minimal: `allow` is a list of path
//! prefixes where a trailing slash declares a subtree and an exact entry
//! declares that one file. Violations are ordinary diagnostics (POL-001),
//! so gating (errors, exit 1) versus warn-only (warnings, exit 0) falls
//! out of the shared severity model rather than a special exit path.

use crate::OutputFormat;
use crate::config::{self, ChangePolicy};
use crate::diag::{self, Diagnostic};
use std::path::Path;
use std::process::ExitCode;

// arqix:implements REQ-01-01-07-02
/// Whether `file` is inside the declared scope: subtree entries (trailing
/// slash) match by prefix, plain entries match exactly.
fn allowed(policy: &ChangePolicy, file: &str) -> bool {
    policy.allow.iter().any(|entry| {
        if entry.ends_with('/') {
            file.starts_with(entry.as_str())
        } else {
            file == entry
        }
    })
}

// arqix:implements REQ-01-01-07-02
// arqix:implements REQ-01-01-07-03
/// `arqix policy check <file>...`
pub fn check(files: &[String], format: OutputFormat) -> ExitCode {
    // arqix:implements REQ-01-01-07-01
    let Some(policy) = config::change_policy(Path::new(".")) else {
        // The mechanism is optional: no declared policy means there is
        // nothing to enforce (US-01-01-07 acceptance criteria).
        match format {
            OutputFormat::Json => diag::emit(&[], format),
            OutputFormat::Text => {
                println!("no change policy declared ([policies.change]) — nothing to enforce");
            }
        }
        return ExitCode::SUCCESS;
    };

    let warn_only = policy.mode == "warn";
    let diagnostics: Vec<Diagnostic> = files
        .iter()
        .filter(|file| !allowed(&policy, file))
        .map(|file| {
            let message = format!("{file} is outside the declared change scope");
            if warn_only {
                Diagnostic::warning("POL-001", message).at(file)
            } else {
                Diagnostic::error("POL-001", message).at(file)
            }
        })
        .collect();

    diag::emit(&diagnostics, format);
    if diagnostics.is_empty() && matches!(format, OutputFormat::Text) {
        println!("change scope ok ({} file(s) checked)", files.len());
    }
    diag::exit_code(&diagnostics)
}
