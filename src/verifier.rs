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
const REGISTRY: [(&str, &[&str]); 10] = [
    ("format", &["fmt", "--check"]),
    ("lint", &["lint", "run"]),
    ("trace-scan", &["trace", "scan"]),
    ("coverage", &["trace", "coverage"]),
    ("ratchet", &["trace", "ratchet"]),
    ("freshness", &["trace", "freshness"]),
    // The ported corpus checks the product now self-hosts (REQ-04-01-14-04).
    ("requirements", &["lint", "requirements"]),
    ("frontmatter", &["lint", "frontmatter"]),
    ("markers", &["trace", "markers"]),
    ("report-freshness", &["report", "snapshot", "--check"]),
];

// arqix:implements REQ-04-01-05-01
// arqix:implements REQ-04-01-05-02
// arqix:implements REQ-04-01-05-03
// arqix:implements REQ-04-01-05-04
// arqix:implements REQ-04-01-14-01
// arqix:implements REQ-04-01-14-02
// arqix:implements REQ-04-01-14-04
// arqix:implements REQ-04-01-14-05
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
        let informational = policy.informational.iter().any(|i| i == name);

        // report-freshness follows the snapshot strategy (REQ-04-01-14-05): in a
        // context the strategy does not gate — `on-demand`, or `main-only` off
        // the default branch — the step is skipped rather than run, so a
        // legitimately stale snapshot on a parallel branch never fails the loop.
        if name == "report-freshness" && !report_freshness_gates(std::path::Path::new(".")) {
            effective_codes.push(0);
            steps.push(json!({
                "step": name,
                "exit_code": 0,
                "ok": true,
                "informational": informational,
                "skipped": true,
            }));
            continue;
        }

        let code = Command::new(&exe)
            .args(*args)
            .output()
            .ok()
            .and_then(|o| o.status.code())
            .map(i64::from)
            .unwrap_or(-1);
        // An informational step forgives findings (exit 1), never system
        // errors: a crashed sub-step is a broken loop either way.
        let effective = if informational && code == 1 { 0 } else { code };
        effective_codes.push(effective);
        steps.push(json!({
            "step": name,
            "exit_code": code,
            "ok": code == 0,
            "informational": informational,
            "skipped": false,
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
                let skipped = step["skipped"].as_bool().unwrap_or(false);
                let name = step["step"].as_str().unwrap_or("?");
                if skipped {
                    println!("skip {name} (skipped)");
                    continue;
                }
                let status = if step["ok"].as_bool().unwrap_or(false) {
                    "ok"
                } else if informational {
                    "info"
                } else {
                    "FAIL"
                };
                println!("{status:4} {name} (exit {})", step["exit_code"]);
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

// arqix:implements REQ-04-01-14-05
/// Whether the report-freshness sub-step gates in the current context. The
/// report snapshot strategy (config-audit row C17) decides: `committed` gates
/// everywhere, `on-demand` never gates, and `main-only` gates only on the
/// default branch — the resolution the reference sequencer performs
/// (`scripts/arqix`, step 9). A non-gating context skips the step rather than
/// running a check that must not fail there.
fn report_freshness_gates(dir: &std::path::Path) -> bool {
    match crate::config::snapshot_strategy(dir).as_str() {
        "on-demand" => false,
        "main-only" => matches!(current_branch().as_deref(), Some("main") | Some("master")),
        // "committed" and any unrecognised value gate everywhere.
        _ => true,
    }
}

/// The current branch for the `main-only` resolution: the CI-provided
/// `GITHUB_REF_NAME` if set, otherwise `git rev-parse --abbrev-ref HEAD`. None
/// when neither is available, which is treated as a non-default branch.
fn current_branch() -> Option<String> {
    if let Ok(name) = std::env::var("GITHUB_REF_NAME")
        && !name.is_empty()
    {
        return Some(name);
    }
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let branch = String::from_utf8(output.stdout).ok()?.trim().to_string();
    if branch.is_empty() {
        None
    } else {
        Some(branch)
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
