# SpaceX / xAI evidence

H100 NVL continuous-batch power trace + exact commands (2026-07-11):

**→ [evidence/spacex-xai-h100-2026-07-11/START_HERE.md](evidence/spacex-xai-h100-2026-07-11/START_HERE.md)**

Direct CSV: [power_trace_sustain_30m.csv](evidence/spacex-xai-h100-2026-07-11/power_trace_sustain_30m.csv)

---

## TRADE CUDA (GPU under load) — 2026-07-11

**→ [evidence/spacex-xai-h100-TRADE-cuda/START_HERE.md](evidence/spacex-xai-h100-TRADE-cuda/START_HERE.md)**

Prefill median ~177 W · decode ~169 W · J/token re-measured with pynvml under CUDA load.

---

## Baseline vs geodesic (same H100, same width) — 2026-07-11

**→ [evidence/spacex-xai-h100-BASELINE-vs-geo/START_HERE.md](evidence/spacex-xai-h100-BASELINE-vs-geo/START_HERE.md)**

Head-to-head single-layer J/token: geodesic TRADE vs PyTorch unfused FP16 + Flash SDPA + morph wedges.

---

## 12L stack H2H — TRADE vs PyTorch+Flash (2026-07-11)

**→ [evidence/spacex-xai-h100-STACK12-H2H/START_HERE.md](evidence/spacex-xai-h100-STACK12-H2H/START_HERE.md)**

Multi-layer prefill H2H on same H100 NVL. PT+Flash wins on thr and J/token; TRADE GPU is loaded (~177 W). Published honestly.
