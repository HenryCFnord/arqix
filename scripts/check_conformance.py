#!/usr/bin/env python3
"""Trace-oracle conformance cross-check (arc42 chapter 8, oracle policy).

The Rust trace engine owns the `trace` contract; the Python oracle
(scripts/arqix_trace.py) is retained as a cross-check for a grace period.
This script asserts the two implementations still agree on the real corpus:

    scan       JSON value-equal
    coverage   JSON value-equal
    check      JSON value-equal (known and unknown requirement)
    matrix     byte-identical CSV (both types)

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
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
ORACLE = SCRIPT_DIR / "arqix_trace.py"


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
    ]
    ok = all(checks)
    print(f"conformance: {'ok' if ok else 'FAILED'} ({sum(checks)}/{len(checks)})")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
