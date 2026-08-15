# Luxi Book — public try (unsigned)

Closed binaries from [luxi-quant-engine](https://github.com/RegularJoe-CEO/luxi-quant-engine). **No engine source. No NDA.**

## Downloads

| File | Platform | Notes |
|------|----------|--------|
| `luxi-book-macos-arm64` | macOS Apple Silicon | CPU only |
| `luxi-book-linux-x86_64` | Linux x86_64 | CPU only |
| `luxi-book-linux-x86_64-cuda` | Linux x86_64 + NVIDIA | supports `--mode gpu` |
| `example_book.csv` | all | synthetic example book |
| `*.sha256` | all | checksums |

There is **no** macOS GPU binary. Use the CUDA Linux build only on machines with an NVIDIA driver.

## Commands

```bash
# Mac
chmod +x luxi-book-macos-arm64
shasum -a 256 -c luxi-book-macos-arm64.sha256
./luxi-book-macos-arm64 price --book example_book.csv --out report.csv --receipt receipt.json

# Linux CPU
chmod +x luxi-book-linux-x86_64
./luxi-book-linux-x86_64 price --book example_book.csv --out report.csv --receipt receipt.json

# Linux CUDA (NVIDIA required at runtime)
chmod +x luxi-book-linux-x86_64-cuda
./luxi-book-linux-x86_64-cuda price --book example_book.csv --out report.csv --receipt receipt.json --mode gpu
```

## Measured receipt (`example_book.csv` only)

`4a21b1e708fa5c694bf48237df5e5bd3b94599e6273d07986283c6c6b8e3c97a`

Same hash observed on Mac Mini CPU, RunPod x86 CPU, A100, H100, H200 (two GPU runs each).  
**Not** a universal CPU↔GPU claim.

ATM_CALL: `10.4505835721856215`

Unsigned: macOS right-click → Open; Linux should run as-is after `chmod +x`.
