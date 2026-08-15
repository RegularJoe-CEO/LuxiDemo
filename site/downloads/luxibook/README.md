# Luxi Book — public try (unsigned)

Closed binary from [luxi-quant-engine](https://github.com/RegularJoe-CEO/luxi-quant-engine). **No engine source. No NDA.**

## Files

| File | Platform |
|------|----------|
| `luxi-book-macos-arm64` | macOS Apple Silicon (CPU) |
| `luxi-book-macos-arm64.sha256` | checksum |
| `example_book.csv` | synthetic example book |

**Linux x86_64 CPU / CUDA:** build from the engine repo (this tree ships macOS first; Linux binaries land when we cut a multi-arch release).

```bash
git clone https://github.com/RegularJoe-CEO/luxi-quant-engine.git
cd luxi-quant-engine
cargo build --release --bin luxi-book
# NVIDIA:
cargo build --release --features cuda --bin luxi-book
```

## Commands

```bash
chmod +x luxi-book-macos-arm64
shasum -a 256 -c luxi-book-macos-arm64.sha256
./luxi-book-macos-arm64 price \
  --book example_book.csv --out report.csv --receipt receipt.json

# Linux + NVIDIA only:
./luxi-book price --book example_book.csv --out report.csv --receipt receipt.json --mode gpu
```

## Measured receipt (`example_book.csv` only)

`4a21b1e708fa5c694bf48237df5e5bd3b94599e6273d07986283c6c6b8e3c97a`

Same hash observed on Mac Mini CPU, RunPod x86 CPU, A100, H100, H200 (two GPU runs each).  
**Not** a universal CPU↔GPU claim.

ATM_CALL: `10.4505835721856215`

Unsigned: macOS right-click → Open on first run.
