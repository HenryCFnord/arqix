// arqix C4 model — single source of truth for architecture views (ADR-0002).
// Views embedded in documentation are Mermaid diagrams derived from this
// workspace; regenerate or update them together with this file.

workspace "arqix" "Documentation-as-code toolchain with traceability" {

    model {
        maintainer = person "Documentation Maintainer" "Owns standards, templates, schemas, and governance (persona Mara, PER-01)."
        builder = person "Builder" "Changes the corpus alongside code, pipelines, and architecture (persona Bernadette, PER-09)."
        assessor = person "Assessor" "Consumes the corpus as input: coverage, evidence bundles, the catalogue, and the site (persona Adrian, PER-10)."
        agent = person "Coding Agent" "Executes story-by-story tasks against deterministic contracts (persona Casey, PER-08)."

        arqix = softwareSystem "arqix CLI" "Deterministic documentation-as-code toolchain: create, format, lint, assemble, trace, report, publish." {
            cli = container "arqix binary" "Single Rust CLI; all commands resolve against the effective configuration (REQ-00-00-00-06)." "Rust" {
                cliEntry = component "CLI Entrypoint & Dispatch" "Argument parsing and subcommand routing; composition root wiring config resolution, feature components, and the diagnostics/exit-code contract (REQ-00-00-00-02/03/06)."
                docParser = component "Document Parser" "Single deterministic parse of a document's YAML frontmatter into the semantic model — id, iri, title, classes, triples, language — retaining the raw frontmatter lines and body for the mechanical rewriter (Rust port of the Python oracle's parser). Body directives and trace markers are extracted downstream by the Assembler and Trace Engine (REQ-05-01-10-*, REQ-01-01-03-03)."
                configResolver = component "Config Resolver" "Loads arqix.toml, applies defaults and overrides, renders effective config (REQ-01-01-16-*)."
                docStore = component "Document Store & Catalog" "Discovers documents over the configured roots, emits the JSON catalog reading declared IDs, and backs doc list/read/search (REQ-05-01-08-*, REQ-05-01-10-*, REQ-02-01-06-*)."
                templateEngine = component "Template Engine" "Instantiates configured templates per kind with placeholder substitution (REQ-00-00-00-05, REQ-01-01-05-*)."
                formatter = component "Formatter & Finaliser" "Canonical rewrites over the parser CST: fmt (key order, directive normalisation, REQ-01-01-03-*) and finalise (mechanical metadata updates with injected clock, REQ-01-01-06-*). The only component that mutates existing source documents (REQ-00-00-00-08, ADR-0004)."
                linter = component "Linter" "Include, reference, ID-policy, lifecycle, and done-claim checks plus the translation-source check (REQ-01-01-04-*, REQ-01-01-18-*, REQ-03-01-09-*, REQ-00-00-00-10)."
                assembler = component "Assembler" "Expands include directives with declared heading levels (ADR-0013), rebases fragment links, detects cycles, enforces containment, writes pages and the JSONL log (REQ-02-01-09-*, REQ-02-01-11-*, REQ-02-01-12-*, REQ-04-01-01-*)."
                traceEngine = component "Trace Engine" "Scans verifies/implements/plans markers, builds the trace graph, matrices, coverage with joined test outcomes, and the regression ratchet (REQ-03-01-05-*, REQ-03-01-02-*, REQ-01-01-08-*, REQ-03-01-10-*, REQ-04-01-15/16-*)."
                reporter = component "Report & Export" "Audit-oriented exports with stable schemas (REQ-04-01-12-*): evidence bundles (REQ-03-01-04-*) and the OKF knowledge bundle (REQ-05-01-15-*)."
                publisher = component "Publish & Render Orchestrator" "Stages artefact-ready pages and the specification catalogue, drives Pandoc and site toolchains per language (REQ-04-01-03-*, REQ-04-01-07-*, REQ-04-01-17-*)."
                policyChecker = component "Policy Checker" "Evaluates changed files against declared change scope (REQ-01-01-07-*, REQ-00-00-00-07)."
                mcpServer = component "MCP Server" "Exposes search/read/list/trace with filters over stdio (hand-rolled JSON-RPC subset, ADR-0014); transport separated from tool logic (REQ-05-01-12-*)."
                verifier = component "Verification Orchestrator" "Sequences the configured verify sub-steps (format, lint, trace scan, coverage, ratchet — REQ-04-01-14-*) via the stable command interface; fail-fast/aggregate modes and per-step JSON results; never implements a check itself (REQ-04-01-05-*, ADR-0003)."
                diagnostics = component "Diagnostics & Exit Codes" "Machine-readable diagnostics and the 0/1/2 exit-code contract (REQ-00-00-00-02/03, REQ-04-01-08-*, REQ-04-01-10-*)."
            }
            config = container "arqix.toml" "Repository configuration: kinds, templates, roots, policies, i18n." "TOML"
            corpus = container "Documentation Corpus" "Markdown documents with frontmatter, ontology vocabulary, trace markers." "Markdown + YAML"
        }

        renderToolchain = softwareSystem "Render Toolchain" "Configured external renderers invoked by arqix: Pandoc by default for PDF, a configured site command (e.g. Zensical or MkDocs); errors forwarded transparently." "External"
        gitRepo = softwareSystem "Git Repository" "Source of truth for corpus, configuration, and code; provides changed-file lists for policy checks." "External"
        ciPages = softwareSystem "CI & Pages" "Runs the verification loop as a gate and publishes language-aware sites." "External"
        mcpClient = softwareSystem "MCP Client" "Agent or IDE consuming documentation over the Model Context Protocol." "External"

        maintainer -> arqix "Defines standards, runs fmt/lint, reviews diagnostics"
        builder -> arqix "Creates and searches documents alongside implementation"
        assessor -> arqix "Reads coverage, evidence bundles, and the published specification"
        agent -> arqix "Runs deterministic loops: doc new, verify, trace check"
        arqix -> gitRepo "Reads and writes documents within configured roots (REQ-00-00-00-13)"
        arqix -> renderToolchain "Invokes for PDF and site rendering; never executes document content (REQ-00-00-00-14)"
        ciPages -> arqix "Runs verify with stable exit codes (REQ-00-00-00-02)"
        mcpClient -> arqix "Calls search/read/list/trace over stdio"

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
        cliEntry -> reporter "Routes report bundle and report knowledge"
        cliEntry -> publisher "Routes publish and render"
        cliEntry -> verifier "Routes verify"
        cliEntry -> policyChecker "Routes policy check"
        cliEntry -> mcpServer "Routes mcp serve"
        verifier -> formatter "Runs the format sub-step in check mode"
        verifier -> linter "Runs the lint sub-step"
        verifier -> traceEngine "Runs the trace scan, coverage, and ratchet sub-steps"
        verifier -> diagnostics "Aggregates per-step results and diagnostic references"
        docStore -> docParser "Parses documents into the semantic model"
        formatter -> docStore "Iterates the corpus to format and finalise every document"
        linter -> docParser "Validates the parsed semantic model"
        assembler -> docParser "Reads directives from the parsed model"
        traceEngine -> docParser "Reads markers and frontmatter links"
        templateEngine -> docStore "Obtains IDs, slugs, and target paths"
        assembler -> docStore "Resolves include targets"
        linter -> docStore "Validates documents against contracts"
        linter -> traceEngine "Resolves reference-target markers and reads verified-requirement IDs"
        assembler -> linter "Reuses the include-directive grammar"
        publisher -> assembler "Expands includes for staging and rendering"
        publisher -> docStore "Discovers the documents to stage"
        publisher -> traceEngine "Reads coverage status for the specification catalogue"
        mcpServer -> docStore "Serves search/read/list"
        mcpServer -> traceEngine "Serves the trace tool from the coverage rows"
        reporter -> traceEngine "Exports graph, matrices, coverage"
        reporter -> docStore "Discovers the documents for the knowledge bundle"
        reporter -> assembler "Expands includes for artefact-ready concepts"
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
