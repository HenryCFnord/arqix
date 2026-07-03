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
                configResolver = component "Config Resolver" "Loads arqix.toml, applies defaults and overrides, renders effective config (REQ-01-01-16-*)."
                docStore = component "Document Store & Catalog" "Discovers documents, derives IDs/slugs (REQ-00-00-00-04), emits the JSON catalog (REQ-05-01-08-*)."
                templateEngine = component "Template Engine" "Instantiates configured templates per kind with placeholder substitution (REQ-00-00-00-05, REQ-01-01-05-*)."
                formatter = component "Formatter" "Canonical frontmatter key order and directive normalisation (REQ-01-01-03-*)."
                linter = component "Linter" "Include/metadata/ID checks, schema contracts, i18n profile (REQ-01-01-04-*, REQ-01-01-10-*, REQ-00-00-00-10)."
                assembler = component "Assembler" "Parses chapter/include directives, expands globs, detects cycles, writes pages and the JSONL log (REQ-02-01-09-*, REQ-02-01-11-*, REQ-04-01-01-*)."
                traceEngine = component "Trace Engine" "Scans markers, builds the trace graph, matrices, and coverage (REQ-03-01-05-*, REQ-03-01-02-*, REQ-01-01-08-*)."
                reporter = component "Report & Export" "Audit-oriented exports with stable schemas (REQ-04-01-12-*), evidence bundles (REQ-03-01-04-*)."
                publisher = component "Publish & Render Orchestrator" "Drives Pandoc and site toolchains per language (REQ-04-01-03-*, REQ-04-01-07-*)."
                policyChecker = component "Policy Checker" "Evaluates changed files against declared change scope (REQ-01-01-07-*, REQ-00-00-00-07)."
                mcpServer = component "MCP Server" "Exposes search/read/list over stdio; transport separated from tool logic (REQ-05-01-12-*)."
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

        configResolver -> docStore "Provides effective configuration"
        docStore -> templateEngine "Supplies IDs, slugs, and target paths"
        assembler -> docStore "Resolves include targets"
        linter -> docStore "Validates documents against contracts"
        traceEngine -> docStore "Reads frontmatter links"
        publisher -> assembler "Publishes assembled pages"
        mcpServer -> docStore "Serves search/read/list"
        formatter -> docStore "Rewrites documents canonically"
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
        component cli "Components" "Subsystems of the arqix binary, cut along the requirement clusters." {
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
