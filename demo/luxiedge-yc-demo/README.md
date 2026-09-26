# LuxiEdge commercial demo (binary only - no source)

**Product:** LuxiEdge version-100  
**What this is:** OpenAI-shaped HTTP server (demo build, toy generate path)  
**What this is not:** Engine source, CUDA TRADE kernels, or full model weights  

## Quick start (any laptop)

```bash
# macOS Apple Silicon
chmod +x bin/luxiedge-serve-macos-arm64
./bin/luxiedge-serve-macos-arm64 --bind 127.0.0.1:8787

# Linux x86_64
chmod +x bin/luxiedge-serve-linux-x86_64
./bin/luxiedge-serve-linux-x86_64 --bind 127.0.0.1:8787
```

Then:

```bash
curl -s http://127.0.0.1:8787/v1/models | python3 -m json.tool
curl -s -X POST http://127.0.0.1:8787/v1/completions \
  -H 'content-type: application/json' \
  -d '{"prompt":"Why measure joules per token?","max_tokens":24}'
curl -s -X POST http://127.0.0.1:8787/v1/audit -d '{}'
```

## Scoreboard routes

The `/v1/gtm`, `/health` and `/dashboard` routes include static scoreboard fields compiled into this demo build, for API integration testing; they are not benchmark results. Measured results are in [`BENCHMARKS.md`](../../BENCHMARKS.md).

## Comparison against vLLM 0.25.1

Luxi runs Qwen2-7B on an H100 with bit-identical results at any batch size. On long prompts (2k to 32k tokens) it is 3–7% faster than vLLM 0.25.1 and uses 2–5% less energy per token. When generating tokens it matches vLLM's speed but uses 2–7% more energy per token. Against vLLM's own deterministic (batch-invariant) mode, Luxi is faster and uses less energy on every test, including 1.5–2.7× faster token generation with 16–50% less energy.

Tables, method and disclosure: [`BENCHMARKS.md`](../../BENCHMARKS.md) · brief: `docs/PUBLIC_H2H_PREFILL_ENERGY_BRIEF.md`

## Honest limits

- Local binary uses a **toy generate path** for instant API demos (receipts + energy scale).  
- Energy fields in completion responses are fixed per-token estimates, not live measurements.  
- Measured H100 results: [`BENCHMARKS.md`](../../BENCHMARKS.md).  
- Board joules ≠ facility wall-plug.  
- Not a claim of full multi-tenant OpenAI-server leadership vs every recipe.

## Verify download

```bash
shasum -a 256 -c bin/luxiedge-serve-macos-arm64.sha256
# or linux .sha256
```

## Contact

e@ewaller.com · https://luxiedge.com
