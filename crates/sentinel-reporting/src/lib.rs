//! # sentinel-reporting
//!
//! Evidence-backed report synthesis across formats.
//!
//! Part of the **Sentinel Swarm** research platform. This crate is an early
//! scaffold: its public surface will grow as the implementation plan lands.
//! See the workspace `README.md` and `ROADMAP.md`.

/// Crate version, surfaced by `sentinel doctor` for diagnostics.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Human-readable crate identity.
pub const CRATE: &str = "sentinel-reporting";
