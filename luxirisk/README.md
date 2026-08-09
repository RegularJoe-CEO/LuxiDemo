# LuxiRisk v0.2

**Free · closed-source · tiny offline CLI**

Three high-frequency risk calculations for crypto / retail traders, each with a
**greppable Ed25519-signed receipt** (`lxr1_…`) so numbers posted in Discord,
Telegram, and X can be checked by third parties.

| | |
|---|---|
| **Offline by default** | Zero network calls. No telemetry. No exchange APIs. |
| **Signed receipts** | Per-install Ed25519 key; cannot mint a valid `lxr1_` without the private key. |
| **Deterministic math** | Same inputs → same numbers on macOS, Linux, Windows. |
| **Closed binary** | Engine source is not distributed. Formulas + vectors are public. |

Built by the team behind LuxiEdge — [luxiedge.com](https://luxiedge.com)

## What it calculates (v0.2)

1. **Liquidation price** (isolated margin approximation)
2. **Position size from risk %** of account
3. **Max dollar loss** at stop (consistent with #2)

Exact math + receipt bytes: [`FORMULAS.md`](FORMULAS.md)  
Public vectors: [`test-vectors/`](test-vectors/)  
Changelog: [`CHANGELOG.md`](CHANGELOG.md)

## Quick start

### 1. Download

From [**luxirisk-v0.2**](https://github.com/RegularJoe-CEO/LuxiDemo/releases/tag/luxirisk-v0.2)
(or [`dist/`](dist/) when present):

| Platform | Binary |
|----------|--------|
| macOS ARM64 | `luxirisk-macos-arm64` |
| Linux x86_64 | `luxirisk-linux-x86_64` |
| Windows x86_64 | `luxirisk-windows-x86_64.exe` |

Each ships with a matching `.sha256`. Prefer also checking **GitHub Actions
provenance attestation** on the release (see Trust model).

### 2. Verify the binary hash

```bash
shasum -a 256 -c luxirisk-macos-arm64.sha256   # macOS
sha256sum -c luxirisk-linux-x86_64.sha256      # Linux
```

### 3. Run

```bash
chmod +x luxirisk-macos-arm64
./luxirisk-macos-arm64 liq --side long --entry 65000 --leverage 10
```

First run creates a **per-install identity** (Ed25519 keypair) and prints your
fingerprint. Receipts look like:

```text
Liquidation price: 58825
Receipt:           lxr1_TFhSAg…
Fingerprint:       9e53c8756c83c875   # your install's id differs
```

## CLI

```bash
luxirisk liq  --side long|short --entry PRICE --leverage L [--mmr 0.005]
luxirisk size --balance BAL --risk PCT --entry PRICE --stop PRICE
luxirisk risk --size Q --entry PRICE --stop PRICE
luxirisk verify lxr1_… --payload-file claim.txt
luxirisk verify lxr1_… liq --side long --entry … --leverage … --expect-liq …
luxirisk id                 # show fingerprint + public key
luxirisk vectors            # formula + signature self-test
```

**Flags**

| Flag | Meaning |
|------|---------|
| `--full-receipt` | Print canonical payload + signature hex |
| `--json` | Machine-readable output |
| `--stamp` | **Opt-in network**: bind to latest [drand](https://drand.love) beacon |
| `--beacon VALUE` | Offline time-binding with a user-supplied value |

`--stamp` is the **only** network call LuxiRisk ever makes. Endpoint:

```text
https://api.drand.sh/public/latest
```

Beginner tip: risk only **1% of your account** per trade.

## Verify a receipt offline

### With the binary

```bash
./luxirisk-macos-arm64 verify 'lxr1_…' liq \
  --side long --entry 65000 --leverage 10 --expect-liq 58825
```

### Without the binary (Python)

```bash
pip install cryptography   # or: pip install pynacl
python3 test-vectors/verify_receipts.py
```

## Trust model (honest)

| Claim | Status | How to check |
|-------|--------|----------------|
| Formulas are public | Yes | [`FORMULAS.md`](FORMULAS.md) |
| Outputs match formulas | Yes | [`test-vectors/`](test-vectors/) |
| Receipts are Ed25519-signed | Yes | `verify` / `verify_receipts.py` |
| Randoms cannot mint valid `lxr1_` | Yes | Needs install private key (or RE) |
| Default fully offline | Yes | No sockets unless `--stamp` |
| Per-install stable fingerprint | Yes | `luxirisk id` |
| Binary hash published | Yes | `.sha256` + `SHA256SUMS` |
| Build provenance (CI attestation) | When released via GHA | GitHub Attestations on the release |
| Apple notarization / Windows Authenticode | **When signing credentials available** | See residual risks |

### Residual risks (do not overclaim)

1. **Per-install private key lives on the user's disk.** Malware with file
   access can steal it and forge receipts *as that install*. Protect the config
   directory; treat fingerprints like pseudonyms, not hardware roots of trust.
2. **A reverse-engineered binary can still sign.** Closed source raises the
   cost; it does not create cryptographic impossibility for a determined
   attacker who extracts or reimplements the signer. What it *does* stop is the
   v0.1 failure mode: casual minting of “valid” hashes with no key material.
3. **Unsigned / un-notarized downloads** may trigger OS warnings on stock
   macOS/Windows until Developer ID + notarization (macOS) and Authenticode
   (Windows) are applied for a given release. Prefer hashes + provenance even
   when OS SmartScreen/Gatekeeper is green.
4. **`--stamp` trusts the drand HTTPS endpoint** for that moment. Prefer
   offline `--beacon` or verify the round against an independent archive.
5. **Social trust ≠ market truth.** A verified receipt proves “this install
   signed these numbers under these formulas,” not that an exchange would
   liquidate at that price.

## Identity location

| OS | Path |
|----|------|
| macOS | `~/Library/Application Support/LuxiRisk/` |
| Linux | `~/.config/luxirisk/` |
| Windows | `%APPDATA%\LuxiRisk\` |
| Override | `LUXIRISK_HOME` |

## Non-goals (v0.2)

No funding rates, Kelly, Monte Carlo, prop-firm modes, exchange connections,
website, open-sourced engine, or local UI (removed; CLI only).

## Release

- Tag: **`luxirisk-v0.2`**
- Product version: **0.2.0**
- Parent catalog: [DEMOS.md](../DEMOS.md)

## License / distribution

**Proprietary closed binary.** Free to download and use. Engine implementation
source is not published. Public materials in this directory may be shared freely
for verification and education.

---

Built by the team behind LuxiEdge — [luxiedge.com](https://luxiedge.com)
