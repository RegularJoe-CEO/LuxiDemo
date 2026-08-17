# Luxi Book v0.2.0 matrix evidence

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
| `rtx5090/` | GeForce RTX 5090 (Blackwell, compute capability 12.0) | **Strongest row.** Device code is **runtime JIT of embedded PTX built for sm_80** — **not** a compiled-in sm_120 / sm_120f target. Receipt `cuda_build`: `features=cuda; PTX sm_80; -fmad=false`. JIT on hardware newer than the PTX arch produced bit-identical f64 output. Endpoint `157.157.221.29:54914` (not used in the first four-device set; different /16 from A100). |
| `rtx4090/` | GeForce RTX 4090 (Ada sm_89) | Different GPU generation from Hopper |
| `a100/` | A100-SXM4-80GB (Ampere sm_80) | **Measured under v0.2.0** (`4a86d2a`). `host_simd_capability=avx2` — first avx2 x86 host in this matrix (pricing remains scalar-deterministic on every host). Endpoint `38.80.152.72:31171` (not used in the first four-device set). |
| `h200/` | H200 (Hopper sm_90) | |
| `h100-a/` | H100 80GB HBM3 | UUID `GPU-b3bcc572-…` · install fp in `meta.json` |
| `h100-b/` | H100 80GB HBM3 | UUID `GPU-d13abf0e-…` · **different device + install**, not claimed as a separate physical host |
| `macos-arm64/` | Apple Silicon CPU only | Published `luxi-book-macos-arm64` binary |

### RTX 5090 code path (do not blur)

The published CUDA binary embeds **PTX for sm_80**. On the RTX 5090 (CC 12.0), the driver **JIT-compiles that PTX at runtime** into device code for the local architecture. This is **not** an offline `nvcc -arch=sm_120` (or sm_120f) image baked into the binary. Why it matters: a **distinct code-generation path**, compiled at runtime on hardware newer than the kernel’s PTX target, still produced **bit-identical** f64 results (`output_vector_sha256` and `book_price_bits` match CPU and every other GPU row).

### Host independence (A100 + RTX 5090)

| Host | Endpoint | GPU UUID | Notes |
|------|----------|----------|--------|
| A100 | `38.80.152.72:31171` | `GPU-b2a79db3-…` | New to this matrix (not in the first four-device set) |
| RTX 5090 | `157.157.221.29:54914` | `GPU-93ed02cb-…` | New to this matrix; public IP on a **different /16** (`157.157` vs `38.80`) |

No datacenter or region claim is made from these endpoints alone.

H100 A and B shared a public IP (different pod ports) with **different container hostnames** and **different GPU UUIDs**. Public claim: **two devices / two installs**, not “two servers.”

Matrix = **6 NVIDIA GPUs** across **5 RunPod public endpoints** (prior three endpoints for 4090/H200/H100s, plus `38.80.152.72:31171` and `157.157.221.29:54914`) **+** Mac Mini for the published arm64 download.

## Non-claim

CPU↔GPU agreement is **measured** on these devices for this book. It is **not** a universal claim for every book or every SKU.
