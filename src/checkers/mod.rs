//! Checkers: the ported reference consistency checkers (arc42 chapter 8,
//! oracle policy). Each checker family is a `lint <family>` subcommand ported
//! from a retired Python reference implementation (removed 2026-07-15 after
//! conformance; see git history). The families share
//! the faithful-port shape — a pure checker over the corpus that produces the
//! oracle's exact findings and exit codes — added as a sibling module here.

pub mod frontmatter;
pub mod requirements;
