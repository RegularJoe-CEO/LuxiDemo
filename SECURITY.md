# SECURITY.md

## Reporting Vulnerabilities
If you discover a security vulnerability in eRock, please report it **privately**:
- Email: e@ewaller.com
- Include a minimal repro, version/commit hash, and your environment.
Do not open a public GitHub issue for security concerns.

## Security Principles
- Minimal attack surface: static Rust binary; no shell-outs; no dynamic code loading.
- Secure communication: prefer Unix Domain Sockets (UDS) locally; support gRPC over TCP with TLS for remote.
- Least privilege by default: numeric expression evaluation only.
- Supply chain hygiene: audited dependencies and pinned versions via Cargo.lock.
- Reproducible builds: deterministic builds encouraged for integrity verification.
- No embedded secrets: credentials or keys must never be committed to the repo.

## Supported Versions
Security updates target the latest release and main. Older versions may not receive patches.

| Version | Supported |
|--------:|:---------:|
| main    |    ✅     |
| latest  |    ✅     |
| < latest|    ⚠️     |

## Secure Configuration Guidance
- Prefer UDS (/tmp/erock.sock) for single-host.
- For TCP: enforce TLS (mTLS recommended) and firewalling.
- Run as non-root with least privileges.
- Apply resource limits (ulimits/cgroups).
- Containers: distroless, drop caps, read-only root FS, seccomp/apparmor.

## Input Handling
- Numeric expressions only; avoid any APIs with file/process/network access.
- Validate/limit expression size, recursion depth, and evaluation time (DoS mitigation).
- Authenticate clients and rate-limit untrusted sources.

## Supply Chain & Build Integrity
- Use 'cargo audit' in CI.
- Pin versions in Cargo.lock and review diffs on update.
- Optional SLSA provenance for builds.
- Optional 'cargo vendor' for hermetic builds.

## Responsible Disclosure
We aim to acknowledge within 72 hours and provide a remediation timeline where feasible. Coordinated disclosure is appreciated.

## Security Roadmap
- CI: automatic cargo audit + dependency review gating.
- Optional FIPS-validated crypto when TLS is enabled (enterprise builds).
- Hardened container baselines for edge deployments.
- SOC 2 readiness artifacts for enterprise customers.
- Fuzzing/property-based tests for parser/evaluator components.
