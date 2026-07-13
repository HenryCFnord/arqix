// arqix - a CLI for structured technical documentation and Architecture-as-Code workflows.
//
// Command surface per ADR-0005 (noun-verb scheme, `verify` as top-level
// exception) and the command-ownership table in arc42 chapter 5, which is
// the normative command map. Every command is a stub until its story is
// implemented test-first (see AGENTS.md, "Test-driven implementation").

mod assembler;
mod checkers;
mod config;
mod diag;
mod linter;
mod mcp;
mod parser;
mod policy;
mod publisher;
mod reporter;
mod rewriter;
mod store;
mod templates;
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
    /// Assemble: expand include directives into pages
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
    /// Requirement shortcuts (`req new` = `doc new requirement`)
    Req {
        #[command(subcommand)]
        command: AliasNewCommand,
    },
    /// User-story shortcuts (`us new` = `doc new user-story`)
    Us {
        #[command(subcommand)]
        command: AliasNewCommand,
    },
    /// ADR shortcuts (`adr new` = `doc new adr`)
    Adr {
        #[command(subcommand)]
        command: AliasNewCommand,
    },
    /// Initialise the repository in one command (alias for `doc init`)
    Init {
        /// Package path (defaults to the first configured root)
        path: Option<String>,
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
    Init {
        /// Package path (defaults to the first configured root)
        path: Option<String>,
    },
    /// Create a document of the given kind from its configured template
    New {
        kind: String,
        /// Title substituted into the template (the slug derives from it)
        #[arg(long)]
        title: Option<String>,
        /// Explicit ID instead of the generated one (uniqueness still checked)
        #[arg(long)]
        id: Option<String>,
        /// Report the planned ID and target path without writing
        #[arg(long)]
        dry_run: bool,
    },
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
    ///
    /// The unit is created in the first configured root (default `docs/`),
    /// as a sibling of the package's `units/` content. Its ID is supplied
    /// automatically — the next free `unit-arc42-NN` minted from the
    /// corpus — and everything beyond the generated identity (title,
    /// triples, lifecycle metadata) is optional: fill it in afterwards and
    /// let `arqix fmt` keep the shape canonical.
    New,
}

#[derive(Subcommand)]
enum LintCommand {
    /// Run the configured lint checks
    Run,
    /// Validate requirement documents against the authoring rules
    /// (RFC 2119 subset + EARS patterns, ID/kind/metadata/derivation scheme)
    Requirements {
        /// Suppress REQ-LNK-006 warnings while derivation is incomplete
        #[arg(long)]
        allow_unlinked_stories: bool,
    },
    /// Validate frontmatter, canonical formatting, and ontology-vocabulary use
    /// across the architecture and ontology documents (FMT/FM/ONT rule families)
    Frontmatter {
        /// Suppress ONT-005 warnings for owl.inverse-of names that have no
        /// property document yet
        #[arg(long)]
        allow_undefined_inverse: bool,
    },
}

/// The `new` half of the creation aliases (REQ-01-01-05-02): `req new`,
/// `us new`, and `adr new` mirror `doc new <kind>` exactly.
#[derive(Subcommand)]
enum AliasNewCommand {
    /// Create the document from its configured template
    New {
        /// Title substituted into the template (the slug derives from it)
        #[arg(long)]
        title: Option<String>,
        /// Explicit ID instead of the generated one (uniqueness still checked)
        #[arg(long)]
        id: Option<String>,
        /// Plan the creation without writing the file
        #[arg(long)]
        dry_run: bool,
    },
}

#[derive(Subcommand)]
enum AssembleCommand {
    /// Build pages by expanding include directives
    Build,
}

#[derive(Subcommand)]
enum TraceCommand {
    /// Scan code, tests, and documents for trace markers
    Scan,
    /// Check marker links for a requirement
    Check { requirement: String },
    /// Report requirements coverage
    Coverage {
        /// JUnit XML results report; joined outcomes demote failed or
        /// skipped claims (US-03-01-10)
        #[arg(long)]
        results: Option<String>,
    },
    /// Project traceability matrices
    Matrix {
        /// Matrix type: req-test (default) or us-req
        #[arg(long = "type", default_value = "req-test")]
        matrix_type: String,
    },
    /// Fail when verified coverage decreases against the baseline
    Ratchet {
        /// Baseline matrix snapshot (default: the committed req-test matrix)
        #[arg(long)]
        baseline: Option<String>,
    },
    /// Report active markers gone stale against their target's version history
    Freshness,
    /// Gate test functions for trace markers (the ported TDD marker gate)
    Markers,
}

