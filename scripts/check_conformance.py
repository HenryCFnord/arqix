#!/usr/bin/env python3
"""Oracle conformance cross-check (arc42 chapter 8, oracle policy).

The Rust engine owns the trace, frontmatter, and requirements contracts;
the Python oracles (scripts/arqix_trace.py, check_frontmatter.py,
check_requirements.py) are retained as cross-checks for a grace period.
This script asserts the implementations still agree on the real corpus:

    scan       JSON value-equal
    coverage   JSON value-equal
    check      JSON value-equal (known and unknown requirement)
    matrix     byte-identical CSV (both types)
    markers    JSON value-equal (the ported TDD marker gate;
               oracle: check_trace_markers.py)
    lint requirements
               JSON value-equal (the ported requirements checker;
               oracle: check_requirements.py)
    lint frontmatter
               JSON value-equal (the ported frontmatter/formatting/ontology
               checker; oracle: check_frontmatter.py)
    report snapshot
               byte-identical report units under one stamp, plus matching
               freshness `--check` exit codes (oracle: arqix_report.py)

Exit codes are compared on every check alongside the output.

Like the other checkers: stdlib-only, deterministic output, stable exit
codes (0 conformant, 1 divergence, 2 usage/I-O error).

Usage:
    python3 scripts/check_conformance.py [--bin PATH]

--bin defaults to target/debug/arqix; the ARQIX_BIN environment variable
overrides the default as well (the same override the test harness uses).
"""

import argparse
import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
ORACLE = SCRIPT_DIR / "arqix_trace.py"
MARKER_ORACLE = SCRIPT_DIR / "check_trace_markers.py"
REQUIREMENTS_ORACLE = SCRIPT_DIR / "check_requirements.py"
FRONTMATTER_ORACLE = SCRIPT_DIR / "check_frontmatter.py"
REPORT_ORACLE = SCRIPT_DIR / "arqix_report.py"


def run(argv):
    proc = subprocess.run(argv, capture_output=True, text=True)
    return proc.returncode, proc.stdout


def compare_json(name, rust_bin, oracle_args, rust_args):
    rust_code, rust_out = run([rust_bin, *rust_args, "--format", "json"])
    oracle_code, oracle_out = run(
        [sys.executable, str(ORACLE), *oracle_args, "--format", "json"]
    )
    try:
        rust_value = json.loads(rust_out)
        oracle_value = json.loads(oracle_out)
    except json.JSONDecodeError as err:
        print(f"FAIL {name}: output is not JSON ({err})")
        return False
    if rust_value != oracle_value:
        print(f"FAIL {name}: JSON values diverge")
        return False
    if rust_code != oracle_code:
        print(f"FAIL {name}: exit codes diverge (rust {rust_code}, oracle {oracle_code})")
        return False
    print(f"ok   {name} (value-equal, exit {rust_code})")
    return True


def compare_markers(name, rust_bin):
    """`trace markers`: the ported TDD marker gate. The oracle is
    check_trace_markers.py (its own `--json` flag, not arqix_trace.py), so it
    needs its own comparison — JSON value-equal and equal exit codes on the
    real corpus."""
    rust_code, rust_out = run([rust_bin, "trace", "markers", "--format", "json"])
    oracle_code, oracle_out = run([sys.executable, str(MARKER_ORACLE), "--json"])
    try:
        rust_value = json.loads(rust_out)
        oracle_value = json.loads(oracle_out)
    except json.JSONDecodeError as err:
        print(f"FAIL {name}: output is not JSON ({err})")
        return False
    if rust_value != oracle_value:
        print(f"FAIL {name}: JSON values diverge")
        return False
    if rust_code != oracle_code:
        print(f"FAIL {name}: exit codes diverge (rust {rust_code}, oracle {oracle_code})")
        return False
    print(f"ok   {name} (value-equal, exit {rust_code})")
    return True


def compare_requirements(name, rust_bin):
    """`lint requirements`: the ported requirements checker. The oracle is
    check_requirements.py (its own `--json` flag, not arqix_trace.py), so it
    needs its own comparison — JSON value-equal and equal exit codes on the
    real corpus."""
    rust_code, rust_out = run([rust_bin, "lint", "requirements", "--format", "json"])
    oracle_code, oracle_out = run([sys.executable, str(REQUIREMENTS_ORACLE), "--json"])
    try:
        rust_value = json.loads(rust_out)
        oracle_value = json.loads(oracle_out)
    except json.JSONDecodeError as err:
        print(f"FAIL {name}: output is not JSON ({err})")
        return False
    if rust_value != oracle_value:
        print(f"FAIL {name}: JSON values diverge")
        return False
    if rust_code != oracle_code:
        print(f"FAIL {name}: exit codes diverge (rust {rust_code}, oracle {oracle_code})")
        return False
    print(f"ok   {name} (value-equal, exit {rust_code})")
    return True


