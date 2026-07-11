# City block: null free-ride + device-resident decode/KV (H100 NVL)

**Date:** 2026-07-10  
**Branch:** `feat/phase5-commercial-launch` (Phases 1–4 complete)  
**Goal:** Same inference work, less electricity — cut HBM traffic and host tax, not plaque demos.  
**Commercial:** dual AGPL + commercial license · [pricing](pricing.md) · [docs hub](index.md)

## Executive summary

| Area | Result | Grade |
|------|--------|-------|
| WNSM null residual | **2.5×10⁻⁸** (was ~0.04–0.12) | A |
| **12-layer device-resident stack** | **1 H2D + pointer-swap residual + 1 D2H**; vs modular CUDA **max abs = 0** | **A** |
| Multi-layer free-ride bus | free-ride **faster** than OFF+side H2D | B+ |
| Parallel KV decode kernel | batch **~30–50×** vs host-step | A− |
| Device QKV + resident residual serve | batch path **~8× less J** than full recompute (64 new toks) | B+ |
| True AR sequential step | ~0.3 ms/step ungraphed; **CUDA Graph PATH_E** live on H100 (bit-exact) | B+ |
| Speculative + WNSM verifier | bit-exact vs sequential greedy (CPU AUDIT) | A |
| Quant paths FP8/INT8 | FP8 exact grid ≡ f32; INT8 Lane B; AUDIT→f32 force | A− |
| **v0.5 Usable Server (Phase 2)** | HTTP + continuous batch + paged KV + agent demo + dashboard | **A−** |
| **v1.0 Multi-Hardware (Phase 3)** | Metal / WebGPU / consumer CUDA / multi-shard / sparse+INT4 AUDIT | **A−** |
| **v2.0 Training + Reversible (Phase 4)** | Checkpoint recompute bit-exact; dual-run train; MoE; geodesic long-ctx | **A−** |
| vs production vLLM/fleet | **not claimed** | — |

## Full-stack J/token (honest money number — H100 NVL pynvml ≥6s)

Pure GPU sustain (`LUXI_STACK_JOULES_ONLY=1`). Formula: `J_job = median_W × ms_per_iter/1000`.

| Stage | Shape | median W | ms/iter | **J / job** | **J / token** |
|-------|-------|---------:|--------:|------------:|--------------:|
| **Prefill 12-layer device-resident** | seq=1024, h=768 | **172.2** | 74.28 | **12.79 J** | **0.0125 J/tok** |
| **Decode resident-batch** (attn KV stage) | 64 new @ ~2112 ctx | **162.5** | 2.96 | **0.480 J** | **0.0075 J/gen_tok** |
| **Total job** (12L prefill@1024 + 64 decode) | — | — | — | **13.27 J** | — |

Notes:
- **TRADE energy path:** `decoder_forward_cuda_device_resident` (default) — traffic **h2d=1, d2h=1, d2d_residual=0**; residual handoff proven by resident ≡ modular CUDA (**0 abs**).
- **AUDIT path (`LUXI_RECEIPT_AUDIT=1`):** same entrypoint uses pure-CPU multi-layer residual chain — **max abs vs CPU ≤ 5e-7** (bit-exact 0; unit test `multi_layer_audit_stack_bit_exact_vs_layer_loop`).
- TRADE geodesic vs CPU can drift ~0.3–0.5 (energy kernels); criterion 2 is the **AUDIT-aligned** stack, not TRADE vs CPU.
- Decode path is **single-layer attention KV** (not yet 12-layer AR decode).

## 1) Tight null of `w_proj`

- Method: f64 Cholesky of \(G = W^\top W\), project \(v \leftarrow v - W G^{-1} W^\top v\), MGS.
- Residual \(\|W^\top v\|/\|v\|\): **2.5e-8** (target was 1e-5).
- Code: `src/wnsm_null.rs` (`install_null_of_w_proj`, unit test enforces `< 1e-5`).

## 2) Device-resident multi-layer payload bus

- Shared `d_payload` / `borrow_payload` across layers.
- 12-layer seq=1024 pd=64: ON free-ride **48.8 ms** vs OFF **52.7 ms**.

## 3) Decode + KV (the energy lever)

### Root cause fixed

Old `waller_kv_decode` used **one thread per head** (~ms/step). Parallel kernel + batch grid.

### Paths (h=768, H100 NVL)

