//! Checkers: the ported reference consistency checkers (arc42 chapter 8,
//! oracle policy). Each checker family is a `lint <family>` subcommand whose
//! Python reference implementation under `scripts/` stays as the conformance
//! oracle for the grace period (roadmap phase 5 item 9). The families share
//! the faithful-port shape — a pure checker over the corpus that produces the
//! oracle's exact findings and exit codes — so the next family (slice 3's
//! `lint frontmatter`) is added the same way, as a sibling module here.

pub mod requirements;
