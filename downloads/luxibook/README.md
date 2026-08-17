# Luxi Book — public try

Closed binaries from [luxi-quant-engine](https://github.com/RegularJoe-CEO/luxi-quant-engine) **v0.2.1**, built from engine commit `02388f7`— the value each binary reports as `git_sha` (`git_dirty=false`). **v0.2.1** is the Φ fix (Taylor `|x|≤1.5`; A&S 7.1.14 continued-fraction `erfc` for `|x|>1.5`). Do not use a **v0.2.0** download for accuracy claims outside the acceptance band — that version could return non-probabilities for `|d| ≳ 2.13`. Post-fix matrix: [`evidence/v0.2.1-phi-fix-matrix/`](evidence/v0.2.1-phi-fix-matrix/). Historical v0.2.0 matrix (determinism only): [`evidence/v0.2.0-matrix/`](evidence/v0.2.0-matrix/). **No engine source. No NDA.**

**“Unsigned” here means the OS binary is not Apple-notarized / code-signed.**  
It does **not** mean the calculation receipt is unsigned. Receipts are **Ed25519-signed** (`lxq2_…`).

## Downloads

| File | Platform | Notes |
|------|----------|--------|
| `luxi-book-macos-arm64` | macOS Apple Silicon | CPU only |
| `luxi-book-linux-x86_64` | Linux x86_64 | CPU only |
| `luxi-book-linux-x86_64-cuda` | Linux x86_64 + NVIDIA | supports `--mode gpu` |
| `example_book.csv` | all | synthetic example book |
| `*.sha256` | all | binary checksums (content hash of the download) |
| [`evidence/v0.2.1-phi-fix-matrix/`](evidence/v0.2.1-phi-fix-matrix/) | all | post-Φ-fix matrix (example + stress A/B) |
| [`evidence/v0.2.0-matrix/`](evidence/v0.2.0-matrix/) | all | historical v0.2.0 matrix (do not mix with v0.2.1) |

There is **no** macOS GPU binary. Use the CUDA Linux build only on machines with an NVIDIA driver.

## Commands

```bash
# Mac
chmod +x luxi-book-macos-arm64
shasum -a 256 -c luxi-book-macos-arm64.sha256
./luxi-book-macos-arm64 price --book example_book.csv --out report.csv --receipt receipt.json
./luxi-book-macos-arm64 verify --book example_book.csv --receipt receipt.json

# Linux CPU
chmod +x luxi-book-linux-x86_64
./luxi-book-linux-x86_64 price --book example_book.csv --out report.csv --receipt receipt.json
./luxi-book-linux-x86_64 verify --book example_book.csv --receipt receipt.json

# Linux CUDA (NVIDIA required at runtime for --mode gpu)
chmod +x luxi-book-linux-x86_64-cuda
./luxi-book-linux-x86_64-cuda price --book example_book.csv --out report.csv --receipt receipt.json --mode gpu
./luxi-book-linux-x86_64-cuda verify --book example_book.csv --receipt receipt.json --mode gpu
```

Optional: `--stamp` (requires a stamp-enabled build) brackets the run with a public randomness beacon for a **not-before** time bound only — not a two-sided “timestamp.”

First `price` creates a per-install Ed25519 key (mode `0600`) under the platform config directory (or `LUXIQUANT_HOME`). No setup step is required.

## Deterministic output (the comparable value)

For **`example_book.csv` only**, the **output vector hash** is:

`4a21b1e708fa5c694bf48237df5e5bd3b94599e6273d07986283c6c6b8e3c97a`

ATM_CALL price: `10.4505835721856215`  
`book_price` bits: `343698c067f66240` (full decimal `151.70016507843832`)

Hostile **`stress_book_a.csv`** (237 rows; post-fix constant on v0.2.1):

`902667a1070b83bff57ac642cf16779d998b5a954c046c450b154237e9e196e2`  
`book_price` = `488338590.9188194`

This hash is **not** the signed receipt string. Two installs can share this hash and still produce **different** `lxq2_…` seals. That is intentional.

### Cross-box matrix after Φ fix (v0.2.1)

See [`evidence/v0.2.1-phi-fix-matrix/`](evidence/v0.2.1-phi-fix-matrix/). Five GPUs (H200, H100, A100, RTX 5090, RTX 4090): example + stress A bit-identical CPU↔GPU. **Those hosts are not the v0.2.0 matrix pods** — do not merge the tables (e.g. the v0.2.1 4090 host reports `avx2`; the v0.2.0 4090 host reported `avx512`).

### Cross-box matrix (v0.2.0 historical · same CUDA binary on GPU rows)

**Superseded for accuracy.** Kept as determinism evidence on the pre-fix Φ path for `example_book.csv` only.

Measured on engine commits **`b4645c2`** (RTX 4090, H200, H100 A/B) and **`4a86d2a`** (A100, RTX 5090). Receipts from either verify under the current download.

**What this measures:** CPU scalar gold path and the CUDA kernel (device port of `exp`/`ln`/`erf`/`normcdf`/`normpdf`, no vendor `libm` for those five) agree bit for bit on this book — across **Ampere** (A100), **Ada** (RTX 4090), **Hopper** (H100/H200), and **Blackwell** (RTX 5090).

| Device | Arch | Host CPU | GPU UUID | CPU | GPU | CPU↔GPU |
|--------|------|----------|----------|-----|-----|---------|
| **RTX 5090** | Blackwell CC 12.0 · **JIT of sm_80 PTX** (not a baked sm_120 target) | EPYC 9354 · avx512 | `GPU-93ed02cb-…` | PASS | PASS | agree |
| RTX 4090 | Ada sm_89 | EPYC 9254 · avx512 | `GPU-4c59e6d9-…` | PASS | PASS | agree |
| **A100-SXM4-80GB** | Ampere sm_80 | EPYC 7543 · **avx2** (first avx2 x86 host in this matrix) | `GPU-b2a79db3-…` | PASS | PASS | agree |
| H200 | Hopper sm_90 | Xeon 8568Y+ · avx512 | `GPU-2d296bd2-…` | PASS | PASS | agree |
| H100 80GB (A) | Hopper sm_90 | Xeon 8462Y+ · avx512 | `GPU-b3bcc572-…` | PASS | PASS | agree |
| H100 80GB (B) | Hopper sm_90 | Xeon 8462Y+ · avx512 | `GPU-d13abf0e-…` | PASS | PASS | agree |
| Mac Mini (arm64 download) | — | Apple Silicon | — | PASS | n/a | — |

**RTX 5090 code path (strongest row — do not blur with offline sm_120 builds):** the published binary embeds **PTX compiled for sm_80** (`cuda_build`: `features=cuda; PTX sm_80; -fmad=false`). On CC 12.0 the driver **JIT-compiles that PTX at runtime**. There is **no** compiled-in sm_120 (or sm_120f) cubin in this binary. A distinct code-generation path, produced at runtime on hardware newer than the kernel’s PTX target, still matched the CPU scalar path bit for bit on `example_book.csv`.

**A100** is a **measured v0.2.0** row (`4a86d2a`), not a v1-era historical line. Its host reports `host_simd_capability=avx2` (book pricing is still the scalar deterministic path on every host).

**Scope of the table:** 6 NVIDIA GPUs on **5 RunPod public endpoints** — prior three endpoints for 4090 / H200 / both H100 pods, plus **A100** at `38.80.152.72:31171` and **RTX 5090** at `157.157.221.29:54914` — **plus** the published Mac Mini arm64 binary. A100 and 5090 were **not** in the first four-device set; the 5090 endpoint is on a **different public /16** (`157.157` vs `38.80`). No datacenter or region claim is made from the endpoints alone. H100 A and B are **two devices and two installs** (distinct GPU UUIDs and `signer_fp`); they are **not** claimed as separate physical servers.

Stored receipts: [`evidence/v0.2.0-matrix/`](evidence/v0.2.0-matrix/) (`r_cpu.json` / `r_gpu.json` per device; A100 and RTX 5090 also store CSVs and run logs).

#### Same SKU, independent installs (H100 A vs B)

| | H100 (A) | H100 (B) |
|--|----------|----------|
| GPU UUID | `GPU-b3bcc572-…` | `GPU-d13abf0e-…` |
| Install `signer_fp` | (see `evidence/…/h100-a/meta.json`) | (see `evidence/…/h100-b/meta.json`) |
| `output_vector_sha256` | identical | identical |
| `lxq2_` seal | different | different |

**The number is install-independent; the receipt is not.** That is the design.

Also historical (v1-era binary, 2026-08-15 — separate from the measured A100 row above): multi-host RunPod x86 CPU (×3 hosts) + H100/H200 two-run GPU matrix under that older binary.

**Non-claim:** CPU↔GPU agreement is **measured** on the devices above for **`example_book.csv` only**. It is **not** a universal claim for every book or every SKU.

## The signed receipt (per-install; not a published constant)

Scheme: **`luxiquant-receipt-v2`**. Share token prefix: **`lxq2_`**.

The receipt is an Ed25519 seal over a canonical text payload that includes the output vector hash, input hash, build id, device fields (when GPU), and `host_simd_capability` (host CPU feature label — book pricing uses scalar deterministic kernels on every host).

**By design the receipt differs on every install and every device.** Do not compare your `lxq2_…` string to someone else’s. Compare **`output_vector_sha256`**.

`key_custody=file`: while custody is file-backed, **any process running as that user can mint a receipt with that key.** The seal binds the install, not a human identity.

What the receipt proves (use these words):

> The receipt proves that the install holding key *K* asserted this output vector hash, from this input hash, on this build, on this device, using this instruction-path label — and that the assertion was made no earlier than `t(beacon_round)`.

It does **not** prove the number is “correct” (that is textbook unit tests in the private engine). It is **tamper-evident**, not tamper-proof.

## OS binary notes

macOS: right-click → Open if Gatekeeper blocks (binary is **not Apple-notarized**).  
Linux: `chmod +x` then run.  
Always check `*.sha256` before trusting a download.
