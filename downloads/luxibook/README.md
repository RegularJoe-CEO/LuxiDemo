# Luxi Book — public try

Closed binaries from [luxi-quant-engine](https://github.com/RegularJoe-CEO/luxi-quant-engine) **v0.2.0**, built from engine commit `4a86d2a`— the value each binary reports as `git_sha`. The cross-device matrix under `evidence/v0.2.0-matrix/` was measured on the earlier v0.2.0 commit `b4645c2`; receipts from `b4645c2` verify under the `4a86d2a` download. **No engine source. No NDA.**

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
| [`evidence/v0.2.0-matrix/`](evidence/v0.2.0-matrix/) | all | stored `receipt.json` files behind the matrix |

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

This hash is **not** the signed receipt string. Two installs can share this hash and still produce **different** `lxq2_…` seals. That is intentional.

### Cross-box matrix (v0.2.0 · build `b4645c2` · same CUDA binary on GPU rows)

**What this measures:** CPU scalar gold path and the CUDA kernel (device port of `exp`/`ln`/`erf`/`normcdf`/`normpdf`, no vendor `libm` for those five) agree bit for bit on this book — including **Hopper** (H100/H200) and **Ada** (RTX 4090), i.e. different GPU generations.

| Device | Arch | Host CPU | GPU UUID | CPU | GPU | CPU↔GPU |
|--------|------|----------|----------|-----|-----|---------|
| **RTX 4090** | Ada sm_89 | EPYC 9254 | `GPU-4c59e6d9-…` | PASS | PASS | agree |
| H200 | Hopper sm_90 | Xeon 8568Y+ | `GPU-2d296bd2-…` | PASS | PASS | agree |
| H100 80GB (A) | Hopper sm_90 | Xeon 8462Y+ | `GPU-b3bcc572-…` | PASS | PASS | agree |
| H100 80GB (B) | Hopper sm_90 | Xeon 8462Y+ | `GPU-d13abf0e-…` | PASS | PASS | agree |
| Mac Mini (arm64 download) | — | Apple Silicon | — | PASS | n/a | — |

**Scope of the table:** 4 NVIDIA GPUs on **3 RunPod public endpoints** (RTX 4090 on `.248`, H200 on `.249`, both H100 pods on `.148` with different ports) **plus** the published Mac Mini arm64 binary. H100 A and B are **two devices and two installs** (distinct GPU UUIDs and `signer_fp`); they are **not** claimed as separate physical servers.

Stored receipts: [`evidence/v0.2.0-matrix/`](evidence/v0.2.0-matrix/) (`r_cpu.json` / `r_gpu.json` per device).

#### Same SKU, independent installs (H100 A vs B)

| | H100 (A) | H100 (B) |
|--|----------|----------|
| GPU UUID | `GPU-b3bcc572-…` | `GPU-d13abf0e-…` |
| Install `signer_fp` | (see `evidence/…/h100-a/meta.json`) | (see `evidence/…/h100-b/meta.json`) |
| `output_vector_sha256` | identical | identical |
| `lxq2_` seal | different | different |

**The number is install-independent; the receipt is not.** That is the design.

#### Historical A100 (v1-era binary, 2026-08-15)

A100 is **not** re-measured under v0.2.0. Keep this dated line only:

| Device | Build | Result |
|--------|-------|--------|
| A100-SXM4-80GB | pre-attestation binary, 2026-08-15 | same **output hash** as above (historical row; not re-asserted for this build) |

Also historical (same date / v1 binary): multi-host RunPod x86 CPU (×3 hosts) + H100/H200 two-run GPU matrix under that older binary.

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
