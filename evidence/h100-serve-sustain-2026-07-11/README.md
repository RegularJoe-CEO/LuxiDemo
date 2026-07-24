# H100 NVL continuous-batch sustain  -  power trace + exact commands

**Date:** 2026-07-11 (UTC)  
**Host GPU:** 1× NVIDIA H100 NVL (UUID `GPU-66f4edbd-61e6-9d1c-2690-11a0f3c11c3f`)  
**Driver / CUDA (pod):** 580.159.03 / 13.0  
**Power limit:** 310 W (min 200 / max 400) · **MIG:** Disabled  
**Repo commit (approx):** `dfc7144` on `feat/long-train-live-metrics` · PR https://github.com/RegularJoe-CEO/attention-transformer-v2/pull/13

---

## What “live power” means (precise)

| Question | Answer |
|----------|--------|
| Source | `nvidia-smi --query-gpu=power.draw --format=csv,noheader,nounits` every approximately 500 ms |
| Scope | **Single GPU board instantaneous power.draw** (NVML path via nvidia-smi CLI) |
| Whole pod / wall plug? | **No** |
| Delta above idle? | **No**  -  absolute board watts, not subtracted |
| During this serve run | The GPU was **not heavily utilized**. This was a CPU continuous-batch path, not a CUDA inference path. Board power remained approximately **63.5 to 64.1 W**, similar to the idle sample. |

**Idle baseline (post-run, 30 s, 0.5 s sample):** median **63.65 W** (min 63.63 / max 63.67)  -  see `idle_power_30s.txt`.

**Measurement note:** the **63.74 W median under load is board-idle-class
power**, not a measurement of a fully utilized H100. The **0.0125 / 0.0075
J/token** values in responses are constants from earlier H100 measurements
applied in software. They are not calculated from `∫ power dt` during this
serve run.

---

## Exact benchmark commands used that night

```bash
# On RunPod H100 NVL node, repo root: attention-transformer-v2
source $HOME/.cargo/env
export PATH=$HOME/.cargo/bin:/usr/local/cuda/bin:$PATH

# Environment
export LUXI_SERVE_THREADS=64
export LUXI_RECEIPT_AUDIT=1
export LUXI_TRADE_ENERGY=1
# (CUDA_ARCH not set for this serve binary  -  serve_v05 is CPU continuous-batch + nvidia-smi power sampling)

# Build / run server
cargo build --release --example serve_v05
./target/release/examples/serve_v05 0.0.0.0:8787
# server logs: live power: ~63.x W across 1 GPU(s) (nvidia-smi)

# Sustained load (30 minutes)
python3 scripts/serve_sustain_load.py \
  --base http://127.0.0.1:8787 \
  --minutes 30 \
  --concurrency 32 \
  --ctx-min 512 \
  --ctx-max 4096 \
  --max-tokens 32 \
  --metrics-interval 10 \
  --out serve_sustain_30m.json
```

**Workload definition for the ~377.6 req/s number:**
- Concurrency: **32** client threads  
- Context ladder (cycled): **512 / 1024 / 2048 / 4096** prompt chars ≈ tokens (byte-level toy tokenizer)  
- Decode: **32** tokens / request  
- Wall: **1800.1 s** · OK: **679 684** · fail: **21** · **~377.6 req/s**  
- Server p50/p99 latency (Prometheus): **2.48 / 4.47 ms**  
- Avg prefill ≈ **1916** tokens/req · avg decode ≈ **32** tokens/req (from lifetime counters)

**Model used by this serve path:**
- **Not** GPT-2 124M weights  
- Deterministic **toy** continuous-batch engine (`ServeEngine` / toy chat) for AUDIT + serving shape  
- Energy fields on responses use **city-block constants**, not live ∫P  

---

## Files in this folder

| File | Contents |
|------|----------|
| **`power_trace_sustain_30m.csv`** | **Primary power trace**  -  180 rows, ~every 10 s over 30 min (`luxi_gpu_power_watts`) |
| `serve_sustain_30m.json` | Full summary + raw `metrics_series` (same power samples as JSON) |
| `metrics_sustain_final.txt` | Final Prometheus `/metrics` scrape after sustain |
| `serve_sustain_server.log` | Server boot log (includes `live power: … nvidia-smi`) |
| `idle_power_30s.txt` | Post-run idle board power samples (median ≈ 63.65 W) |
| `nvidia_smi_power.txt` | `nvidia-smi -q -d POWER` (limits: 310 W, MIG disabled) |
| `nvidia_smi_query.csv` / `nvidia_smi_full.txt` | GPU inventory snapshot |
| `gpu_inventory_sustain.txt` | `nvidia-smi -L` |
| `workload_definition.json` | Structured workload numbers for the 377.6 req/s run |
| `long_train_2000.log` | Companion: 2000-step AdamW train + NVML joules JSON |
| `long_train_dual50.log` | Companion: dual-run determinism match @50 steps |

---

## Companion train command (separate run, same pod class)

```bash
export LUXI_RECEIPT_AUDIT=1 LUXI_TRADE_ENERGY=1
# 12L / h=768 / heads=12 / mlp=3072 residual-MLP + AdamW + corpus.txt  -  NOT full GPT-2 124M
python3 scripts/scale_train_joules_pod.py 2000
```

Train `saved_J` is **activation-checkpoint HBM model energy** (materialize vs recompute), separate from the 63.74 W serve board reading.
