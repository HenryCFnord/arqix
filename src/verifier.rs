//! Verification Orchestrator (ADR-0003): the one-command quality loop. It
//! sequences the configured sub-steps through the same stable command
//! interface the entrypoint uses — it never implements a check itself — and
//! aggregates per-step results. Rendering is deliberately not part of the
//! default loop (REQ-04-01-05-04).

use crate::OutputFormat;
use crate::diag::SCHEMA_VERSION;
use serde_json::json;
use std::process::{Command, ExitCode};

/// The sub-steps the loop can run. Each is invoked as `arqix <args>`, so the
/// orchestrator depends only on the command interface (ADR-0003). Rendering
/// and publishing are absent by design (REQ-04-01-05-04).
const REGISTRY: [(&str, &[&str]); 6] = [
    ("format", &["fmt", "--check"]),
    ("lint", &["lint", "run"]),
    ("trace-scan", &["trace", "scan"]),
    ("coverage", &["trace", "coverage"]),
    ("ratchet", &["trace", "ratchet"]),
    ("freshness", &["trace", "freshness"]),
];

// arqix:implements REQ-04-01-05-01
// arqix:implements REQ-04-01-05-02
// arqix:implements REQ-04-01-05-03
// arqix:implements REQ-04-01-05-04
// arqix:implements REQ-04-01-14-01
// arqix:implements REQ-04-01-14-02
// arqix:implements REQ-03-01-11-03
/// `arqix verify [--fail-fast | --aggregate]` — runs the sub-steps declared
/// in the effective `[policies.verify]` configuration, in their configured
/// order; informational steps report findings without gating.
pub fn verify(fail_fast: bool, format: OutputFormat) -> ExitCode {
    let exe = match std::env::current_exe() {
        Ok(exe) => exe,
        Err(err) => {
            eprintln!("error: cannot locate the arqix executable: {err}");
            return ExitCode::from(2);
        }
    };
    let policy = crate::config::verify_policy(std::path::Path::new("."));

    let mut steps = Vec::new();
    let mut effective_codes = Vec::new();
    for name in &policy.steps {
        let Some((_, args)) = REGISTRY.iter().find(|(known, _)| known == name) else {
            eprintln!(
                "error: [policies.verify] steps: unknown sub-step '{name}' (known: {})",
                REGISTRY.map(|(n, _)| n).join(", ")
            );
            return ExitCode::from(2);
        };
        let code = Command::new(&exe)
            .args(*args)
            .output()
            .ok()
            .and_then(|o| o.status.code())
            .map(i64::from)
            .unwrap_or(-1);
        let informational = policy.informational.iter().any(|i| i == name);
        // An informational step forgives findings (exit 1), never system
        // errors: a crashed sub-step is a broken loop either way.
        let effective = if informational && code == 1 { 0 } else { code };
        effective_codes.push(effective);
        steps.push(json!({
            "step": name,
            "exit_code": code,
            "ok": code == 0,
            "informational": informational,
        }));
        if effective != 0 && fail_fast {
            break;
        }
    }

    let failed = effective_codes.iter().any(|c| *c != 0);
    let report = json!({ "schema_version": SCHEMA_VERSION, "steps": steps, "ok": !failed });
    match format {
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&report).expect("valid JSON")
            );
        }
        OutputFormat::Text => {
            for step in &steps {
                let informational = step["informational"].as_bool().unwrap_or(false);
                let status = if step["ok"].as_bool().unwrap_or(false) {
                    "ok"
                } else if informational {
                    "info"
                } else {
                    "FAIL"
                };
                println!(
                    "{status:4} {} (exit {})",
                    step["step"].as_str().unwrap_or("?"),
                    step["exit_code"],
                );
            }
            println!("verify: {}", if failed { "FAILED" } else { "ok" });
        }
    }

    ExitCode::from(overall_exit(&effective_codes))
}

/// Aggregate sub-step exit codes without losing severity: any step that did
/// not end in the findings channel (0/1) — a usage error, an I/O error, a
/// crash — surfaces as exit 2, never collapsed into exit 1.
fn overall_exit(codes: &[i64]) -> u8 {
    if codes.iter().any(|c| !matches!(c, 0 | 1)) {
        2
    } else if codes.contains(&1) {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::overall_exit;

    // arqix:no-requirement
    #[test]
    fn sub_step_system_errors_are_not_collapsed_to_findings() {
        assert_eq!(overall_exit(&[0, 0, 0, 0]), 0);
        assert_eq!(overall_exit(&[0, 1, 0, 1]), 1);
        // A step exiting 2 (or dying, -1) is a system error, not a finding.
        assert_eq!(overall_exit(&[0, 2, 0, 1]), 2);
        assert_eq!(overall_exit(&[0, -1]), 2);
    }
}
