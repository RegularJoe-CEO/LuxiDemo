# Luxi Book matrix - post-Φ fix (v0.2.1 measurement)

**Do not merge this table with `evidence/v0.2.0-matrix/`.** These are **different RunPod hosts** from the v0.2.0 campaign. Example: the v0.2.1 RTX 4090 host reports `host_simd_capability=avx2`; the v0.2.0 4090 host reported `avx512`. Same SKU labels, different installs.

## Published download (authoritative)

Current public binaries are **engine v0.2.1**, `git_sha` **`02388f778d9017d251bb0fc905a82b89a2e94c9a`**, `git_dirty=false`.

| File | sha256 |
|------|--------|
| `luxi-book-macos-arm64` | `3564b86f2563265ce7a7211f4513a861c75c07d9841e0503f2f9f359a55def6c` |
| `luxi-book-linux-x86_64` | `b4c14b9e0ceddf86e7a518d36d9ea48e5d6b07c72c331294a90af2100cc53c29` |
| `luxi-book-linux-x86_64-cuda` | `ff4dc7b68445f2c653cc8ce746eabc1b81e802036d641228f353ca5e31624a33` |

CUDA binary embeds PTX (self-contained). Label: `features=cuda; PTX sm_80; -fmad=false` (5090 = runtime JIT of sm_80 PTX).

## Measurement note for receipts in this folder

The five-device run stored here was executed with an on-pod build that reported `engine_version=0.2.0` / `git_sha=unknown` while carrying the **same Φ math** as v0.2.1. The **output hashes below match the published v0.2.1 binaries** (re-confirmed on the published downloads). Prefer the published v0.2.1 `engine_version` / `git_sha` for new receipts.

## Constant hashes

| Book | `output_vector_sha256` | `book_price` |
|------|------------------------|--------------|
| `example_book.csv` (10 rows) | `4a21b1e708fa5c694bf48237df5e5bd3b94599e6273d07986283c6c6b8e3c97a` | `151.70016507843832` |
| `stress_book_a.csv` (237 rows) | `902667a1070b83bff57ac642cf16779d998b5a954c046c450b154237e9e196e2` | `488338590.9188194` |

Both hashes are **identical on all five devices, CPU and GPU**. Every `verify` returned PASS.

## Devices (v0.2.1 campaign only)

| Dir | Device | CC | host_simd | Endpoint |
|-----|--------|----|-----------|----------|
| `h200/` | NVIDIA H200 | 9.0 | avx512 | `38.80.152.249:30444` |
| `h100/` | H100 80GB HBM3 | 9.0 | avx512 | `38.80.152.148:31978` |
| `a100/` | A100-SXM4-80GB | 8.0 | avx2 | `195.26.233.55:19592` |
| `rtx5090/` | GeForce RTX 5090 | 12.0 | avx512 | `213.173.111.147:32868` |
| `rtx4090/` | GeForce RTX 4090 | 8.9 | **avx2** | `213.181.111.2:27577` |

## Stress B (validation)

15 degenerate rows, one at a time. **13 refused** on every host. Still accepted (by design of current gates): `BAD_SIGMA_SUBNORMAL`, `BAD_T_SUBNORMAL`.

## E3 answer (CPU vs CUDA)

Same defect path was fixed in both. After the fix, **CPU and GPU agree bit-for-bit** on example_book and on the full hostile stress_book_a across all five SKUs - including Blackwell (5090) via JIT of sm_80 PTX.

## Non-claim

Measured for these two CSVs on these five devices. Not a universal claim for every book or every SKU.
