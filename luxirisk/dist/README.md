# LuxiRisk v0.2 binaries

Closed-source evaluation binaries. Matching `.sha256` files ship beside each
binary. Prefer verifying the digest (and GitHub provenance attestation when
released via CI) before first run.

| File | Platform |
|------|----------|
| `luxirisk-macos-arm64` | macOS Apple Silicon |
| `luxirisk-linux-x86_64` | Linux x86_64 |
| `luxirisk-windows-x86_64.exe` | Windows x86_64 |

Also attach these to the GitHub Release tag **`luxirisk-v0.2`**.

Receipt scheme: Ed25519-signed `lxr1_…` (see [`../FORMULAS.md`](../FORMULAS.md)).  
Engine source is **not** published.