| Path | What | @ 2048+64 |
|------|------|-----------|
| A | Host ships QKV each step | ~0.24 ms/step |
| B | Host QKV batch (one H2D) | ~1.2 ms total |
| C | Device QKV from residual x | step ~0.33 ms; batch ~1.6 ms |
| **D** | **Residual on device**; zero mid H2D/D2H | seq ~19 ms; **batch ~1.4 ms**; bit-exact vs C |

### Job energy (pynvml, 64 new tokens @ ~2112 context)

`J = median_W × ms_per_iter`

| Mode | J / job | vs full |
|------|--------:|--------:|
| Full recompute | 3.92 J | 1× |
| Device step (H2D x each) | 2.36 J | 1.7× |
| Resident seq serve | 2.25 J | 1.7× |
| **Resident batch** | **0.48 J** | **8.2×** |

## Code map

| Piece | Path |
|-------|------|
| Null Cholesky | `src/wnsm_null.rs` |
| Parallel + batch KV decode, 1-token QKV, resident serve | `cuda_src/cuda_extras.cu`, `src/gpu/cuda.rs` |
| Layer APIs | `src/wnsm_transformer.rs` |
| Benches | `examples/cuda_decode_kv_bench.rs`, `cuda_wnsm_*_bench.rs` |
| Joules driver | `scripts/decode_joules_pod.py` |

### Key APIs

- `load_cuda_kv_prefill` / `load_cuda_kv_residuals`
- `forward_cuda_kv_step` / `_batch` (host QKV)
- `forward_cuda_kv_step_from_x` / `_batch_from_x` (device QKV)
- `forward_cuda_kv_serve(sequential, d2h)` — **zero mid-loop host transfer**
- **Exp C:** sequential serve uses **CUDA Graph** when `LUXI_CUDA_GRAPH≠0` (default on); bit-exact vs ungraphed (`max|graph−ungraph|=0`)
- `last_cuda_serve_used_graph()` / `cuda_graph_launch_count()` telemetry
- **CUDA 12 note:** must use 3-arg `cudaGraphInstantiate(exec, graph, flags)` (legacy 5-arg → `cudaErrorInvalidValue`)

### Exp C measured (H100 NVL, 2026-07-10)

| Shape | PATH_D seq (ungraph) | PATH_E graph | max\|E−D\| | used_graph |
|-------|---------------------:|-------------:|-----------:|:----------:|
| 256 prefill + 16 steps, h=64 | 0.534 ms | **0.494 ms** (~1.08×) | **0** | true |
| 2048 prefill + 64 steps, h=768 | 19.398 ms | **19.206 ms** (~1.01×) | **0** | true (9 launches) |

At long context the KV scan dominates; graph still correct and slightly faster. Prefer PATH_D **batch** (1.43 ms) when residuals are pre-known.

### Phase 1 additions (this session)

| Piece | Path |
|-------|------|
| CUDA Graph sequential serve | `src/gpu/cuda.rs` (`serve_loop_from_resident_x`) |
| Speculative + WNSM verifier | `src/speculative.rs`, `examples/wnsm_speculative_chat.rs` |
| Quant FP8/INT8 + AUDIT fallback | `src/quant_paths.rs` (`LUXI_QUANT_MODE`, `LUXI_RECEIPT_AUDIT=1`→f32) |
| AUDIT CI | `scripts/phase1_audit_ci.sh`, `.github/workflows/phase1-audit.yml` |
| Single-node demo | `scripts/v01_single_node_run.sh`, `examples/v01_production_inference_demo.rs` |

**v0.1 Production Inference Demo milestone (H100 NVL, measured):**

| Metric | Value |
|--------|------:|
| Prefill 12L @1024 | **0.0125 J/token** |
| Decode resident-batch | **0.0075 J/gen_token** |
| Full-stack job | **13.27 J** |

## Honest limits

1. Single-layer attention-stage energy for decode, not full multi-layer serving stack.
2. Sequential AR launch tax reduced via CUDA Graphs; residual H2D is not the main cost once kernel is fixed.
3. No claim vs FlashAttention attention-only or production vLLM.
4. Online-softmax tree merge ≈ 1e-3 abs vs serial in some cases; resident path bit-exact with device step.
5. Speculative protocol is AUDIT-proved on CPU/toy+WNSM stack; GPT-2 batched verify remains a research artifact (`gpt2_speculative`).

## Phase 2 — v0.5 Usable Server

See **[`docs/V05_USABLE_SERVER.md`](V05_USABLE_SERVER.md)**.

```bash
bash scripts/v05_usable_server.sh --smoke
cargo run --release --example serve_v05 -- 127.0.0.1:8787
cargo run --release --example agent_home_network
