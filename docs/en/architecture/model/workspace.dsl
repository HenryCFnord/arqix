// arqix C4 model — single source of truth for architecture views (ADR-0002).
// Views embedded in documentation are Mermaid diagrams derived from this
// workspace; regenerate or update them together with this file.

workspace "arqix" "Documentation-as-code toolchain with traceability" {

    model {
        maintainer = person "Documentation Maintainer" "Owns standards, templates, schemas, and governance (persona Mara)."
        developer = person "Developer" "Writes code and documentation in the same flow (personas Dan, Quinn, Aria)."
        agent = person "Coding Agent" "Executes story-by-story tasks against deterministic contracts (persona Casey)."

        arqix = softwareSystem "arqix CLI" "Deterministic documentation-as-code toolchain: create, format, lint, assemble, trace, report, publish." {
            cli = container "arqix binary" "Single Rust CLI; all commands resolve against the effective configuration (REQ-00-00-00-06)." "Rust" {
                cliEntry = component "CLI Entrypoint & Dispatch" "Argument parsing and subcommand routing; composition root wiring config resolution, feature components, and the diagnostics/exit-code contract (REQ-00-00-00-02/03/06)."
                docParser = component "Document Parser" "Single deterministic parse: lossless concrete syntax for rewriting plus the semantic document model — frontmatter, sections/anchors, directives, trace markers (REQ-02-01-09-*, REQ-05-01-10-*, REQ-01-01-03-03)."
                configResolver = component "Config Resolver" "Loads arqix.toml, applies defaults and overrides, renders effective config (REQ-01-01-16-*)."
                docStore = component "Document Store & Catalog" "Discovers documents, derives IDs/slugs (REQ-00-00-00-04), emits the JSON catalog (REQ-05-01-08-*)."
                templateEngine = component "Template Engine" "Instantiates configured templates per kind with placeholder substitution (REQ-00-00-00-05, REQ-01-01-05-*)."
                formatter = component "Formatter & Finaliser" "Canonical rewrites over the parser CST: fmt (key order, directive normalisation, REQ-01-01-03-*) and finalise (mechanical metadata updates with injected clock, REQ-01-01-06-*). The only component that mutates existing source documents (REQ-00-00-00-08, ADR-0004)."
                linter = component "Linter" "Include/metadata/ID checks, schema contracts, i18n profile (REQ-01-01-04-*, REQ-01-01-10-*, REQ-00-00-00-10)."
                assembler = component "Assembler" "Parses chapter/include directives, expands globs, detects cycles, writes pages and the JSONL log (REQ-02-01-09-*, REQ-02-01-11-*, REQ-04-01-01-*)."
                traceEngine = component "Trace Engine" "Scans markers, builds the trace graph, matrices, and coverage (REQ-03-01-05-*, REQ-03-01-02-*, REQ-01-01-08-*)."
                reporter = component "Report & Export" "Audit-oriented exports with stable schemas (REQ-04-01-12-*), evidence bundles (REQ-03-01-04-*)."
                publisher = component "Publish & Render Orchestrator" "Drives Pandoc and site toolchains per language (REQ-04-01-03-*, REQ-04-01-07-*)."
                policyChecker = component "Policy Checker" "Evaluates changed files against declared change scope (REQ-01-01-07-*, REQ-00-00-00-07)."
                mcpServer = component "MCP Server" "Exposes search/read/list over stdio; transport separated from tool logic (REQ-05-01-12-*)."
                verifier = component "Verification Orchestrator" "Sequences the configured verify sub-steps (format, lint, trace scan, coverage) via the stable command interface; fail-fast/aggregate modes and per-step JSON results; never implements a check itself (REQ-04-01-05-*, ADR-0003)."
                diagnostics = component "Diagnostics & Exit Codes" "Machine-readable diagnostics and the 0/1/2 exit-code contract (REQ-00-00-00-02/03, REQ-04-01-08-*, REQ-04-01-10-*)."
            }
            config = container "arqix.toml" "Repository configuration: kinds, templates, roots, policies, i18n." "TOML"
            corpus = container "Documentation Corpus" "Markdown documents with frontmatter, ontology vocabulary, trace markers." "Markdown + YAML"
        }

        renderToolchain = softwareSystem "Render Toolchain" "Pandoc (PDF) and Zensical (site) invoked by arqix; errors forwarded transparently." "External"
        gitRepo = softwareSystem "Git Repository" "Source of truth for corpus, configuration, and code; provides changed-file lists for policy checks." "External"
        ciPages = softwareSystem "CI & Pages" "Runs the verification loop as a gate and publishes language-aware sites." "External"
        mcpClient = softwareSystem "MCP Client" "Agent or IDE consuming documentation over the Model Context Protocol." "External"

        maintainer -> arqix "Defines standards, runs fmt/lint, reviews diagnostics"
        developer -> arqix "Creates and searches documents alongside implementation"
        agent -> arqix "Runs deterministic loops: doc new, verify, trace check"
        arqix -> gitRepo "Reads and writes documents within configured roots (REQ-00-00-00-13)"
        arqix -> renderToolchain "Invokes for PDF and site rendering; never executes document content (REQ-00-00-00-14)"
        ciPages -> arqix "Runs verify with stable exit codes (REQ-00-00-00-02)"
        mcpClient -> arqix "Calls search/read/list over stdio"

        cli -> config "Resolves at startup into the effective configuration"
        cli -> corpus "Reads, creates, formats, and assembles documents"

        cliEntry -> configResolver "Resolves the effective configuration before any command runs"
        cliEntry -> diagnostics "Maps command results to diagnostics and exit codes"
        cliEntry -> docStore "Routes doc list/read/search"
        cliEntry -> templateEngine "Routes doc new"
        cliEntry -> formatter "Routes fmt and finalise"
        cliEntry -> linter "Routes lint"
        cliEntry -> assembler "Routes assemble"
        cliEntry -> traceEngine "Routes trace"
        cliEntry -> reporter "Routes report bundle"
        cliEntry -> publisher "Routes publish and render"
        cliEntry -> verifier "Routes verify"
        cliEntry -> policyChecker "Routes policy check"
        cliEntry -> mcpServer "Routes mcp serve"
        verifier -> formatter "Runs the format sub-step in check mode"
        verifier -> linter "Runs the lint sub-step"
        verifier -> traceEngine "Runs the trace scan and coverage sub-steps"
        verifier -> diagnostics "Aggregates per-step results and diagnostic references"
        docStore -> docParser "Parses documents into the semantic model"
        formatter -> docParser "Rewrites via the lossless concrete syntax"
        linter -> docParser "Validates the parsed semantic model"
        assembler -> docParser "Reads directives from the parsed model"
        traceEngine -> docParser "Reads markers and frontmatter links"
        templateEngine -> docStore "Obtains IDs, slugs, and target paths"
        assembler -> docStore "Resolves include targets"
        linter -> docStore "Validates documents against contracts"
        traceEngine -> docStore "Reads frontmatter links"
        publisher -> assembler "Publishes assembled pages"
        mcpServer -> docStore "Serves search/read/list"
        reporter -> traceEngine "Exports graph, matrices, coverage"
        policyChecker -> configResolver "Reads declared policy"
    }

    views {
        systemContext arqix "SystemContext" "Who uses arqix and which systems it touches." {
            include *
            autoLayout lr
        }
        container arqix "Containers" "The arqix binary, its configuration, and the corpus it operates on." {
            include *
            autoLayout lr
        }
        component cli "Components" "Subsystems of the arqix binary: the entrypoint as composition root, the parser as shared reading layer, the verification orchestrator sequencing the quality gate, and the feature components cut along the requirement clusters." {
            include *
            autoLayout tb
        }
        styles {
            element "External" {
                background #999999
                color #ffffff
            }
            element "Person" {
                shape person
            }
        }
    }
}
