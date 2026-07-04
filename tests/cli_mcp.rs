//! Command contract: `mcp serve` — owned by the MCP Server
//! (arc42 chapter 5): transport separated from tool logic.

mod common;

use std::io::Write;
use std::process::{Command, Stdio};

// arqix:verifies REQ-05-01-12-01
#[test]
#[ignore = "US-05-01-12: not implemented"]
fn mcp_serve_speaks_jsonrpc_over_stdio() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_arqix"))
        .current_dir(common::fixture("minimal"))
        .args(["mcp", "serve"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to start mcp serve");

    let request = "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"initialize\",\"params\":{}}\n";
    child
        .stdin
        .take()
        .expect("stdin piped")
        .write_all(request.as_bytes())
        .expect("failed to write initialize request");

    let output = child.wait_with_output().expect("mcp serve did not exit");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("jsonrpc"),
        "expected a JSON-RPC response: {stdout}"
    );
}
