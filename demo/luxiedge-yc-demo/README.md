# LuxiEdge commercial demo (binary only - no source)

**Historical inference demo:** LuxiEdge version-100. This is not the LuxiQuant Risk State Engine or a current paid product.  
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
curl -s http://127.0.0.1:8787/v1/gtm | python3 -m json.tool
curl -s -X POST http://127.0.0.1:8787/v1/completions \
  -H 'content-type: application/json' \
  -d '{"prompt":"Why measure joules per token?","max_tokens":24}'
curl -s -X POST http://127.0.0.1:8787/v1/audit -d '{}'
```

## Routes

| Route | Returns |
|-------|---------|
| `GET /v1/gtm` | Build info (crate, version, target), determinism note, link to BENCHMARKS.md |
| `GET /health` | Liveness, `build_version`, `benchmarks` link |
| `GET /dashboard` | Live counters for this process; top card links to BENCHMARKS.md |
| `POST /v1/completions` · `/v1/chat/completions` | OpenAI-shaped completion plus a `luxi` block with `token_receipt` (SHA-256 of the generated tokens) and `latency_ms` |
| `POST /v1/audit` | Dual-run determinism self-check |

Measured results are in [`BENCHMARKS.md`](../../BENCHMARKS.md).

## Comparison against vLLM 0.25.1

On the 2026-09-25 H100 Qwen2-7B-Instruct fp16 generation tests, Luxi matched normal vLLM 0.25.1 speed within 2% and used 2–7% more GPU-board energy per output token. On 2k–32k prompts, Luxi was 3–7% faster and used 2–5% less GPU-board energy per prompt token. Luxi produced bit-identical outputs across the stated tested batch sizes and repeats. Against vLLM batch-invariant mode it was faster in every tested cell, including 1.5–2.7× faster generation with 16–50% less GPU-board energy.

Tables, method and disclosure: [`BENCHMARKS.md`](../../BENCHMARKS.md) · brief: `docs/PUBLIC_H2H_PREFILL_ENERGY_BRIEF.md`

## Honest limits

- Local binary uses a **toy generate path** for instant API demos (token receipts + latency).  
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