#[derive(Subcommand)]
enum ReportCommand {
    /// Produce an evidence bundle for the given requirement or story IDs
    Bundle {
        /// Requirement or story IDs; a story stands for the requirements
        /// derived from it
        #[arg(required = true)]
        ids: Vec<String>,
        /// Bundle output directory (default: bundle)
        #[arg(long)]
        out: Option<String>,
        /// Generation stamp recorded in the bundle metadata (e.g. "<sha>, <date>")
        #[arg(long)]
        stamp: Option<String>,
    },
    /// Export the corpus as an Open Knowledge Format bundle
    Knowledge {
        /// Bundle output directory (default: knowledge)
        #[arg(long)]
        out: Option<String>,
    },
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
    /// Render a PDF artefact via the configured renderer (Pandoc)
    Pdf {
        /// Selected Markdown files; the staged artefact-ready pages
        /// otherwise
        files: Vec<String>,
        /// Language root to render (the configured default otherwise)
        #[arg(long)]
        lang: Option<String>,
        /// Output path, overriding the configured artefact mode
        #[arg(long)]
        out: Option<String>,
    },
}

#[derive(Subcommand)]
enum PolicyCommand {
    /// Evaluate changed files against the declared change scope
    Check {
        /// Changed files to evaluate (e.g. from `git diff --name-only`)
        #[arg(required = true)]
        files: Vec<String>,
    },
}

#[derive(Subcommand)]
enum McpCommand {
    /// Serve search/read/list over stdio
    Serve,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Command::Config { command } => match command {
            ConfigCommand::Validate => config::validate(cli.format),
            ConfigCommand::Show => config::show(cli.format),
        },
        Command::Doc { command } => match command {
            DocCommand::Init { path } => templates::init(path.as_deref(), cli.format),
            DocCommand::New {
                kind,
                title,
                id,
                dry_run,
            } => templates::new_document(
                &kind,
                templates::NewOptions {
                    title: title.as_deref(),
                    id: id.as_deref(),
                    dry_run,
                },
                cli.format,
            ),
            DocCommand::List { kind } => store::list(kind.as_deref(), cli.format),
            DocCommand::Read { id } => store::read(&id, cli.format),
            DocCommand::Search { query } => store::search(&query, cli.format),
        },
        Command::Unit { command } => match command {
            UnitCommand::New => {
                templates::new_document("unit", templates::NewOptions::default(), cli.format)
            }
        },
        Command::Fmt { check } => rewriter::fmt(check, cli.format),
        Command::Finalise { date } => rewriter::finalise(&date, cli.format),
        Command::Lint { command } => match command {
            LintCommand::Run => linter::run(cli.format),
            LintCommand::Requirements {
                allow_unlinked_stories,
            } => checkers::requirements::lint(cli.format, allow_unlinked_stories),
            LintCommand::Frontmatter {
                allow_undefined_inverse,
            } => checkers::frontmatter::lint(cli.format, allow_undefined_inverse),
        },
        Command::Assemble { command } => match command {
            AssembleCommand::Build => assembler::build(cli.format),
        },
        Command::Trace { command } => match command {
            TraceCommand::Scan => trace::scan(cli.format),
            TraceCommand::Check { requirement } => trace::check_command(&requirement, cli.format),
            TraceCommand::Coverage { results } => {
                trace::coverage_command(results.as_deref(), cli.format)
            }
            TraceCommand::Matrix { matrix_type } => trace::matrix_command(&matrix_type, cli.format),
            TraceCommand::Ratchet { baseline } => {
                trace::ratchet_command(baseline.as_deref(), cli.format)
            }
            TraceCommand::Freshness => trace::freshness_command(cli.format),
            TraceCommand::Markers => trace::markers_command(cli.format),
        },
        Command::Report { command } => match command {
            ReportCommand::Bundle { ids, out, stamp } => {
                reporter::bundle(&ids, out.as_deref(), stamp.as_deref(), cli.format)
            }
            ReportCommand::Knowledge { out } => reporter::knowledge(out.as_deref(), cli.format),
        },
        Command::Publish { command } => match command {
            PublishCommand::Site { lang } => publisher::site(lang.as_deref(), cli.format),
        },
        Command::Render { command } => match command {
            RenderCommand::Pdf { files, lang, out } => {
                publisher::pdf(&files, lang.as_deref(), out.as_deref(), cli.format)
            }
        },
        Command::Policy { command } => match command {
            PolicyCommand::Check { files } => policy::check(&files, cli.format),
        },
        Command::Verify {
            fail_fast,
            aggregate: _,
        } => verifier::verify(fail_fast, cli.format),
        Command::Mcp { command } => match command {
            McpCommand::Serve => mcp::serve(),
        },
        // arqix:implements REQ-01-01-05-02
        Command::Req { command } => alias_new("requirement", command, cli.format),
        Command::Us { command } => alias_new("user-story", command, cli.format),
        Command::Adr { command } => alias_new("adr", command, cli.format),
        // arqix:implements REQ-01-01-01-03
        // The top-level init alias dispatches straight into `doc init` — one
        // code path, no second initialisation surface.
        Command::Init { path } => templates::init(path.as_deref(), cli.format),
    }
}

/// The creation aliases dispatch exactly like `doc new <kind>` — one code
/// path, no second creation surface (REQ-01-01-05-02).
fn alias_new(kind: &str, command: AliasNewCommand, format: OutputFormat) -> ExitCode {
    let AliasNewCommand::New { title, id, dry_run } = command;
    templates::new_document(
        kind,
        templates::NewOptions {
            title: title.as_deref(),
            id: id.as_deref(),
            dry_run,
        },
        format,
    )
}
