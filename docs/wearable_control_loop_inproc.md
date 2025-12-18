# Wearable Control Loop (In-Process) — 60Hz / 512f / Heavy Expr — Multi-Trial
**Short name:** WCL_INPROC_60HZ_512_HEAVY_MTRIAL  
**Where:** `~/Luxi-full` (all local)

## Purpose
Measure deadline misses + latency for a wearable-style fixed-rate loop when LuxiEdge evaluation is called **in-process** (no HTTP/Warp). Use this for the “NVG/thermal device loop” performance claim; HTTP mode is for integration/demos.

## Test Definition (locked)
- Rate: **60 Hz** → budget **16.667 ms**
- Per-trial duration: **3600 ticks** (60s) + **60 warm-up** ticks
- Payload: **512 floats**
- Expression:
  `tanh(log(sqrt(x*x+1.0))+exp(-x))*sin(x)+cos(x)`
- Evaluator path: matches `src/bin/l4_benchmark.rs` logic (Rhai compile once; eval per element)

## Files
- In-process runner: `src/bin/l4_control_loop.rs`
- Multi-trial helper (local temp): `/tmp/l4_inproc_multi.rs` → `/tmp/l4_inproc_multi`
- Doc (this file): `docs/wearable_control_loop_inproc.md`
- Results folder: `results/`

## Run: single trial (1 minute)
```bash
cd ~/Luxi-full
cargo run --release --bin l4_control_loop
