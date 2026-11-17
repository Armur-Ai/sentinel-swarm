# Security Policy & Responsible Use

Sentinel Swarm is a platform for **authorized** vulnerability research, reproducible security testing, and defensive analysis. This document covers both how to report issues in the platform and the boundaries of acceptable use.

## Responsible use

Use Sentinel Swarm only against binaries you **own** or are **explicitly authorized** to analyze.

The platform is designed to make misuse hard and auditable:

- **Explicit target authorization** and scope enforcement — scope is never silently expanded.
- **Isolated execution** of unknown native binaries in disposable environments; unknown binaries are never executed on the orchestration host.
- **Capability-based authorization** — workers can only use the tools and resources they were granted, and cannot modify their own security policy.
- **Untrusted content** — decompiled strings, binary metadata, comments, and tool output are treated as data, never as instructions to an agent.
- **No remote egress of private artifacts** — binaries, source, credentials, and proprietary artifacts are not sent to remote model providers without explicit authorization.

**Explicitly out of scope, by design:** unrestricted exploit deployment, stealth, persistence, credential theft, and unauthorized access workflows. Contributions that add these capabilities will not be accepted.

## Reporting a vulnerability in Sentinel Swarm

If you discover a security issue **in Sentinel Swarm itself** (for example, a sandbox escape, a policy-enforcement bypass, or a path by which untrusted binary content could influence agent control flow), please report it privately rather than opening a public issue:

- Use GitHub's **private vulnerability reporting** on this repository (Security → Report a vulnerability), or
- Open a minimal issue asking for a private contact and we will follow up.

Please include a clear description, affected version/commit, and reproduction steps against an authorized target. We will acknowledge, investigate, and coordinate disclosure.

## Scope of this policy

This policy concerns the security of the Sentinel Swarm codebase and its isolation guarantees. Findings produced *by* the platform about third-party software are the responsibility of the operator and must follow the affected project's own coordinated-disclosure process.
