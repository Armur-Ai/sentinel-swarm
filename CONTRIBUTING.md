# Contributing to Sentinel Swarm

Thank you for your interest — Sentinel Swarm is a research platform and collaboration is genuinely welcome, especially tool adapters, reproducible test targets, and research ideas.

## Ground rules

1. **Build real, testable components.** No mocks presented as working integrations; no fabricated tool output. If a tool isn't wired, the code says so.
2. **Keep every artifact attributable.** Observations carry their tool, version, configuration, binary hash, timestamp, and confidence. Don't add a code path that produces an un-provenanced result.
3. **Deterministic before AI.** A capability must work without a model before an AI mode is layered on. AI conclusions are hypotheses, never verified evidence.
4. **Authorized targets only.** Tests and examples use deliberately vulnerable or clearly-owned/authorized binaries. Nothing here targets systems you don't own or aren't authorized to test, and the project excludes exploit deployment, stealth, persistence, and credential theft.
5. **Optional tools stay optional.** Never make a proprietary tool (Binary Ninja, WinDbg, BinDiff) a hard requirement; degrade gracefully when a tool is missing.

## Development

```bash
cargo build --workspace
cargo test --workspace
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
```

CI runs fmt, clippy (warnings-as-errors), build, and test on every PR. Please keep it green.

## What makes a good contribution

- **A tool adapter** with: runtime availability detection, a deterministic invocation, structured output parsing, graceful handling when the tool is absent, and at least one test against an authorized fixture.
- **A test target** — a small, deliberately vulnerable or historically fixed/vulnerable open-source binary with a reproducible build.
- **Evidence-graph work** that preserves provenance and reproduction status.
- **Docs and diagrams** that explain the architecture accurately.

## Submitting

Open an issue to discuss larger changes first. Keep PRs focused, describe the behavior change, and note what is tested vs. experimental. By contributing you agree your work is licensed under the project's [MIT License](LICENSE).

## Security

If you believe you've found a security issue in Sentinel Swarm itself, please follow [SECURITY.md](SECURITY.md) rather than opening a public issue.
