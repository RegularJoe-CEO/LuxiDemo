# Luxi for Quantitative Finance

## Official product boundary

**LuxiQuant Risk State Engine, Risk Runtime v0.4.0**, is LuxiEdge's only sellable product. It is offered only through a paid confidential evaluation under NDA. It is proprietary, has no public download, and has no public price. It provides deterministic authenticated risk state as a scenario-by-position matrix changes. Customer pricing, market data, scenarios, and methodology remain under customer control. Authoritative rebuilds remain comparators and reconciliation paths. [Request a confidential evaluation](https://luxiedge.com/pilot).

## Free public demos

**LuxiBook v0.2.1** is the free option-book pricer demo:

- CSV position book → European **Black-Scholes / Black-76** prices
- Five Greeks
- **SHA-256** receipt over the canonical f64 little-endian output vector
- Closed binaries (macOS ARM64 CPU, Linux x86_64 CPU, Linux x86_64 CUDA)
- You bring `T`, `r`, and `σ` - no live feed, no IV surface, no VaR, no fund desk

**LuxiRisk v0.2** is a separate **free legacy demo calculator** (not the LuxiQuant Risk State Engine): offline liquidation,
position size, and stop-loss math with Ed25519 `lxr1_` calculation receipts.
It is not the institutional option book.

Older **numerical** demos (`luxiedge-demo` REST `/evaluate`, operator validate)
still ship as supporting math surfaces. They are **not** LuxiBook.

## How to run LuxiBook

```bash
# Mac CPU
chmod +x downloads/luxibook/luxi-book-macos-arm64
./downloads/luxibook/luxi-book-macos-arm64 price \
  --book downloads/luxibook/example_book.csv \
  --out report.csv --receipt receipt.json

# Linux CPU
./downloads/luxibook/luxi-book-linux-x86_64 price \
  --book downloads/luxibook/example_book.csv \
  --out report.csv --receipt receipt.json

# Linux CUDA (NVIDIA driver required; no silent CPU fallback)
./downloads/luxibook/luxi-book-linux-x86_64-cuda price \
  --book downloads/luxibook/example_book.csv \
  --out report.csv --receipt receipt.json --mode gpu
```

Measured on **`example_book.csv` only**:

| Field | Value |
|-------|--------|
| Output vector SHA-256 | `4a21b1e708fa5c694bf48237df5e5bd3b94599e6273d07986283c6c6b8e3c97a` |
| ATM_CALL | `10.4505835721856215` |
| v0.2.0 boxes (2026-08-16) | Mac Mini CPU · Linux x86_64 CPU |
| Historical multi-GPU (2026-08-15) | Mini · RunPod x86 (×3) · A100 · H100 · H200 - same **hash** |

**Non-claim:** this book, these boxes, this kernel - not “all GPUs always match.”  
Signed `lxq2_…` receipts are **per-install** (not a constant to publish).

Downloads: [`downloads/luxibook/`](../downloads/luxibook/)  
How to run: [`DEMOS.md`](../DEMOS.md)

## Why the output hash matters

Same inputs → same **output vector hash** on the machines you measured. Store the
hash with the run. Later, re-run the same CSV; if the hash matches, the published
vector matches. The Ed25519 seal binds that hash to an install and is not itself
expected to match across machines.

That is **scoped reproducibility**, not a guarantee that every future GPU,
driver, or book shape will hash-match without re-measurement.

## What we do not claim on this page

| Claim | Status |
|-------|--------|
| Universal bit-exact results on every CPU and GPU | **No** - only measured boxes for Book |
| Desk VaR / full risk engine / live market data | **No** |
| LuxiRisk v0.2 = LuxiQuant Risk State Engine | **No** - free legacy demo calculator |
| TestFort 286B ops/s as option pricing thr | **No** - separate numerical suite |
| `risk-pipeline --synthetic` as product face | **No** - internal/synthetic only |

## Supporting numerical REST (secondary)

The v3.0 `luxiedge-demo` binary still exposes expression evaluation, e.g.:

```bash
curl -X POST http://127.0.0.1:9090/evaluate \
  -H "Content-Type: application/json" \
  -d '{"expr":"normcdf(x)","values":[-2.0,-1.0,0.0,1.0,2.0],"precision":"f32"}'
```

That path is useful for operator smoke and historical numerical receipts. It is
**not** the LuxiBook product surface. See [`DEMOS.md`](../DEMOS.md).

## Getting started

| Goal | Link |
|------|------|
| Download Book | [`downloads/luxibook/`](../downloads/luxibook/) |
| LuxiRisk legacy demo calculator | [luxirisk/](../luxirisk/) |
| Measured tables | [RESULTS.md](../RESULTS.md) |
| Full catalog | [DEMOS.md](../DEMOS.md) |
| Architecture | [LUXI_SYSTEM.md](../LUXI_SYSTEM.md) |

## Contact

Confidential evaluation: [luxiedge.com/pilot](https://luxiedge.com/pilot) · Proof: [luxiedge.com/proof](https://luxiedge.com/proof)
