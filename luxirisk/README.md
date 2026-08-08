# LuxiRisk v0.1

**Free · closed-source · tiny offline binary**

Three high-frequency risk calculations for crypto / retail traders, each with a
short, independently verifiable cryptographic receipt — so you can post
trustworthy numbers in Discord, Telegram, and X.

| | |
|---|---|
| **Offline** | Zero network calls. No telemetry. No exchange APIs. |
| **Deterministic** | Same inputs → same numbers and same receipts on macOS, Linux, and Windows. |
| **Closed binary** | Engine source is not distributed. Formulas and test vectors are public. |
| **Tiny & fast** | Single static CLI binary; instant cold start. |

## What it calculates (v0.1 only)

1. **Liquidation price** (isolated margin approximation)
2. **Position size from risk %** of account
3. **Max dollar loss** at stop (consistent with #2)

Exact math: [`FORMULAS.md`](FORMULAS.md)  
Public vectors: [`test-vectors/`](test-vectors/)

## Quick start (< 60 seconds)

### 1. Download

From the release
[**luxirisk-v0.1**](https://github.com/RegularJoe-CEO/LuxiDemo/releases/tag/luxirisk-v0.1):

| Platform | Binary | Checksum |
|----------|--------|----------|
| macOS ARM64 | `luxirisk-macos-arm64` | `luxirisk-macos-arm64.sha256` |
| Linux x86_64 | `luxirisk-linux-x86_64` | `luxirisk-linux-x86_64.sha256` |
| Windows x86_64 | `luxirisk-windows-x86_64.exe` | `luxirisk-windows-x86_64.exe.sha256` |

Repo copies (when present) live under [`dist/`](dist/).

### 2. Verify the binary hash

```bash
# macOS
shasum -a 256 -c luxirisk-macos-arm64.sha256

# Linux
sha256sum -c luxirisk-linux-x86_64.sha256
```

Compare the digest printed for the `.exe` on Windows with the matching `.sha256`
file (PowerShell: `Get-FileHash .\luxirisk-windows-x86_64.exe -Algorithm SHA256`).

### 3. Run

```bash
chmod +x luxirisk-macos-arm64   # macOS / Linux
./luxirisk-macos-arm64 liq --side long --entry 65000 --leverage 10
```

Expected:

```text
Liquidation price: 58825
Receipt (short):   a896b6f35054
```

## CLI

```bash
luxirisk liq  --side long|short --entry PRICE --leverage L [--mmr 0.005]
luxirisk size --balance BAL --risk PCT --entry PRICE --stop PRICE
luxirisk size --balance BAL --risk PCT --entry PRICE --stop-pct PCT
luxirisk risk --size Q --entry PRICE --stop PRICE
luxirisk ui                 # optional localhost page (still 100% offline)
luxirisk vectors            # built-in sanity check
```

**Flags**

- `--full-receipt` — full SHA-256 + canonical payload
- `--json` — machine-readable output
- `--mmr` / `--mmr-pct` — override maintenance margin (default 0.5%)

**Beginner tip:** risk only **1% of your account** per trade.

```bash
luxirisk size --balance 10000 --risk 1 --entry 65000 --stop 63000
# → position size 0.05 base, notional 3250, risk amount 100
```

## Optional local UI

```bash
./luxirisk-macos-arm64 ui
# opens http://127.0.0.1:8765/  (bind override: --bind 127.0.0.1:PORT)
```

The UI is a single embedded page served only on localhost. It never phones home.
Cryptographic receipts are produced by the CLI (authoritative path).

## Verify a receipt independently

No binary required for the hash step:

```bash
python3 test-vectors/verify_receipts.py
```

Or recompute one payload yourself (see [`FORMULAS.md`](FORMULAS.md)):

```bash
python3 - <<'PY'
import hashlib
payload = """luxirisk-receipt-v1
entry=65000
leverage=10
liq_price=58825
mmr=0.005
op=liq
side=long
tool=luxirisk
version=0.1.0
"""
print(hashlib.sha256(payload.encode()).hexdigest()[:12])
# a896b6f35054
PY
```

## Trust model

| Claim | How to check |
|-------|----------------|
| Formulas are public | [`FORMULAS.md`](FORMULAS.md) |
| Outputs match formulas | [`test-vectors/vectors.json`](test-vectors/vectors.json) |
| Receipts match payloads | `python3 test-vectors/verify_receipts.py` |
| Binary is unmodified | Compare to published `.sha256` |
| Offline | No network features; UI binds localhost only |
| Closed source | Only compiled binaries are distributed |

## Non-goals (v0.1)

No funding rates, Kelly criterion, Monte Carlo, prop-firm modes, exchange
connections, website, or open-sourcing of the core engine.

## Release

- Tag: **`luxirisk-v0.1`**
- Product version: **0.1.0**
- Parent catalog: [DEMOS.md](../DEMOS.md) · [LuxiDemo README](../README.md)

## License / distribution

**Proprietary closed binary.** Free to download and use. Engine implementation
source is not published. Public materials in this directory (formulas, vectors,
docs) may be shared freely for verification and education.
