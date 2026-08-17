# Luxi Book matrix — post-Φ fix (v0.2.1 measurement)

**Binary:** same `luxi-book-linux-x86_64-cuda` on every row  
`sha256 = 8cdebe5e53d464d5a292c71f653ea5ce65fd856e56fd458bafdc633164eedfbb`  
**Engine:** `0.2.0` with continued-fraction `erfc` for `|x| > 1.5` (Φ cliff fix).  
`git_sha` in receipts: `unknown` (on-pod build without embedded git).  
**CUDA build label:** `features=cuda; PTX sm_80; -fmad=false` (5090 runs via runtime JIT of sm_80 PTX).

## Constant hashes

| Book | `output_vector_sha256` | `book_price` |
|------|------------------------|--------------|
| `example_book.csv` (10 rows) | `4a21b1e708fa5c694bf48237df5e5bd3b94599e6273d07986283c6c6b8e3c97a` | `151.70016507843832` |
| `stress_book_a.csv` (237 rows) | `902667a1070b83bff57ac642cf16779d998b5a954c046c450b154237e9e196e2` | `488338590.9188194` |

Both hashes are **identical on all five devices, CPU and GPU**. Every `verify` returned PASS.

## Devices

| Dir | Device | CC | host_simd | Endpoint |
|-----|--------|----|-----------|----------|
| `h200/` | NVIDIA H200 | 9.0 | avx512 | `38.80.152.249:30444` |
| `h100/` | H100 80GB HBM3 | 9.0 | avx512 | `38.80.152.148:31978` |
| `a100/` | A100-SXM4-80GB | 8.0 | avx2 | `195.26.233.55:19592` |
| `rtx5090/` | GeForce RTX 5090 | 12.0 | avx512 | `213.173.111.147:32868` |
| `rtx4090/` | GeForce RTX 4090 | 8.9 | avx2 | `213.181.111.2:27577` |

## Stress B (validation)

15 degenerate rows, one at a time. **13 refused** on every host. Still accepted (by design of current gates): `BAD_SIGMA_SUBNORMAL`, `BAD_T_SUBNORMAL`.

## E3 answer (CPU vs CUDA)

Same defect path was fixed in both. After the fix, **CPU and GPU agree bit-for-bit** on example_book and on the full hostile stress_book_a across all five SKUs — including Blackwell (5090) via JIT of sm_80 PTX.

## Non-claim

Measured for these two CSVs on these five devices. Not a universal claim for every book or every SKU.
