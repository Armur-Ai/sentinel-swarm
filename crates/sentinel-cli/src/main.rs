//! `sentinel` — the Sentinel Swarm command-line interface.
//!
//! This is an early, honest scaffold. Commands that are not yet functional say
//! so explicitly rather than pretending to work; `doctor` and `init` do real
//! work today. See `ROADMAP.md` for what is planned.

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name = "sentinel",
    version,
    about = "Sentinel Swarm — Autonomous Binary Research Through Coordinated Intelligence",
    long_about = "A Rust-native, model-agnostic research platform that coordinates \
heterogeneous swarms of specialized agents to investigate compiled software for \
AUTHORIZED vulnerability research. Deterministic mode runs without any AI model."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Mode {
    /// No LLM: deterministic workers only.
    Deterministic,
    /// Deterministic analysis with AI assisting interpretation/prioritization.
    Hybrid,
    /// AI plans and coordinates bounded investigations under enforced policy.
    Autonomous,
}

#[derive(Subcommand)]
enum Command {
    /// Create a local Sentinel workspace (config + data directories).
    Init,
    /// Report platform health and which optional analysis tools are available.
    Doctor,
    /// Inspect a binary's format, architecture, and basic metadata.
    Inspect { binary: PathBuf },
    /// Run a research investigation against an authorized target binary.
    Research {
        binary: PathBuf,
        #[arg(long, value_enum, default_value_t = Mode::Deterministic)]
        mode: Mode,
    },
    /// List research jobs.
    Jobs,
    /// List validated findings.
    Findings,
    /// Show the evidence trail behind a finding.
    Evidence { finding_id: String },
    /// Render a report for a completed job.
    Report { job_id: String },
    /// Run the reproducible benchmark suite.
    Benchmark,
}

/// A tool the platform can use. Optional/proprietary tools must degrade
/// gracefully and are never required for deterministic mode.
struct Tool {
    name: &'static str,
    stage: &'static str,
    optional: bool,
}

const TOOLS: &[Tool] = &[
    Tool { name: "rizin", stage: "reverse-engineering", optional: false },
    Tool { name: "ghidra", stage: "reverse-engineering", optional: false },
    Tool { name: "binaryninja", stage: "reverse-engineering", optional: true },
    Tool { name: "angr", stage: "program-reasoning", optional: false },
    Tool { name: "z3", stage: "program-reasoning", optional: false },
    Tool { name: "afl-fuzz", stage: "fuzzing", optional: false },
    Tool { name: "honggfuzz", stage: "fuzzing", optional: true },
    Tool { name: "gdb", stage: "dynamic-analysis", optional: false },
    Tool { name: "frida", stage: "dynamic-analysis", optional: true },
    Tool { name: "valgrind", stage: "crash-triage", optional: true },
    Tool { name: "bindiff", stage: "patch-analysis", optional: true },
];

/// Returns true if `name` resolves to an executable on PATH. Pure std so the
/// CLI has no runtime dependency for this check.
fn on_path(name: &str) -> bool {
    let Some(paths) = std::env::var_os("PATH") else {
        return false;
    };
    std::env::split_paths(&paths).any(|dir| {
        let candidate = dir.join(name);
        candidate.is_file()
    })
}

fn config_dir() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".sentinel")
}

fn doctor() {
    println!("Sentinel Swarm {}", env!("CARGO_PKG_VERSION"));
    println!("core crate: {} v{}", sentinel_core::CRATE, sentinel_core::VERSION);
    println!();
    println!("Optional analysis tools (deterministic mode needs none of these):");
    let mut available = 0;
    for t in TOOLS {
        let ok = on_path(t.name);
        if ok {
            available += 1;
        }
        let status = if ok { "available" } else { "missing  " };
        let kind = if t.optional { "optional" } else { "recommended" };
        println!("  [{status}] {:<14} {:<20} ({kind})", t.name, t.stage);
    }
    println!();
    println!("{available}/{} tools available.", TOOLS.len());
    println!("Missing tools are reported, never silently skipped. Proprietary tools");
    println!("(Binary Ninja, WinDbg, BinDiff) remain optional and must be licensed by you.");
}

fn init() {
    let dir = config_dir();
    match std::fs::create_dir_all(dir.join("artifacts")) {
        Ok(_) => {
            println!("Initialized Sentinel workspace at {}", dir.display());
            println!("  - config:    {}", dir.join("config.toml").display());
            println!("  - artifacts: {}", dir.join("artifacts").display());
            println!("Run `sentinel doctor` to check tool availability.");
        }
        Err(e) => eprintln!("failed to initialize workspace at {}: {e}", dir.display()),
    }
}

/// Honest placeholder for commands whose backend is not yet wired.
fn not_yet(cmd: &str) {
    eprintln!("`sentinel {cmd}` is planned but not yet implemented.");
    eprintln!("See the roadmap: https://github.com/Armur-Ai/sentinel-swarm/blob/main/ROADMAP.md");
    std::process::exit(2);
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();
    match cli.command {
        Command::Init => init(),
        Command::Doctor => doctor(),
        Command::Inspect { .. } => not_yet("inspect"),
        Command::Research { .. } => not_yet("research"),
        Command::Jobs => not_yet("jobs"),
        Command::Findings => not_yet("findings"),
        Command::Evidence { .. } => not_yet("evidence"),
        Command::Report { .. } => not_yet("report"),
        Command::Benchmark => not_yet("benchmark"),
    }
}
