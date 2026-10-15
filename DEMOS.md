# Public demo catalog

Luxi demos are distributed as **compiled evaluation binaries**. Evaluators can
run the tools, supply inputs, inspect outputs, and compare **output vector hashes**
(and signed calculation receipts) without receiving the private implementation source.

**Portfolio:** LuxiQuant Risk State Engine (Risk Runtime v0.4.0) is the paid LuxiQuant product (evaluation-only, no public download; [luxiedge.com](https://luxiedge.com/luxiquant)). Luxi **Book** is the free public receipt and determinism demonstration. Luxi**Risk** v0.2 is a separate free legacy demo calculator.  
This repo ships **binaries + markdown only** - marketing HTML lives on Replit (luxiedge.com), not here.

## Demo 0: LuxiBook - CSV European options + attested receipt (the Quant try)

**Primary public Quant artifact. Closed binary only - no engine source.**  
**OS binary is not Apple-notarized / code-signed** (Gatekeeper friction).  
**Calculation receipt is Ed25519-signed** (`luxiquant-receipt-v2`, `lxq2_…`) - different from the OS “unsigned” note.

CSV position book → European **Black-Scholes / Black-76** → five Greeks → **output vector SHA-256** on the canonical f64 LE vector → **signed receipt**. You bring `T`, `r`, and `σ`. No live feed, no IV, no VaR, not a fund desk. Not `risk-pipeline`.

| Platform | Binary |
|----------|--------|
| macOS ARM64 (CPU) | [`downloads/luxibook/luxi-book-macos-arm64`](downloads/luxibook/luxi-book-macos-arm64) |
| Linux x86_64 (CPU) | [`downloads/luxibook/luxi-book-linux-x86_64`](downloads/luxibook/luxi-book-linux-x86_64) |
| Linux x86_64 (CUDA / NVIDIA) | [`downloads/luxibook/luxi-book-linux-x86_64-cuda`](downloads/luxibook/luxi-book-linux-x86_64-cuda) |
| Example CSV | [`downloads/luxibook/example_book.csv`](downloads/luxibook/example_book.csv) |

No macOS GPU binary. CUDA binary needs a live NVIDIA driver at runtime for `--mode gpu`.

```bash
chmod +x downloads/luxibook/luxi-book-macos-arm64
shasum -a 256 -c downloads/luxibook/luxi-book-macos-arm64.sha256
./downloads/luxibook/luxi-book-macos-arm64 price \
  --book downloads/luxibook/example_book.csv \
  --out report.csv --receipt receipt.json
./downloads/luxibook/luxi-book-macos-arm64 verify \
  --book downloads/luxibook/example_book.csv \
  --receipt receipt.json

# Linux NVIDIA:
./downloads/luxibook/luxi-book-linux-x86_64-cuda price \
  --book downloads/luxibook/example_book.csv \
  --out report.csv --receipt receipt.json --mode gpu
```

**Comparable value on `example_book.csv`:** `output_vector_sha256` =  
`4a21b1e708fa5c694bf48237df5e5bd3b94599e6273d07986283c6c6b8e3c97a`  
ATM_CALL `10.4505835721856215`. Kernel `black_scholes_book_kernel`.

**v0.2.0 matrix (engine `b4645c2`):** Mac Mini arm64 CPU + four NVIDIA devices (RTX 4090 Ada, H200, H100×2) with CPU↔GPU hash agreement on this book - see [`downloads/luxibook/README.md`](downloads/luxibook/README.md) and stored receipts under [`evidence/v0.2.0-matrix/`](downloads/luxibook/evidence/v0.2.0-matrix/).  
**A100:** still **historical** only (2026-08-15 pre-attestation binary, same output hash) - not re-measured under v0.2.0.  
**Non-claim:** this book, these devices, this kernel - not all GPUs always match.

The signed `lxq2_…` receipt is **per-install** (not a published constant). Details: [`downloads/luxibook/README.md`](downloads/luxibook/README.md).

Docs: this file · binaries: [`downloads/luxibook/`](downloads/luxibook/) · engine source: **private** (not in this repo)

## Demo 1: LuxiRisk v0.2 - free legacy demo calculator (`lxr1_` receipts)

**Free legacy demo calculator - not the LuxiQuant Risk State Engine and not LuxiBook.**
**Closed binary only - no engine source. OS code-signing is not applied (unsigned
download; expect Gatekeeper / SmartScreen friction).** Three calculations for
leveraged crypto / retail traders, each with an **Ed25519-signed** `lxr1_…`
**calculation receipt** (per-install identity) for Discord, Telegram, and X.

| Calculation | Example |
|-------------|---------|
| Liquidation price (isolated approx.) | `liq --side long --entry 65000 --leverage 10` → **58825** |
| Position size from risk % | `size --balance 10000 --risk 1 --entry 65000 --stop 63000` → **0.05** base |
| Max $ loss at stop | `risk --size 0.5 --entry 65000 --stop 63000` → **1000** |

| Platform | Binary in repo |
|----------|----------------|
| macOS ARM64 | [`luxirisk/dist/luxirisk-macos-arm64`](luxirisk/dist/luxirisk-macos-arm64) |
| Linux x86_64 | [`luxirisk/dist/luxirisk-linux-x86_64`](luxirisk/dist/luxirisk-linux-x86_64) |
| Windows x86_64 | [`luxirisk/dist/luxirisk-windows-x86_64.exe`](luxirisk/dist/luxirisk-windows-x86_64.exe) |

```bash
chmod +x luxirisk/dist/luxirisk-macos-arm64
shasum -a 256 -c luxirisk/dist/luxirisk-macos-arm64.sha256
# macOS first run: right-click → Open (binary is unsigned)
./luxirisk/dist/luxirisk-macos-arm64 liq --side long --entry 65000 --leverage 10
# receipt starts with lxr1_  ·  fingerprint is per-install
./luxirisk/dist/luxirisk-macos-arm64 verify 'lxr1_…' liq \
  --side long --entry 65000 --leverage 10 --expect-liq 58825
pip install cryptography && python3 luxirisk/test-vectors/verify_receipts.py
```

- Home: [`luxirisk/`](luxirisk/) · formulas: [`luxirisk/FORMULAS.md`](luxirisk/FORMULAS.md)
- Test vectors: [`luxirisk/test-vectors/`](luxirisk/test-vectors/)
- Release: [**luxirisk-v0.2**](https://github.com/RegularJoe-CEO/LuxiDemo/releases/tag/luxirisk-v0.2) (**unsigned** binaries + checksums)
- Offline by default · optional `--stamp` / `--beacon` · CLI only · Built by the team behind LuxiEdge - luxiedge.com
## Demo 2: version-100 commercial serve (inference)

**Binary only - no engine source. Not LuxiBook.** OpenAI-shaped HTTP API with a toy generate path.

| Platform | Binary in repo |
|----------|----------------|
| macOS ARM64 | [`downloads/luxiedge-serve-macos-arm64`](downloads/luxiedge-serve-macos-arm64) |
| Linux x86_64 | [`downloads/luxiedge-serve-linux-x86_64`](downloads/luxiedge-serve-linux-x86_64) |

```bash
chmod +x luxiedge-serve-macos-arm64
./luxiedge-serve-macos-arm64 --bind 127.0.0.1:8787
curl -s http://127.0.0.1:8787/v1/models | python3 -m json.tool
curl -s http://127.0.0.1:8787/v1/gtm | python3 -m json.tool
curl -s -X POST http://127.0.0.1:8787/v1/audit -d '{}'
```

- Package (binaries + docs): [`demo/luxiedge-yc-demo/`](demo/luxiedge-yc-demo/) · also [`downloads/`](downloads/)
- Measured H100 results: [`BENCHMARKS.md`](BENCHMARKS.md)
- Local generate path is a toy path for instant API demos. Completions return a `token_receipt` and `latency_ms`.
- `/v1/gtm` returns build info, a determinism note and a link to [`BENCHMARKS.md`](BENCHMARKS.md); `/health` includes `build_version` and the benchmarks link; the `/dashboard` top card links to BENCHMARKS.md.

## Download and verify (numerical v3.0)

Download the matching files from the
[v3.0 release](https://github.com/RegularJoe-CEO/LuxiDemo/releases/tag/v3.0):

- `luxiedge-demo-<platform>`: 30-operator numerical demo and REST service
- `luxiedge-lite-<platform>`: smaller numerical evaluation surface
- `luxi-tools-<platform>`: ATE, energy, orbital, and robotics demonstrations
- matching `.sha256` files

Example for Linux x86-64:

```bash
chmod +x luxiedge-demo-linux-x86_64 luxi-tools-linux-x86_64
sha256sum luxiedge-demo-linux-x86_64
sha256sum luxi-tools-linux-x86_64
```

Compare each printed digest with the corresponding downloaded `.sha256` file.
The checksum files retain their release-build path prefix, so compare the digest
rather than assuming `sha256sum -c` will resolve that path from every download
directory.

## Demo 3: numerical validation (not the option book)

```bash
./luxiedge-demo-linux-x86_64 validate
./luxiedge-demo-linux-x86_64 list
```

The current demo lists 30 operators, including transcendental functions,
statistics, activation functions, `quant_chain`, GELU, SiLU, and RMSNorm.

## Demo 4: local REST evaluation

Start the compiled service:

```bash
./luxiedge-demo-linux-x86_64 --port 9090
```

From another terminal:

```bash
curl -X POST http://127.0.0.1:9090/evaluate \
  -H "Content-Type: application/json" \
  -d '{"expr":"sin(x)","values":[0.5,1.0,1.57,2.0,3.14],"precision":"f32"}'
```

On the tested Linux x86-64 v3.0 binary, three repeated requests produced the
same output receipt:

`27365fdff1b57d9a8077acdc40ca9d6183481a11dd5928e9f7b7e4782d2019a9`

That is a receipt for this published v3.0 binary, this request, and this
machine. It is not a cross-platform guarantee. The v3.1 measurement, which is
not a release, returned f32 `sin(x)` receipt
`0243d52127de37af8a1f49edadcf74a2a299732a5bec38693ee5b1804df28965`
on macOS arm64, macOS x86_64, Linux x86_64, and Linux arm64. GPU was not
measured. `sin(1.57)` was `0.9999997019767761` and `sin(3.14)` was
`0.0015925479819998145`.

The other rows below were measured with `luxiedge-demo` on macOS arm64 only.
They were not re-measured on the other three machines.

| Expression | SHA256 |
|------------|--------|
| sin(x) | 0243d52127de37af8a1f49edadcf74a2a299732a5bec38693ee5b1804df28965 |
| cos(x) | f764241c7473a2306182dd086eae59a5e6e184e38b9d87b07115430744c96666 |
| tan(x) | c2f3a5113c4d5d5febf7ce5e47565aad77bd58382a3ca37e094c766039e5e16a |
| sin(x)*cos(x) | 7f488d2bd014e226f44f366c9e6dcee826667f2d32a0f07e98e21f692cbbfbc2 |
| exp(x) | e9fa90fa7109168e2c6e33c09f4bdfe33f950d1618c63daa94ca50091c22c489 |
| ln(x) | d9f04ca2fca66db0a39f7acfce0c17f3392f8c5d685ffd6b3cdd9878bece894f |
| log10(x) | 3aaaf6e452f8e23a84d92cf205b3407da8f653c824e2f197d6ad5b4bf3777802 |
| sqrt(x) | 3d88c5e3c24598b45ea32cd2347356c254e3cb9d2f92fbb0ce90b0ad6dff3f77 |
| cbrt(x) | ef3075de3e76a739fc32e1affc73c38efda190c8715e4a82ccd5c5309d6e4e67 |
| x^2 | 96620536e0763f86534ef44444a322a03c5e73b8415d737ebcd92206b8f069da |
| x^3 | 3bf83ac5abbab1ce8ff39d19057a4a8bb5fc563dc692a1c17368384149533204 |
| erf(x) | 91da32b8350f1ea6bd387b659c837de40bd7853aa4db2d0c2681a6143ae77988 |
| normcdf(x) | 5bf3342a33330d9bb694053a061d7764f5ca8a5a9927ae58dbb2764b87b35a36 |
| normpdf(x) | 27876beeed37a8722fa68c027f3a91a81c4e0b1fc54cafafa45d7ff5f7940cf7 |
| gamma(x) | 0f45aab892b79f822d0cae278d0d311cca4492f0af00dfb44c295a369b80f24d |

## Demo 5: ATE / Waller / WNSM

```bash
./luxi-tools-linux-x86_64 ate
```

The command reports NORMAL and WNSM receipts, maximum output difference,
payload bytes avoided, and an estimated energy field. Treat estimated energy as
illustrative unless a corresponding measured evidence pack is cited.

The tested Linux x86-64 build reported identical NORMAL/WNSM receipts and
`0.00e0` maximum output difference.

## Demo 6: energy-path illustration

```bash
./luxi-tools-linux-x86_64 energy
```

This command illustrates payload-avoidance accounting at several shapes. It is
not the Version 99 H100 measurement and is not facility-energy evidence.

## Demo 7: scientific solvers

```bash
./luxi-tools-linux-x86_64 orbital
```

The orbital demonstration emits receipts for Kepler, Vis-Viva, Hohmann,
drag-decay, J2, and Lambert-style calculations.

## Demo 8: edge and robotics math

```bash
./luxi-tools-linux-x86_64 robotics
```

The robotics demonstration emits receipts for inverse kinematics, Kalman
weighting, tire slip, motor torque, clothoid curvature, and lidar distance.

## Demo 9: inference evidence verifier

The full Version 99 Qwen2-7B engine is not distributed from this public demo
repository. Its public arithmetic check is:

```bash
python3 scripts/verify_v99_pack.py
```

The verifier recomputes the published throughput and GPU-board-energy ratios
from the retained per-run table. It exposes the evidence contract, not the
proprietary engine source. See also [`scripts/README.md`](scripts/README.md).

## Demo coverage and next gates

| Luxi layer | Public demo today | Next validation step |
|---|---|---|
| **LuxiBook** (free demo) | CSV BS/Black-76 + Greeks + output-vector hash + Ed25519 `lxq2_` receipt; macOS + Linux CPU + Linux CUDA | Free demonstration only; not evidence for the Risk State Engine; no universal GPU claim |
| **LuxiRisk** (free legacy demo calculator) | Offline calculator CLI + Ed25519 `lxr1_` receipts + verify + public vectors | [**luxirisk-v0.2**](https://github.com/RegularJoe-CEO/LuxiDemo/releases/tag/luxirisk-v0.2) (**OS binaries not code-signed**; Gatekeeper/SmartScreen friction) |
| LuxiQuant numerical | v3.0 binary validation + REST + operator receipts | Keep secondary to Book; optional multi-platform numerical matrix |
| Inference evidence (not an offer) | Serve API demo + H100 benchmarks vs vLLM and SGLang ([`BENCHMARKS.md`](BENCHMARKS.md)) + drift demo ([`INFERENCE.md`](INFERENCE.md#drift-demo)) + Version 99 verifier | Evidence only; inference is not a separate paid offer |
| LuxiPak | None | Admission/placement trace versus a baseline |
| LuxiPhase | None | Synthetic load-shaping trace with SLO and stability checks |
| LuxiLoad (roadmap) | None | Roadmap only |
| LuxiSDG (historical) | None | Historical work; not a currently available product |

Concept illustrations are not labelled as working demos.

## Distribution

The public packages contain compiled evaluation binaries, documentation, and
test evidence. Proprietary implementation source and model weights are not
distributed.
