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

  luxiedge-serve-macos-arm64    Inference serve + locked GTM scoreboard (demoted)
  luxiedge-serve-linux-x86_64
  *.sha256

  README.txt                    this file

LuxiRisk freebie binaries live under ../../luxirisk/dist/ (not this folder).
Numerical v3.0 demos (luxiedge-demo, luxi-tools) are on the GitHub v3.0 release.

----------------------------------------------------------------------------
A) Luxi Book (sale / professional try)
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

Measured receipt on example_book.csv only:
  4a21b1e708fa5c694bf48237df5e5bd3b94599e6273d07986283c6c6b8e3c97a
  (Mini CPU · Linux CPU · A100 · H100 · H200). Not a universal GPU claim.

Unsigned binaries: macOS right-click → Open.

----------------------------------------------------------------------------
B) LuxiEdge serve (inference scoreboard — not Book)
----------------------------------------------------------------------------

Product: LuxiEdge version-100 commercial demo binary
What this is: OpenAI-shaped HTTP server + locked GTM scoreboard
What this is not: Engine source, CUDA TRADE kernels, full model weights, Luxi Book

# macOS Apple Silicon
chmod +x luxiedge-serve-macos-arm64
./luxiedge-serve-macos-arm64 --bind 127.0.0.1:8787

# Linux x86_64
chmod +x luxiedge-serve-linux-x86_64
./luxiedge-serve-linux-x86_64 --bind 127.0.0.1:8787

Then:
  curl -s http://127.0.0.1:8787/health | python3 -m json.tool
  curl -s http://127.0.0.1:8787/v1/gtm | python3 -m json.tool
  curl -s -X POST http://127.0.0.1:8787/v1/completions \
    -H 'content-type: application/json' \
    -d '{"prompt":"Why measure joules per token?","max_tokens":24}'
  curl -s -X POST http://127.0.0.1:8787/v1/audit -d '{}'

GET /v1/gtm embeds the measured H100 multi-run lock (not live laptop thr).
Local generate path is a toy for instant API demos. Board joules ≠ wall-plug.

Verify:
  shasum -a 256 -c luxiedge-serve-macos-arm64.sha256

----------------------------------------------------------------------------
C) LuxiRisk freebie
----------------------------------------------------------------------------

See ../../luxirisk/ and release tag luxirisk-v0.2.
Retail/crypto CLI + lxr1_ receipts — not the option book.

Contact: e@ewaller.com · https://luxiedge.com