def compare_frontmatter(name, rust_bin):
    """`lint frontmatter`: the ported frontmatter, formatting, and ontology-
    vocabulary checker. The oracle is check_frontmatter.py (its own `--json`
    flag, not arqix_trace.py), so it needs its own comparison — JSON value-equal
    and equal exit codes on the real corpus."""
    rust_code, rust_out = run([rust_bin, "lint", "frontmatter", "--format", "json"])
    oracle_code, oracle_out = run([sys.executable, str(FRONTMATTER_ORACLE), "--json"])
    try:
        rust_value = json.loads(rust_out)
        oracle_value = json.loads(oracle_out)
    except json.JSONDecodeError as err:
        print(f"FAIL {name}: output is not JSON ({err})")
        return False
    if rust_value != oracle_value:
        print(f"FAIL {name}: JSON values diverge")
        return False
    if rust_code != oracle_code:
        print(f"FAIL {name}: exit codes diverge (rust {rust_code}, oracle {oracle_code})")
        return False
    print(f"ok   {name} (value-equal, exit {rust_code})")
    return True


def compare_snapshot(name, rust_bin):
    """`report snapshot`: the ported question-driven report units (ADR-0008).
    The oracle is arqix_report.py (its own `--snapshot`/`--check` flags, not
    arqix_trace.py), so it needs its own comparison. Generate the units with
    both implementations under the same stamp into separate trees and assert
    byte-identical files, then assert the freshness `--check` exit codes agree
    on the real corpus."""
    stamp = "conformance, 2026-01-01"
    with tempfile.TemporaryDirectory() as tmp:
        rust_out = Path(tmp) / "rust"
        oracle_out = Path(tmp) / "oracle"
        run([rust_bin, "report", "snapshot", "--stamp", stamp, "--out", str(rust_out)])
        run(
            [sys.executable, str(REPORT_ORACLE), "--snapshot", stamp, "--out", str(oracle_out)]
        )
        rust_files = {p.name: p.read_bytes() for p in sorted(rust_out.glob("*.md"))}
        oracle_files = {p.name: p.read_bytes() for p in sorted(oracle_out.glob("*.md"))}
        if rust_files.keys() != oracle_files.keys():
            print(
                f"FAIL {name}: unit set diverges "
                f"(rust {sorted(rust_files)}, oracle {sorted(oracle_files)})"
            )
            return False
        for filename in oracle_files:
            if rust_files[filename] != oracle_files[filename]:
                print(f"FAIL {name}: {filename} bytes diverge")
                return False

    rust_code, _ = run([rust_bin, "report", "snapshot", "--check"])
    oracle_code, _ = run([sys.executable, str(REPORT_ORACLE), "--check"])
    if rust_code != oracle_code:
        print(f"FAIL {name}: --check exit codes diverge (rust {rust_code}, oracle {oracle_code})")
        return False
    print(f"ok   {name} (byte-identical units, --check exit {rust_code})")
    return True


def compare_bytes(name, rust_bin, oracle_args, rust_args):
    rust_code, rust_out = run([rust_bin, *rust_args])
    oracle_code, oracle_out = run([sys.executable, str(ORACLE), *oracle_args])
    if rust_out != oracle_out:
        print(f"FAIL {name}: byte output diverges")
        return False
    if rust_code != oracle_code:
        print(f"FAIL {name}: exit codes diverge (rust {rust_code}, oracle {oracle_code})")
        return False
    print(f"ok   {name} (byte-identical, exit {rust_code})")
    return True


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument(
        "--bin",
        default=os.environ.get("ARQIX_BIN", "target/debug/arqix"),
        help="arqix binary under test (default: target/debug/arqix, or $ARQIX_BIN)",
    )
    args = parser.parse_args(argv)

    if not Path(args.bin).is_file():
        print(f"error: arqix binary not found at {args.bin}", file=sys.stderr)
        return 2
    if not ORACLE.is_file():
        print(f"error: oracle not found at {ORACLE}", file=sys.stderr)
        return 2

    checks = [
        compare_json("trace scan", args.bin, ["scan"], ["trace", "scan"]),
        compare_json("trace coverage", args.bin, ["coverage"], ["trace", "coverage"]),
        # Both trace-check arms: a requirement that exists (the stable
        # cross-cutting determinism requirement) and one that never will.
        compare_json(
            "trace check (known)",
            args.bin,
            ["check", "REQ-00-00-00-01"],
            ["trace", "check", "REQ-00-00-00-01"],
        ),
        compare_json(
            "trace check (unknown)",
            args.bin,
            ["check", "REQ-99-99-99-99"],
            ["trace", "check", "REQ-99-99-99-99"],
        ),
        compare_bytes(
            "trace matrix req-test",
            args.bin,
            ["matrix", "--type", "req-test"],
            ["trace", "matrix", "--type", "req-test"],
        ),
        compare_bytes(
            "trace matrix us-req",
            args.bin,
            ["matrix", "--type", "us-req"],
            ["trace", "matrix", "--type", "us-req"],
        ),
        compare_markers("trace markers", args.bin),
        compare_requirements("lint requirements", args.bin),
        compare_frontmatter("lint frontmatter", args.bin),
        compare_snapshot("report snapshot", args.bin),
    ]
    ok = all(checks)
    print(f"conformance: {'ok' if ok else 'FAILED'} ({sum(checks)}/{len(checks)})")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
