//! Verification Orchestrator (ADR-0003): the one-command quality loop. It
//! sequences the configured sub-steps through the same stable command
//! interface the entrypoint uses — it never implements a check itself — and
//! aggregates per-step results. Rendering is deliberately not part of the
//! default loop (REQ-04-01-05-04).

use crate::OutputFormat;
use crate::diag::SCHEMA_VERSION;
use serde_json::json;
use std::process::{Command, ExitCode};

/// The default sub-steps, in order. Each is invoked as `arqix <args>`, so the
/// orchestrator depends only on the command interface (ADR-0003). Rendering
/// and publishing are absent by design (REQ-04-01-05-04).
const STEPS: [(&str, &[&str]); 4] = [
    ("format", &["fmt", "--check"]),
    ("lint", &["lint", "run"]),
    ("trace-scan", &["trace", "scan"]),
    ("coverage", &["trace", "coverage"]),
];

// arqix:implements REQ-04-01-05-01
// arqix:implements REQ-04-01-05-02
// arqix:implements REQ-04-01-05-03
// arqix:implements REQ-04-01-05-04
/// `arqix verify [--fail-fast | --aggregate]`
pub fn verify(fail_fast: bool, format: OutputFormat) -> ExitCode {
    let exe = match std::env::current_exe() {
        Ok(exe) => exe,
        Err(err) => {
            eprintln!("error: cannot locate the arqix executable: {err}");
            return ExitCode::from(2);
        }
    };

    let mut steps = Vec::new();
    let mut failed = false;
    for (name, args) in STEPS {
        let code = Command::new(&exe)
            .args(args)
            .output()
            .ok()
            .and_then(|o| o.status.code())
            .unwrap_or(-1);
        let ok = code == 0;
        steps.push(json!({ "step": name, "exit_code": code, "ok": ok }));
        if !ok {
            failed = true;
            if fail_fast {
                break;
            }
        }
    }

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
                let status = if step["ok"].as_bool().unwrap_or(false) {
                    "ok"
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

    if failed {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
