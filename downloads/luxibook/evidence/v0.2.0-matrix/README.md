# Luxi Book v0.2.0 matrix evidence

**Build:** engine `0.2.0` · commit `b4645c27a07c32bf2622f33d1a8981d50c967a02` · same `luxi-book-linux-x86_64-cuda` binary throughout GPU rows.  
**Book:** `example_book.csv` only.  
**Constant output hash on every row / mode:**

`4a21b1e708fa5c694bf48237df5e5bd3b94599e6273d07986283c6c6b8e3c97a`

ATM_CALL = `10.4505835721856215`

Each directory holds `r_cpu.json` and (where GPU) `r_gpu.json` from those runs, plus `meta.json` on GPU pods.

## Layout

| Directory | Device | Notes |
|-----------|--------|--------|
| `rtx4090/` | GeForce RTX 4090 (Ada sm_89) | Lead row — different GPU generation from Hopper |
| `h200/` | H200 (Hopper sm_90) | |
| `h100-a/` | H100 80GB HBM3 | UUID `GPU-b3bcc572-…` · install fp in `meta.json` |
| `h100-b/` | H100 80GB HBM3 | UUID `GPU-d13abf0e-…` · **different device + install**, not claimed as a separate physical host |
| `macos-arm64/` | Apple Silicon CPU only | Published `luxi-book-macos-arm64` binary |

H100 A and B shared a public IP (different pod ports) with **different container hostnames** and **different GPU UUIDs**. Public claim: **two devices / two installs**, not “two servers.” Matrix = **4 GPUs across 3 RunPod public endpoints** (+ Mac Mini for the published arm64 download).

## Non-claim

CPU↔GPU agreement is **measured** on these devices for this book. It is **not** a universal claim for every book or every SKU.
