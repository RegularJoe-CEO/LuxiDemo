# Prefill aggressive accel lock
**Pack:** `prefill_accel_lock_20260807T233111Z`
**GPU:** H100 80GB · flash+device_resident+FP16

## Phase A top thr (1×10s)
- 45592.6 pos/s · B=72 · A_dual_fuse_B72
- 45455.4 pos/s · B=64 · A_dual_mlp_B64
- 45379.7 pos/s · B=68 · A_pair_B68
- 45355.6 pos/s · B=68 · A_dual_pair_mlp_B68
- 45334.5 pos/s · B=72 · A_base_B72
- 45323.5 pos/s · B=72 · A_dual_mlp_B72
- 45285.1 pos/s · B=68 · A_dual_pair_B68
- 45253.8 pos/s · B=64 · A_dual_pair_fuse_B64

## Phase B multi-run (5×15s) + energy
- **B_multirun_dual_fuse_B72**: thr_med=44907.4 (n=5 stdev=3686.4) J/pos=0.015483 W=688.768 flash=True
- **B_multirun_dual_gemm_B72**: thr_med=44859.9 (n=5 stdev=308.2) J/pos=0.015322 W=687.54 flash=True
- **B_multirun_dual_mlp_B64**: thr_med=44805.1 (n=5 stdev=566.8) J/pos=0.015412 W=687.9575 flash=True
- **B_multirun_pair_B68**: thr_med=44404.4 (n=5 stdev=113.0) J/pos=0.015399 W=685.6285 flash=True
- **B_multirun_base_B32**: thr_med=42764.4 (n=5 stdev=306.1) J/pos=0.016102 W=687.99 flash=True
- **B_multirun_base_B16**: thr_med=39985.0 (n=5 stdev=475.9) J/pos=0.016844 W=675.651 flash=True

## Det dual-run
{
  "dual_gemm_B72": {
    "run1": 45206.5,
    "run2": 37401.4,
    "rel_diff": 0.17265437492396002,
    "both_flash": true
  },
  "pair_B64": {
    "run1": 45351.7,
    "run2": 43754.3,
    "rel_diff": 0.03522249441586521,
    "both_flash": true
  }
}

## vs prior freeze B16 thr 41221 / J 0.0169
- Best multirun thr **44907** vs 41221 → **1.09×**
- Best J/pos **0.015483** vs 0.0169 → ratio **0.917**
