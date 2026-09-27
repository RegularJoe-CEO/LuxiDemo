# Luxi c8 vs SGLang deterministic — H100 race runbook (prepared 2026-09-26, nothing run on a GPU yet)

[internal pre-run status note removed]

## 0. Pod (RunPod WEB CONSOLE only, fresh pod, do not touch [other pods redacted])
- 1x H100 SXM 80GB, Eric's network volume (California DC) attached if an H100 is available there; otherwise H200 (approved), otherwise H100 SXM elsewhere (model comes from HF, 58 s last time).
- Template: RunPod PyTorch 2.4 / CUDA 12.4.1 devel (Ubuntu 22.04). This gives nvcc 12.4, the version Luxi was built with on 2026-09-25/26.
- If the filter is offered, choose **CUDA 13.0**, which means driver >= 580. SGLang 0.5.20 (current) ships CUDA-13-only wheels.
  On a 570 driver (the last pod had 570.195.03), pod_setup.sh falls back to the CUDA 12.9 lane, which is sglang 0.5.19 (the last CUDA 12 release, pip overrides; untested).
- Container disk >= 80 GB (vLLM, SGLang and torch venvs plus the Rust build). Copy the "SSH over exposed TCP" command from Connect.

## 1. Box -> pod
    cd /workspace/luxi_sglang_race_2026-09-26
    ./box_pod.sh push  "<ssh cmd>"      # Luxi source (base c112ef6 + patches 0001-0007 = c8; local commit e50c91f, tree 35851a3), 2026-09-25 materials, kit
    ./box_pod.sh setup "<ssh cmd>"      # background: model, vLLM 0.25.1+cu129, SGLang, Rust 1.98.1, Luxi build with cuBLAS 12.9 RUNPATH, FA3 shim
    ./box_pod.sh run   "<ssh cmd>" "tail -5 /workspace/race/setup/pod_setup.log; tail -3 /workspace/race/setup/*.log"
  (POD_KEY=[ssh key path redacted] if the default key is not the one registered in RunPod.)

## 2. Gates (on pod, about 15-25 min)
    ./box_pod.sh run "<ssh cmd>" "nohup bash /workspace/race/kit/gates.sh luxi D E > /workspace/race/gates.out 2>&1 &"
- Luxi: the full 19-case long gate (as long/gate_c8_final) plus the 5-prompt HF logits gate. Pass means: " Paris" (12095), argmax and top-5 == HF fp16,
  one logits hash across B=1/16/64 and repeats. Bonus check: hashes equal to the 2026-09-26 final gate (same bits, same engine).
- SGLang (D = det fp16, E = det bf16): det_vllm.py + det_vllm2.py probes with the same prompts, same fillers and same shapes, plus HF correctness.
  If fa3 fails: SGL_BACKEND=flashinfer (then triton). Record the error text.

## 3. Race (matched alternating blocks, same method as long/FINAL: 3 runs x 12 s per cell per block, 2 blocks per engine -> n=6)
    ./box_pod.sh run "<ssh cmd>" "nohup bash /workspace/race/kit/race.sh /workspace/race/R1 3 12 'D L E D L E' > /workspace/race/r1.out 2>&1 &"
    optional if budget allows: /workspace/race/R2 3 12 'N V N V'
- Each block takes about 7-8 min, so R1 is about 45 min and R2 about 30 min. Create /workspace/race/R1/STOP to stop cleanly after the current block.

## 4. Summarize, pull, stop
    python3 kit/summarize_race.py luxi_c8 luxi_c8=R1/L2.json,R1/L5.json sglang_det_fp16=R1/D1.json,R1/D4.json sglang_det_bf16=R1/E3.json,R1/E6.json [sglang_normal_fp16=R2/N1.json,R2/N3.json vllm_default=R2/V2.json,R2/V4.json] > summary_tables.md
    ./box_pod.sh pull "<ssh cmd>"   -> pod_results/
  Then STOP the pod in the web console (stop, not terminate) and confirm it shows Stopped.

## Budget estimate (H100 SXM about $2.7-3/h): setup 30-45 min + gates 20 min + R1 45 min (+ R2 30 min) = about 2-2.5 h = about $6-8.

## Notes for fairness / honesty
- The 2026-09-25/26 race ran vLLM and Luxi in **float16** (Luxi is FP16-only). The task text says bf16.
  SGLang deterministic mode uses DeepGEMM for bf16 matmuls but a Triton persistent kernel for fp16 (sglang/srt/batch_invariant_ops), so dtype changes SGLang-det speed.
  The kit therefore runs SGLang-det at both fp16 (same format as Luxi) and bf16, and labels them separately.
- SGLang vendors batch_invariant_ops ("Adapted from thinking-machines-lab/batch_invariant_ops"). Upstream TML main at prep time: f22b1fb (2025-11-04).
  On H100, deterministic mode defaults to fa3 with num_splits=1 (same as Luxi's FA3 decode setting) and forces the pytorch sampling backend.
- Radix/prefix cache is disabled for SGLang, matching vLLM enable_prefix_caching=False. SGLang prompt pools use the same RNG seed as the vLLM arm, so the token ids are identical.
- The GitHub release version-100-serve Linux binary (634 KB) is the toy OpenAI-shaped demo server. It is NOT the benchmarked engine (cuda_qwen7b_trade, qwen2_fast path), so Luxi is rebuilt from source.
