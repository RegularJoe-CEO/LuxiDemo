Luxi public downloads (this folder)
==================================

Layout
------
  luxibook/                     Luxi Book — primary Quant try (CSV options + SHA-256)
    luxi-book-macos-arm64
    luxi-book-linux-x86_64
    luxi-book-linux-x86_64-cuda   (NVIDIA; --mode gpu)
    example_book.csv
    *.sha256
    README.md

  luxi-replay-demo/             Luxi inference replay demo 1.0.1 (Linux x86_64, NVIDIA GPU or CPU)
    luxi-replay-demo-1.0.1-linux-x86_64.tar.gz
    luxi-replay-demo-1.0.1-linux-x86_64.tar.gz.sha256
    luxi-replay-demo-1.0.0-linux-x86_64.tar.gz          (previous release, same engine files)
    luxi-replay-demo-1.0.0-linux-x86_64.tar.gz.sha256
    LICENSE.txt  THIRD_PARTY_NOTICES.md  README.md

  luxiedge-serve-macos-arm64    Historical API toy (version-100 serve); not the current reasoning engine
  luxiedge-serve-linux-x86_64
  *.sha256

  README.txt                    this file

LuxiSize, the sizing calculator included with LuxiBook, has its binaries under
../luxirisk/dist/ (not this folder).
Numerical v3.0 demos (luxiedge-demo, luxi-tools) are on the GitHub v3.0 release. That release
describes 10-day-expiring demo binaries: historical, not current working packages, not inference.

This repository is binaries + evidence + markdown only.
The marketing website https://luxiedge.com is deployed from Replit — not from this repo.

----------------------------------------------------------------------------
A) LuxiBook (free demo, includes LuxiSize)
----------------------------------------------------------------------------

# Mac CPU
chmod +x luxibook/luxi-book-macos-arm64
shasum -a 256 -c luxibook/luxi-book-macos-arm64.sha256
./luxibook/luxi-book-macos-arm64 price \
  --book luxibook/example_book.csv --out report.csv --receipt receipt.json

# Linux CPU
./luxibook/luxi-book-linux-x86_64 price \
  --book luxibook/example_book.csv --out report.csv --receipt receipt.json

# Linux CUDA (live NVIDIA driver required)
./luxibook/luxi-book-linux-x86_64-cuda price \
  --book luxibook/example_book.csv --out report.csv --receipt receipt.json --mode gpu

Measured output hash on example_book.csv only:
  4a21b1e708fa5c694bf48237df5e5bd3b94599e6273d07986283c6c6b8e3c97a
  v0.2.1 matrix, 2026-08-16: H200 · H100 · A100 · RTX 5090 · RTX 4090, CPU and CUDA pricing
  paths, 20 signed receipts across two books (luxibook/evidence/v0.2.1-phi-fix-matrix/).
  Not a universal GPU claim, and separate from the November inference results.

Unsigned binaries: macOS right-click → Open.

----------------------------------------------------------------------------
B) Luxi inference replay demo 1.0.1 (current inference download)
----------------------------------------------------------------------------

Closed binary, evaluation license, engine source private. Linux x86_64, glibc 2.34+.
NVIDIA GPU (compute capability 8.0/8.6/8.9/9.0/12.0, driver 570+) or an x86_64 CPU (short replay only;
tested on AMD EPYC 7543, Intel Xeon Gold 6342 and one sandbox CPU). GPUs tested: RTX A6000 (1.0.1),
RTX 5090 and A40 (1.0.0, same engine files).
Full instructions, tested machines and identities: luxi-replay-demo/README.md

  Pinned direct link:
  https://github.com/RegularJoe-CEO/LuxiDemo/raw/88ebbe60311070f292cb81ba07a7132242ca694a/downloads/luxi-replay-demo/luxi-replay-demo-1.0.1-linux-x86_64.tar.gz

  cd luxi-replay-demo
  sha256sum -c luxi-replay-demo-1.0.1-linux-x86_64.tar.gz.sha256
  # c2114aa7bd6c47822bdf94de2f7d77f1aa773a32dcc3268072cfb979c2e5c8b3
  tar xzf luxi-replay-demo-1.0.1-linux-x86_64.tar.gz
  cd luxi-replay-demo-1.0.1-linux-x86_64
  sha256sum -c SHA256SUMS
  ./install.sh --gpu            # or --cpu
  ./luxi fetch-model            # 8.7 GB from Hugging Face, sha256-checked
  ./luxi demo --device gpu      # or --device cpu; checked against the signed Nov 9 H100 NVL record
  ./luxi full-suite --device gpu

  Previous release 1.0.0 (same engine files) stays in luxi-replay-demo/:
  https://github.com/RegularJoe-CEO/LuxiDemo/raw/32d85ac1891eba4d03b7dc365690acaea78b5aea/downloads/luxi-replay-demo/luxi-replay-demo-1.0.0-linux-x86_64.tar.gz
  # b1f6cf0b024f0270223807662ebc15d72191db4838c408bf6727400c5ec539c9

Private GPU-swap evaluation on your own workload: https://luxiedge.com/contact#gpu-swap-evaluation

----------------------------------------------------------------------------
C) LuxiEdge serve (historical API toy; not Book, not the current engine)
----------------------------------------------------------------------------

Product: LuxiEdge version-100 serve demo binary (July 2026)
What this is: OpenAI-shaped HTTP server with a toy generate path, kept for its API shape
What this is not: the current reasoning engine, the benchmarked engine, engine source,
CUDA TRADE kernels, full model weights, Luxi Book

# macOS Apple Silicon
chmod +x luxiedge-serve-macos-arm64
./luxiedge-serve-macos-arm64 --bind 127.0.0.1:8787

# Linux x86_64
chmod +x luxiedge-serve-linux-x86_64
./luxiedge-serve-linux-x86_64 --bind 127.0.0.1:8787

Then:
  curl -s http://127.0.0.1:8787/v1/models | python3 -m json.tool
  curl -s http://127.0.0.1:8787/v1/gtm | python3 -m json.tool
  curl -s -X POST http://127.0.0.1:8787/v1/completions \
    -H 'content-type: application/json' \
    -d '{"prompt":"Why measure joules per token?","max_tokens":24}'
  curl -s -X POST http://127.0.0.1:8787/v1/audit -d '{}'

Local generate path is a toy for instant API demos. Completion responses carry
a luxi block with the SHA-256 token_receipt of the generated tokens and the
request latency_ms. /v1/gtm returns build info, a determinism note and a link
to BENCHMARKS.md; /health includes build_version and the benchmarks link; the
/dashboard top card links to BENCHMARKS.md.
The toy path does not run the benchmarked engine. Historical September H100 results:
../BENCHMARKS.md. Board joules ≠ wall-plug.

Verify:
  shasum -a 256 -c luxiedge-serve-macos-arm64.sha256

----------------------------------------------------------------------------
D) LuxiSize (part of LuxiBook)
----------------------------------------------------------------------------

See ../luxirisk/ and release tag luxirisk-v0.2.
Sizing calculator CLI + lxr1_ receipts. Not the option book pricer, not the LuxiQuant Risk State Engine.
The ../luxirisk/dist/ binaries and the luxirisk-v0.2 release assets are different builds with
different digests. Check each with its own checksums (see ../DEMOS.md, LuxiSize section).

Contact: eric@luxiedge.com · https://luxiedge.com
