# LuxiEdge commercial demo (binary only - no source)

**Product:** LuxiEdge version-100  
**What this is:** OpenAI-shaped HTTP server + locked GTM scoreboard  
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
curl -s http://127.0.0.1:8787/health | python3 -m json.tool
curl -s http://127.0.0.1:8787/v1/gtm | python3 -m json.tool
curl -s -X POST http://127.0.0.1:8787/v1/completions \
  -H 'content-type: application/json' \
  -d '{"prompt":"Why measure joules per token?","max_tokens":24}'
curl -s -X POST http://127.0.0.1:8787/v1/audit -d '{}'
open http://127.0.0.1:8787/dashboard   # or browser
```

## What the scoreboard proves

`GET /v1/gtm` embeds the **measured H100 multi-run lock** (not live laptop thr):

| Cell | Thr median (pos/s) | Board J/pos | Det |
|------|-------------------:|------------:|----:|
| B16 | ~39,865 | ~0.0168 | 1.0 |
| B32 | ~42,967 | ~0.0160 | 1.0 |

Matched prefill vs vLLM: **~1.17 to 1.18×** thr · **~10 to 14%** lower board J/pos.

Details: `docs/PUBLIC_GTM_ONE_PAGER.md` · `docs/PUBLIC_H2H_PREFILL_ENERGY_BRIEF.md` · `evidence/MULTI_RUN_LOCK_SLIM.json`

## Honest limits

- Local binary uses a **toy generate path** for instant API demos (receipts + energy scale).  
- **Money thr/J** were measured on NVIDIA H100 with the TRADE executor - see scoreboard + docs.  
- Board joules ≠ facility wall-plug.  
- Not a claim of full multi-tenant OpenAI-server leadership vs every recipe.

## Verify download

```bash
shasum -a 256 -c bin/luxiedge-serve-macos-arm64.sha256
# or linux .sha256
```

## Contact

e@ewaller.com · https://luxiedge.com
