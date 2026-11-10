# Public demo catalog

Luxi demos are distributed as **compiled evaluation binaries**. Evaluators can
run the tools, supply inputs, inspect outputs, and compare **output vector hashes**
(and signed calculation receipts) without receiving the private implementation source.

**Portfolio:** LuxiEdge is one core engine with seven commercial products ([luxiedge.com](https://luxiedge.com)). Only some have anything public here, and they are not equally validated. Public downloads in this repository: the **Luxi inference replay demo** (Demo 1, closed binary, Linux x86_64) and the free **LuxiBook** demo (Demo 0, includes **LuxiSize**). The LuxiQuant Risk State Engine (Risk Runtime v0.4.2) has no public download or public prices ([luxiedge.com/luxiquant](https://luxiedge.com/luxiquant)). Haplotastic has no public package here.  
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

**Current v0.2.1 matrix (2026-08-16):** five GPUs (H200, H100, A100, RTX 5090, RTX 4090), each on both the LuxiBook CPU and CUDA pricing paths, with 20 signed receipts across two books (`example_book.csv` and `stress_book_a.csv`): [`downloads/luxibook/evidence/v0.2.1-phi-fix-matrix/`](downloads/luxibook/evidence/v0.2.1-phi-fix-matrix/). This LuxiBook matrix is separate from the November inference results.  
**Historical v0.2.0 matrix (engine `b4645c2`, determinism only):** Mac Mini arm64 CPU + RTX 4090, H200, H100×2, stored under [`evidence/v0.2.0-matrix/`](downloads/luxibook/evidence/v0.2.0-matrix/). Do not use v0.2.0 for accuracy claims (see [`downloads/luxibook/README.md`](downloads/luxibook/README.md)).  
**Non-claim:** this book, these devices, this kernel - not all GPUs always match.

The signed `lxq2_…` receipt is **per-install** (not a published constant). Details: [`downloads/luxibook/README.md`](downloads/luxibook/README.md).

Docs: this file · binaries: [`downloads/luxibook/`](downloads/luxibook/) · engine source: **private** (not in this repo)

### Part of LuxiBook: LuxiSize sizing calculator (`lxr1_` receipts)

**Included with the free LuxiBook download. Not the LuxiQuant Risk State Engine and not the LuxiBook option pricer.**
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

**Two different v0.2 builds: pair each download with its own checksum.** The binaries in [`luxirisk/dist/`](luxirisk/dist/) and the assets on the `luxirisk-v0.2` release carry the same version label but have different digests. Check a repository copy with the `.sha256` or `SHA256SUMS` in `luxirisk/dist/` (run `shasum -a 256 -c` inside that folder). Check a release download with the release's own `SHA256SUMS` (`sha256sum -c SHA256SUMS --ignore-missing` in the download folder); the release's per-file `.sha256` files carry a `dist/` path prefix, so compare their digest by eye.

| File | `luxirisk/dist/` sha256 | `luxirisk-v0.2` release sha256 |
|---|---|---|
| `luxirisk-linux-x86_64` | `66a4ab65fd71316db059cfd04b2a7c39e0de733352333f68168cf74fd56972e3` | `bc0a2bb606f050cdf8c8ecc3101a2d4df42b55f8a43c60a6d0394eeb26a00e57` |
| `luxirisk-macos-arm64` | `154dc9c990c46c0d6b2afe02aa676fd8ece33a3f70e61b00002a92bbb507c284` | `6385d6cb55260b9b223fe6b26726ec03272de8d25569b2b0adaf07b2cd402a9f` |
| `luxirisk-windows-x86_64.exe` | `50f4e00ce60c37f0de75262c3ea5605768f6049cfa62933f448b32386f0df17e` | `878023843c37303636becc44d160197de63ebdd6c04671ca6ab0856265adbf95` |

## Demo 1: Luxi inference replay (closed binary, Linux x86_64)

**Current inference download. Closed binary, evaluation license; engine source private.** A frozen DeepSeek-R1-Distill-Qwen-7B w8a8 job from the November 2026 R1 evaluation, replayed by a closed-binary build of the Luxi engine (Attention Transformer v2, engine v27 final). Runs on an NVIDIA GPU (compute capability 8.0, 8.6, 8.9, 9.0 or 12.0) or an x86_64 CPU (tested: AMD EPYC 7543, Intel Xeon Gold 6342 and one sandbox CPU whose model the VM did not expose). Not a chat app, API server or speed benchmark.

| File | sha256 |
|---|---|
| [`downloads/luxi-replay-demo/luxi-replay-demo-1.0.1-linux-x86_64.tar.gz`](downloads/luxi-replay-demo/luxi-replay-demo-1.0.1-linux-x86_64.tar.gz) (25,476,758 bytes) | `c2114aa7bd6c47822bdf94de2f7d77f1aa773a32dcc3268072cfb979c2e5c8b3` |

```bash
curl -LO https://github.com/RegularJoe-CEO/LuxiDemo/raw/88ebbe60311070f292cb81ba07a7132242ca694a/downloads/luxi-replay-demo/luxi-replay-demo-1.0.1-linux-x86_64.tar.gz
echo "c2114aa7bd6c47822bdf94de2f7d77f1aa773a32dcc3268072cfb979c2e5c8b3  luxi-replay-demo-1.0.1-linux-x86_64.tar.gz" | sha256sum -c
tar xzf luxi-replay-demo-1.0.1-linux-x86_64.tar.gz && cd luxi-replay-demo-1.0.1-linux-x86_64
sha256sum -c SHA256SUMS
./install.sh --gpu                 # or --cpu
./luxi fetch-model                 # 8.7 GB from Hugging Face, sha256-checked
./luxi inspect --device gpu --check-weights
./luxi demo --device gpu           # or --device cpu; RESULT: SAME OUTPUTS = identical to the signed Nov 9 H100 NVL record
./luxi verify luxi_out/bundle_tiny-math500-090_gpu-*.json --trusted-key <public key printed by the demo>
./luxi compare reference/nov09_h100_nvl_signed/short_replay/bundle_tiny_h100.json luxi_out/bundle_tiny-math500-090_gpu-*.json
./luxi tamper-test reference/nov09_h100_nvl_signed/full_suite/bundle_a4-math500-090.json \
     --trusted-key reference/keys/luxi_benchmark_key_2026-11.pub
./luxi compare <record from machine A> <record from machine B>
./luxi full-suite --device gpu     # 136 answers vs the signed November 9 H100 NVL records
```

Tested on November 10, 2026 from a clean install. Release 1.0.1: the 32-token replay was identical to the signed November 9 H100 NVL short record on an RTX A6000, its host CPU (Intel Xeon Gold 6342) and a sandbox x86_64 CPU, and the full suite matched 136/136 answers and 546,114 tokens of the signed November 9 H100 NVL records on the RTX A6000. Release 1.0.0, which ships the same engine files: identical 32-token records on an RTX 5090, an A40, their host CPUs (AMD EPYC 7543, Intel Xeon Gold 6342) and the sandbox CPU, also identical to the 1.0.1 records, and the full suite matched 136/136 on the RTX 5090 and the A40. Other listed GPUs are compiled for but were not run with this build. Release 1.0.0 (sha256 `b1f6cf0b…39c9`) stays in the same folder. Exact commands, license, identities, changes and limits: [`downloads/luxi-replay-demo/README.md`](downloads/luxi-replay-demo/README.md). Evidence: [`evidence/r1-replay-package-1.0.1-2026-11-10/`](evidence/r1-replay-package-1.0.1-2026-11-10/), [`evidence/r1-replay-package-2026-11-10/`](evidence/r1-replay-package-2026-11-10/) and [`evidence/r1-distill-7b-gpu-swap-2026-11-09/`](evidence/r1-distill-7b-gpu-swap-2026-11-09/). Your own model or workload: [private GPU-swap evaluation](https://luxiedge.com/contact#gpu-swap-evaluation).

## Demo 2: version-100 serve (historical API toy)

**Historical. Binary only - no engine source. Not LuxiBook, and not the current reasoning engine.** OpenAI-shaped HTTP API with a toy generate path, kept for its API shape. For real inference replay use [Demo 1](#demo-1-luxi-inference-replay-closed-binary-linux-x86_64).

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
- Its generate path does not run the benchmarked engine. Historical September H100 results: [`BENCHMARKS.md`](BENCHMARKS.md)
- Local generate path is a toy path for instant API demos. Completions return a `token_receipt` and `latency_ms`.
- `/v1/gtm` returns build info, a determinism note and a link to [`BENCHMARKS.md`](BENCHMARKS.md); `/health` includes `build_version` and the benchmarks link; the `/dashboard` top card links to BENCHMARKS.md.

## Download and verify (numerical v3.0, historical)

**Historical.** The v3.0 release (June 2026) describes 10-day-expiring demo binaries. They are kept as a record of the numerical demos, not as current working packages, and they are not inference packages. Demos 3 to 8 below use them.

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

## Demo 9: Version 99 evidence verifier (historical)

The Version 99 Qwen2-7B engine (July 2026) is not distributed. This verifier is a summary arithmetic check, not a
runnable engine or a raw-output comparison; for a runnable replay use [Demo 1](#demo-1-luxi-inference-replay-closed-binary-linux-x86_64). Its public arithmetic check is:

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
| **LuxiSize** (sizing calculator in LuxiBook) | Offline calculator CLI + Ed25519 `lxr1_` receipts + verify + public vectors | [**luxirisk-v0.2**](https://github.com/RegularJoe-CEO/LuxiDemo/releases/tag/luxirisk-v0.2) (**OS binaries not code-signed**; Gatekeeper/SmartScreen friction) |
| LuxiQuant numerical | v3.0 binary validation + REST + operator receipts | Keep secondary to Book; optional multi-platform numerical matrix |
| **Inference** (commercial product) | Demo 1: closed-binary R1 replay package (GPU and CPU replay, offline verify, tamper test, compare, 136-answer GPU suite); November 9 nine-GPU evidence; November 10 package test. Historical: September H100 benchmarks vs vLLM and SGLang ([`BENCHMARKS.md`](BENCHMARKS.md)), drift demo ([`INFERENCE.md`](INFERENCE.md#drift-demo)), Version 99 verifier, serve API toy | Run the package on more GPU types; your own model or workload through a [private GPU-swap evaluation](https://luxiedge.com/contact#gpu-swap-evaluation) |
| Haplotastic (commercial product) | None in this repository | [luxiedge.com/haplotastic](https://luxiedge.com/haplotastic) |
| Genomics research (Dorado, PairHMM) | Oxford Nanopore Dorado basecaller drift proof + like-for-like CUPTI GPU kernel timings ([`proofs/dorado-drift/DORADO_DRIFT.md`](proofs/dorado-drift/DORADO_DRIFT.md)); Deterministic-PairHMM paper | Research studies of single workloads, not a Haplotastic variant-calling benchmark package; no public engine |
| LuxiPak | None | Admission/placement trace versus a baseline |
| LuxiPhase | None | Synthetic load-shaping trace with SLO and stability checks |
| LuxiLoad (roadmap) | None | Roadmap only |
| LuxiSDG (historical) | None | Historical work; not a currently available product |

Concept illustrations are not labelled as working demos.

## Distribution

The public packages contain compiled evaluation binaries, documentation, and
test evidence. Proprietary implementation source and model weights are not
distributed.
