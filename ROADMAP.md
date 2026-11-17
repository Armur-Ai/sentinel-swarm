# Sentinel Swarm — Roadmap

This is the **public, surface-level** roadmap. It states the phases and what each one delivers, not the task-by-task breakdown. It is intentionally high-level; the detailed engineering plan is maintained privately.

**Honesty policy:** nothing is described as working before it is. Each phase lists a goal and an "exit" — a result a stranger can reproduce. Advanced capabilities are **goals until implemented and benchmarked.**

## Where we are today

Early scaffold. The repository contains the Rust workspace and eleven crate boundaries, the CLI skeleton (`init` and `doctor` do real work; other commands honestly report they are not yet implemented), the architecture diagram, and the engineering plan. No analysis integration is wired yet — that begins in Phase 1.

## Guiding principles

- **Deterministic first.** Every capability must work with no AI model before an AI mode is added.
- **Evidence over chatter.** The typed evidence graph is the source of truth, not agent conversations.
- **Reproducibility is the product.** A finding without provenance is not a finding.
- **Authorized targets only.** Scope is enforced, never silently expanded; isolation is real, not process cleanup.

## Phases

### Phase 0 — Repository foundation
Rust workspace, CLI shell, configuration, logging, the tool registry, the policy framework, and test infrastructure.
**Exit:** `cargo build` is clean; `sentinel doctor` reports tool availability honestly; CI runs fmt, clippy, build, and test.

### Phase 1 — Functional deterministic research
The tool-adapter framework (graceful degradation when a tool is absent), the evidence-graph foundation (typed nodes/edges with full provenance, SQLite + content-addressed artifacts), the first reverse-engineering integration, a first dynamic-analysis integration, and a minimal Research Director that runs the smallest end-to-end deterministic workflow.
**Exit:** a deliberately vulnerable, authorized test binary is analyzed end to end, producing an evidence trail — with no AI model.

### Phase 2 — Micro-swarm runtime
The reusable swarm runtime: agent/subagent lifecycle, dynamic spawning, parent–child relationships, concurrent scheduling, task cancellation, work deduplication, structured handoffs, and backward investigation. The policy and isolation layer becomes real.
**Exit:** multiple deterministic workers run concurrently inside a stage, hand off through immutable evidence, and recover from a worker failure.

### Phase 3 — Discovery and validation
Fuzzing campaigns in isolated environments, sanitizer result parsing, crash deduplication, reproducer storage, and the candidate→validated finding gate.
**Exit:** a fuzzing campaign produces a reproducible crash that is triaged into a confidence-scored finding with artifacts.

### Phase 4 — Program reasoning and patch analysis
Bounded symbolic analysis and constraint workflows (solver-proven results kept distinct from hypotheses), and authorized binary-version diffing with regression validation.
**Exit:** a reachability/constraint question is answered with provenance; a known fix is shown to change behavior across versions.

### Phase 5 — AI intelligence
The model-agnostic provider layer (local and OpenAI-compatible), separated model roles (planning, hypothesis, interpretation, critique, report synthesis), hybrid and autonomous modes where the model proposes and the controller validates before execution, and egress/injection safeguards.
**Exit:** an AI-assisted investigation runs under enforced policy; model conclusions are labeled as hypotheses, never as verified evidence.

### Phase 6 — Additional tools and platforms
Broader tool coverage (Binary Ninja, Rizin, Triton, Z3, libFuzzer, honggfuzz, Frida, WinDbg) and Windows support, with separate execution backends.
**Exit:** the supported-tool matrix is generated from the registry, and each listed tool has a test that proves it runs.

### Phase 7 — Documentation and release
Architecture diagrams, the full README, published benchmarks with their method, security documentation, the contribution guide, and release packaging.
**Exit:** a tagged release a newcomer can install and use for a deterministic investigation.

## Benchmarks

Published only when they exist, with their method, comparing — at equal resource budgets — a deterministic toolchain, a single AI agent, uncoordinated concurrent agents, and Sentinel Swarm's evidence-driven coordination. **No invented results.**

## Get involved

This is a research platform, and collaboration is welcome — issues, ideas, tool adapters, and reproducible test targets especially. See [CONTRIBUTING.md](CONTRIBUTING.md).
