// arqix - a CLI for structured technical documentation and Architecture-as-Code workflows.
// This is the initial skeleton. Commands and subcommands will be added incrementally.

use std::env;
use std::process;

fn main() {
    let mut args = env::args().skip(1);

    if let Some(first) = args.next() {
        match first.as_str() {
            "--help" | "-h" => {
                println!("arqix v{}", env!("CARGO_PKG_VERSION"));
                println!("A CLI for structured technical documentation.");
                println!();
                println!("Usage:");
                println!("  arqix [OPTIONS]");
                println!();
                println!("Options:");
                println!("  -h, --help       Print this help message");
                println!("  -V, --version    Print version information");
                return;
            }
            "--version" | "-V" => {
                println!("arqix v{}", env!("CARGO_PKG_VERSION"));
                return;
            }
            unknown => {
                eprintln!("error: unrecognized argument `{unknown}`");
                eprintln!("Run `arqix --help` for usage.");
                process::exit(2);
            }
        }
    }

    println!("arqix v{}", env!("CARGO_PKG_VERSION"));
    println!("A CLI for structured technical documentation.");
    println!("Run `arqix --help` for usage.");
}
