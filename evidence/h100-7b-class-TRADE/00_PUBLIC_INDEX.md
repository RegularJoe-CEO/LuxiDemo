# Public: 7B-class TRADE on H100 NVL (public)

This pack is on **public** [LuxiDemo](https://github.com/RegularJoe-CEO/LuxiDemo) so external evaluators can open it without private-repo access.

**Start here:** [START_HERE.md](./START_HERE.md)

**Headline (product thr/energy  -  use these):**
- **~403 prefill tok/s** at sustain_seq=128
- **0.630 ± 0.002 J/token** (sustain-only pynvml board power)
- Median board **~254 W**
- Full **28-layer** Qwen2-7B-scale stack (h=3584)

Root `run_1..3` uses sequence length 5 and is classified as a microbenchmark,
not a product-level result.

Sibling packs in this repo: TRADE-cuda, STACK12-H2H, WNSM-free-ride, LONGCTX-scaling, BASELINE-vs-geo, 2026-07-11 sustain.
