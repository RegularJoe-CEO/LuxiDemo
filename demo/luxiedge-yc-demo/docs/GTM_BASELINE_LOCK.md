# LuxiEdge GTM baseline lock (version-100)

**Status:** Internal go-to-market lock — measured H100 prefill executor  
**Lane:** TRADE energy/throughput (not AUDIT bit-exact gold)  
**Date:** 2026-07-31  

This is the configuration to sell and defend until a stronger pack replaces it.

---

## Product configuration (TRADE energy path)

| Knob | Value |
|------|--------|
| Attention | Flash-class control (bridge) |
| Stack | Device-resident multi-layer (1× host upload / 1× download boundary) |
| Weights | FP16 residency on GPU |
| Model class | Qwen2-7B-Instruct |
| Sequence | 128 (prefill positions) |
| Batch | **16** primary · **32** scale |

**Do not sell:** AUDIT lane as the thr/J path. Keep dual-lane story: **AUDIT = trust**, **TRADE = thr + joules**.

---

## Locked metrics (multi-run formal pack — **authoritative**)

**Campaign:** 5 × 15s sustains each · H100 · Flash + device-resident + FP16  
**Source:** multi-run lock on version-100 GTM hour pack  

| Batch | Thr median (pos/s) | Thr min–max | Thr stdev | J/pos median | Det (5-run token match) | Flash |
|------:|-------------------:|------------:|----------:|-------------:|------------------------:|:-----:|
| **16** | **39,865** | 39,504–41,050 | ~671 | **0.0168** | **1.0** | yes |
| **32** | **42,967** | 42,635–43,570 | ~387 | **0.0160** | **1.0** | yes |

**Primary sell cell: B=16** (tight variance, commercial batch). **B=32** for scale.

### Peer H2H (matched prefill, same day — supporting)

| Batch | Luxi thr | vLLM thr | Luxi/vLLM thr | Luxi J/pos | vLLM J/pos |
|------:|---------:|---------:|--------------:|-----------:|-----------:|
| 16 | ~41.7k* | ~35.8k | **1.17×** | ~0.0171 | ~0.0190 |
| 32 | ~43.9k* | ~37.1k | **1.18×** | ~0.0158 | ~0.0182 |

\*H2H used 2×20s medians; multi-run 5×15s above is the thr lock for GTM quotes.

---

## GTM claims (allowed)

1. **Faster prefill throughput** than vLLM 0.25.x on H100 at S=128, B=16/32 under matched token accounting.  
2. **Lower board joules per prompt position** under that same protocol.  
3. **Deterministic dual-run behavior** on the Luxi TRADE path for fixed prompts (token-id agreement).  
4. **Batch scale holds efficiency** (thr rises B1→B32; J/pos falls; det stays 1.0).

## GTM claims (forbidden until more packs)

- Full multi-tenant continuous-batch *server* win vs vLLM OpenAI API  
- Decode-only thr crown  
- Wall-plug / PUE  
- AUDIT bit-exact on GPU TRADE kernels  
- “Always wins every shape”

---

## Serving surface

| Surface | Role for GTM |
|---------|----------------|
| `cuda_qwen7b_trade` sustain | **Money path** — thr + NVML energy |
| `serve_v05` HTTP | OpenAI-shaped API + **`GET /v1/gtm` scoreboard** — **do not** quote HTTP thr as TRADE thr |
| AUDIT receipts | Compliance / dual-lane story |

Commercial scripts:

- `scripts/gtm_demo_one_shot.sh` — TRADE sustain (money path)
- `scripts/gtm_serve_boot.sh` — HTTP + GTM energy mode
- `scripts/gtm_pod_commercial.sh` — TRADE + serve smoke on pod

Serve doc: [`GTM_COMMERCIAL_SERVE.md`](GTM_COMMERCIAL_SERVE.md).

---

## One-line pitch

> On H100, LuxiEdge’s energy path moves more prefill work per second than vLLM at commercial batch, uses fewer board joules per position, and keeps dual-run determinism—while a separate AUDIT lane protects bit-exact trust.

---

## Contact

e@ewaller.com
