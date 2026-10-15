# LuxiBook v0.2.0 matrix evidence

**Build:** engine `0.2.0` · same published `luxi-book-linux-x86_64-cuda` binary identity (`output_vector_sha256` below) on every GPU row.  
**Measured on:** commit `b4645c2` (RTX 4090, H200, H100 A/B) and commit `4a86d2a` (A100, RTX 5090). Receipts from either verify under the current download.  
**Book:** `example_book.csv` only.  
**Constant output hash on every row / mode:**

`4a21b1e708fa5c694bf48237df5e5bd3b94599e6273d07986283c6c6b8e3c97a`

ATM_CALL = `10.4505835721856215`  
`book_price` bits = `343698c067f66240` (decimal `151.70016507843832`)

Each directory holds `r_cpu.json` and (where GPU) `r_gpu.json` from those runs, plus `meta.json` on GPU pods. A100 and RTX 5090 also store the matching CSVs and price/verify logs from the measured runs.

## Layout

| Directory | Device | Notes |
|-----------|--------|--------|
| `rtx5090/` | GeForce RTX 5090 (Blackwell, compute capability 12.0) | **Strongest row.** Device code is **runtime JIT of embedded PTX built for sm_80** - **not** a compiled-in sm_120 / sm_120f target. Receipt `cuda_build`: `features=cuda; PTX sm_80; -fmad=false`. JIT on hardware newer than the PTX arch produced bit-identical f64 output. |
| `rtx4090/` | GeForce RTX 4090 (Ada sm_89) | Different GPU generation from Hopper |
| `a100/` | A100-SXM4-80GB (Ampere sm_80) | **Measured under v0.2.0** (`4a86d2a`). `host_simd_capability=avx2` - first avx2 x86 host in this matrix (pricing remains scalar-deterministic on every host). The historical endpoint is intentionally omitted from this public evidence page. |
| `h200/` | H200 (Hopper sm_90) | |
| `h100-a/` | H100 80GB HBM3 | Separate recorded device and install |
| `h100-b/` | H100 80GB HBM3 | Separate recorded device and install; not claimed as a separate physical host |
| `macos-arm64/` | Apple Silicon CPU only | Published `luxi-book-macos-arm64` binary |

### RTX 5090 code path (do not blur)

The published CUDA binary embeds **PTX for sm_80**. On the RTX 5090 (CC 12.0), the driver **JIT-compiles that PTX at runtime** into device code for the local architecture. This is **not** an offline `nvcc -arch=sm_120` (or sm_120f) image baked into the binary. Why it matters: a **distinct code-generation path**, compiled at runtime on hardware newer than the kernel’s PTX target, still produced **bit-identical** f64 results (`output_vector_sha256` and `book_price_bits` match CPU and every other GPU row).

### Host independence (A100 + RTX 5090)

The A100 and RTX 5090 rows came from separate provider instances. Connection endpoints and device identifiers are intentionally omitted from this public page.

H100 A and B are recorded as two devices and two installs. They are not claimed as two physical servers.

Matrix = **6 NVIDIA GPUs** across recorded provider instances **+** Mac Mini for the published arm64 download.

## Non-claim

CPU↔GPU agreement is **measured** on these devices for this book. It is **not** a universal claim for every book or every SKU.
