// arqix - a CLI for structured technical documentation and Architecture-as-Code workflows.
//
// Command surface per ADR-0005 (noun-verb scheme, `verify` as top-level
// exception) and the command-ownership table in arc42 chapter 5, which is
// the normative command map. Every command is a stub until its story is
// implemented test-first (see AGENTS.md, "Test-driven implementation").

mod config;
mod diag;
mod linter;
mod parser;
mod rewriter;
mod store;
mod trace;
mod verifier;

use clap::{Parser, Subcommand, ValueEnum};
use std::process::ExitCode;

/// Output format for diagnostics and command results (REQ-04-01-10-01).
#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
}

#[derive(Parser)]
#[command(
    name = "arqix",
    version,
    about = "Deterministic documentation-as-code toolchain"
)]
struct Cli {
    /// Output format for diagnostics and results
    #[arg(long, global = true, value_enum, default_value_t = OutputFormat::Text)]
    format: OutputFormat,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Configuration: validate and show the effective configuration
    Config {
        #[command(subcommand)]
        command: ConfigCommand,
    },
    /// Documents: scaffold, create, list, read, search
    Doc {
        #[command(subcommand)]
        command: DocCommand,
    },
    /// Units: create architecture units
    Unit {
        #[command(subcommand)]
        command: UnitCommand,
    },
    /// Rewrite documents into canonical form (mechanical only)
    Fmt {
        /// Report unformatted documents without writing (exit 1 if any)
        #[arg(long)]
        check: bool,
    },
    /// Update mechanical metadata such as `updated` (never body text)
    Finalise {
        /// The date to stamp as `updated` (injected, never the wall clock)
        #[arg(long)]
        date: String,
    },
    /// Lint: contract checks over the corpus
    Lint {
        #[command(subcommand)]
        command: LintCommand,
    },
    /// Assemble: expand chapter/include directives into pages
    Assemble {
        #[command(subcommand)]
        command: AssembleCommand,
    },
    /// Traceability: scan markers, check links, project coverage and matrices
    Trace {
        #[command(subcommand)]
        command: TraceCommand,
    },
    /// Export products for audit and compliance
    Report {
        #[command(subcommand)]
        command: ReportCommand,
    },
    /// Publish rendered outputs
    Publish {
        #[command(subcommand)]
        command: PublishCommand,
    },
    /// Render single-artefact outputs
    Render {
        #[command(subcommand)]
        command: RenderCommand,
    },
    /// Policy: evaluate changes against the declared change scope
    Policy {
        #[command(subcommand)]
        command: PolicyCommand,
    },
    /// Run the configured verification loop (format, lint, trace scan, coverage)
    Verify {
        /// Stop at the first failing sub-step
        #[arg(long)]
        fail_fast: bool,
        /// Run every sub-step and aggregate results (default)
        #[arg(long)]
        aggregate: bool,
    },
    /// Model Context Protocol server
    Mcp {
        #[command(subcommand)]
        command: McpCommand,
    },
}

#[derive(Subcommand)]
enum ConfigCommand {
    /// Validate arqix.toml against the configuration contract
    Validate,
    /// Render the effective configuration commands act on
    Show,
}

#[derive(Subcommand)]
enum DocCommand {
    /// Scaffold a documentation package in the current repository
    Init,
    /// Create a document of the given kind from its configured template
    New { kind: String },
    /// List documents as a machine-readable catalog
    List {
        /// Filter the catalog by document kind
        #[arg(long)]
        kind: Option<String>,
    },
    /// Read a document, or a section of it, as structured output
    Read { id: String },
    /// Search documents
    Search { query: String },
}

#[derive(Subcommand)]
enum UnitCommand {
    /// Create an architecture unit from its configured template
    New,
}

#[derive(Subcommand)]
enum LintCommand {
    /// Run the configured lint checks
    Run,
}

#[derive(Subcommand)]
enum AssembleCommand {
    /// Build pages from chapter/include directives
    Build,
}

#[derive(Subcommand)]
enum TraceCommand {
    /// Scan code, tests, and documents for trace markers
    Scan,
    /// Check marker links for a requirement
    Check { requirement: String },
    /// Report requirements coverage
    Coverage,
    /// Project traceability matrices
    Matrix {
        /// Matrix type: req-test (default) or us-req
        #[arg(long = "type", default_value = "req-test")]
        matrix_type: String,
    },
}

#[derive(Subcommand)]
enum ReportCommand {
    /// Produce an evidence bundle
    Bundle,
}

#[derive(Subcommand)]
enum PublishCommand {
    /// Publish the documentation site
    Site {
        /// Language to publish
        #[arg(long)]
        lang: Option<String>,
    },
}

#[derive(Subcommand)]
enum RenderCommand {
    /// Render a PDF artefact
    Pdf,
}

#[derive(Subcommand)]
enum PolicyCommand {
    /// Evaluate changed files against the declared change scope
    Check,
}

#[derive(Subcommand)]
enum McpCommand {
    /// Serve search/read/list over stdio
    Serve,
}

/// Exit code for commands whose story is not yet implemented. Deliberately
/// outside the stable 0/1/2 contract (REQ-00-00-00-02) so nothing can
/// mistake a stub for a real result.
const EXIT_UNIMPLEMENTED: u8 = 70;

fn unimplemented(command: &str) -> ExitCode {
    eprintln!("error: `arqix {command}` is not implemented yet");
    ExitCode::from(EXIT_UNIMPLEMENTED)
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Command::Config { command } => match command {
            ConfigCommand::Validate => config::validate(cli.format),
            ConfigCommand::Show => config::show(cli.format),
        },
        Command::Doc { command } => match command {
            DocCommand::Init => unimplemented("doc init"),
            DocCommand::New { .. } => unimplemented("doc new"),
            DocCommand::List { kind } => store::list(kind.as_deref(), cli.format),
            DocCommand::Read { id } => store::read(&id, cli.format),
            DocCommand::Search { query } => store::search(&query, cli.format),
        },
        Command::Unit { command } => match command {
            UnitCommand::New => unimplemented("unit new"),
        },
        Command::Fmt { check } => rewriter::fmt(check, cli.format),
        Command::Finalise { date } => rewriter::finalise(&date, cli.format),
        Command::Lint { command } => match command {
            LintCommand::Run => linter::run(cli.format),
        },
        Command::Assemble { command } => match command {
            AssembleCommand::Build => unimplemented("assemble build"),
        },
        Command::Trace { command } => match command {
            TraceCommand::Scan => trace::scan(cli.format),
            TraceCommand::Check { requirement } => trace::check_command(&requirement, cli.format),
            TraceCommand::Coverage => trace::coverage_command(cli.format),
            TraceCommand::Matrix { matrix_type } => trace::matrix_command(&matrix_type, cli.format),
        },
        Command::Report { command } => match command {
            ReportCommand::Bundle => unimplemented("report bundle"),
        },
        Command::Publish { command } => match command {
            PublishCommand::Site { .. } => unimplemented("publish site"),
        },
        Command::Render { command } => match command {
            RenderCommand::Pdf => unimplemented("render pdf"),
        },
        Command::Policy { command } => match command {
            PolicyCommand::Check => unimplemented("policy check"),
        },
        Command::Verify {
            fail_fast,
            aggregate: _,
        } => verifier::verify(fail_fast, cli.format),
        Command::Mcp { command } => match command {
            McpCommand::Serve => unimplemented("mcp serve"),
        },
    }
}
