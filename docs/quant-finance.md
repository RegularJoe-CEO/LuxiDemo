# Luxi for Quantitative Finance

## What is public today

**Luxi Book** is the professional Quant try without an NDA:

- CSV position book → European **Black-Scholes / Black-76** prices
- Five Greeks
- **SHA-256** receipt over the canonical f64 little-endian output vector
- Closed binaries (macOS ARM64 CPU, Linux x86_64 CPU, Linux x86_64 CUDA)
- You bring `T`, `r`, and `σ` — no live feed, no IV surface, no VaR, no fund desk

**LuxiRisk** is a separate **freebie**: offline crypto / retail liquidation,
position size, and stop-loss math with Ed25519 `lxr1_` calculation receipts.
It is not the institutional option book.

Older **numerical** demos (`luxiedge-demo` REST `/evaluate`, operator validate)
still ship as supporting math surfaces. They are **not** Luxi Book.

## How to run Luxi Book

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

Measured on **`example_book.csv` only** (2026-08-15):

| Field | Value |
|-------|--------|
| Receipt | `4a21b1e708fa5c694bf48237df5e5bd3b94599e6273d07986283c6c6b8e3c97a` |
| ATM_CALL | `10.4505835721856215` |
| Boxes | Mac Mini CPU · RunPod x86 CPU · A100 · H100 · H200 |

**Non-claim:** this book, these boxes, this kernel — not “all GPUs always match.”

Downloads: [`downloads/luxibook/`](../downloads/luxibook/)  
How to run: [`DEMOS.md`](../DEMOS.md)

## Why receipts matter

Same inputs → same receipt on the machines you measured. Store the hash with
the run. Later, re-run the same CSV and contract; if the receipt matches, the
published vector matches.

That is **scoped reproducibility**, not a guarantee that every future GPU,
driver, or book shape will hash-match without re-measurement.

## What we do not claim on this page

| Claim | Status |
|-------|--------|
| Universal bit-exact results on every CPU and GPU | **No** — only measured boxes for Book |
| Desk VaR / full risk engine / live market data | **No** |
| LuxiRisk = institutional Quant | **No** — freebie retail/crypto CLI |
| TestFort 286B ops/s as option pricing thr | **No** — separate numerical suite |
| `risk-pipeline --synthetic` as product face | **No** — internal/synthetic only |

## Supporting numerical REST (secondary)

The v3.0 `luxiedge-demo` binary still exposes expression evaluation, e.g.:

```bash
curl -X POST http://127.0.0.1:9090/evaluate \
  -H "Content-Type: application/json" \
  -d '{"expr":"normcdf(x)","values":[-2.0,-1.0,0.0,1.0,2.0],"precision":"f32"}'
```

That path is useful for operator smoke and historical numerical receipts. It is
**not** the Luxi Book product surface. See [`DEMOS.md`](../DEMOS.md).

## Getting started

| Goal | Link |
|------|------|
| Download Book | [`downloads/luxibook/`](../downloads/luxibook/) |
| Freebie risk CLI | [luxirisk/](../luxirisk/) |
| Measured tables | [RESULTS.md](../RESULTS.md) |
| Full catalog | [DEMOS.md](../DEMOS.md) |
| Architecture | [LUXI_SYSTEM.md](../LUXI_SYSTEM.md) |

## Contact

Design partners / commercial: e@ewaller.com · [luxiedge.com](https://luxiedge.com)
