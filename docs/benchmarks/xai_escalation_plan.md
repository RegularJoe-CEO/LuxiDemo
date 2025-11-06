# Luxi Edge xAI Escalation Plan (Ops/J North Star)

## Summary
We will deliver reproducible, powered 20s steady‑state ops/J wins, then extend to GPU and cluster scale. We avoid sharing full scripts; instead we provide a minimal open-source “ROC harness” so xAI can independently run the same methodology.

## Canonical metric
- 20s steady‑state ops/J from powermetrics (macOS) or NVML + CPU meters (Linux).
- Compute-only throughput is de-emphasized (kept only for latency/shape context).

## Workload
Scalar, elementwise evaluation per x of:
$$
\phi(x;a)=
\begin{cases}
\sin(x)+a\,x^2,& x<0\\\\
\log(1+x)-\sqrt{|x|}+0.1\,x^3,& x\ge 0
\end{cases}
$$
Operation = one evaluation of $\phi$ per scalar x.

## Current (Apple M1 Pro) reference
- Luxi (TCP, 20s powered): ~20.9k ops/J, ~156k samples/s
- Luxi (UDS, 20s powered): ~20.9k ops/J, ~158k samples/s
- Baselines (in-process PyTorch/TF) are not apples-to-apples for microservice overhead; we keep those for reference only.

## Targets
- T+72h (“Drop A”, macOS M1):
  - ≥3–5× ops/J vs current Luxi microservice (to ~60–100k ops/J)
  - Changes: gRPC/protobuf transport, compile‑once/evaluate‑many (expr_id), end‑to‑end float32, persistent channels, preallocated buffers
- Early next week (“Drop B”, Linux + NVIDIA GPU):
  - Single-GPU T4/A10/A100; 20s steady‑state ops/J via NVML (+ CPU meters)
  - Goal: within 5–15% of in‑process baseline first pass; then parity/better after SIMD/JIT
- Following week (“Drop C”, Scale-out):
  - 1→4→8 nodes; report cluster‑aggregate ops/J and per‑node ops/J

## Engineering changes (near-term)
- Transport: gRPC/protobuf (binary f32 arrays) with persistent channels
- Compile once / evaluate many: POST /compile → expr_id; reuse in /evaluate
- Payloads: float32 throughout; binary buffers; round trip validation
- Batching: server-side streaming or batched evaluate to amortize framing
- Hot path: preallocated buffers, minimized allocations, zero-copy where feasible
- Math core: enable SIMD with wide/simdeez; AST caching

## Repro (no full scripts)
We will publish a tiny, public **luxi-bench-min** harness that:
- Generates φ(x;a), runs Baselines (PyTorch/TF) and RPC (HTTP+JSON and gRPC),
- Captures energy via powermetrics (macOS) or NVML (Linux/NVIDIA) (+ CPU meters),
- Emits CSV + Markdown summary matching our report fields.

This provides independent, repeatable methodology without exposing Luxi internals.

## Deliverables (stable in-repo paths)
- Report: docs/benchmarks/xai_integration.md (overwritten in-place)
- Transport comparisons: docs/benchmarks/torch_luxi_{tcp,uds}_power.{csv,txt}
- Canonical Luxi results (best transport): docs/benchmarks/torch_luxi_power.{csv,txt}, docs/benchmarks/torch_luxi.csv
- Plan (this file): docs/benchmarks/xai_escalation_plan.md

## Risks & mitigations
- JSON/HTTP overhead dominates toy workloads → move to gRPC/protobuf and batching
- Parse/plan cost per request → compile cache with expr_id
- Power capture variance → 20s steady-state; report both compute-time and PM windows; multiple runs if needed

## Comms cadence
- Drop A: within 72 hours (updated report link + ops/J tables)
- Drop B: early next week (GPU section added)
- Drop C: following week (scale-out section)

Owner: Eric. Reviewer: xAI (Grok team).
