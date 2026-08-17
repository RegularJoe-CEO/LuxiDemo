# Luxi Book — public try

Closed binaries from [luxi-quant-engine](https://github.com/RegularJoe-CEO/luxi-quant-engine) **v0.2.0** (build `b4645c2`). **No engine source. No NDA.**

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

This hash is **not** the signed receipt string. Two machines can share this hash and still produce **different** `lxq2_…` receipts (different install keys / device labels). That is intentional.

### Cross-box matrix

| Build | When | Boxes measured | Result |
|-------|------|----------------|--------|
| **v0.2.0** (`b4645c2`, this download) | 2026-08-16 | Mac Mini CPU (Apple ARM); Linux x86_64 CPU (default + CUDA-linked binary, CPU mode) | output hash + ATM_CALL as above |
| **v0.1.x pre-attestation** (prior download) | 2026-08-15 | Mac Mini CPU; RunPod x86 CPU (×3 hosts); A100 / H100 / H200 GPU (two runs each) | same **output hash** on those boxes |

The multi-GPU row is **historical (v1-era binary)**. It is not re-asserted for this v0.2.0 build until re-measured on those GPUs with this binary. **Not** a universal CPU↔GPU claim for every book or every SKU.

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
