# H100 NVL evidence pack (public)

**Public repo:** https://github.com/RegularJoe-CEO/LuxiDemo  
**Folder:** `evidence/h100-serve-sustain-2026-07-11/`

## Open these first

1. **Exact commands + power definition:** [README.md](./README.md)
2. **Raw power trace CSV (180 samples / 30 min):** [power_trace_sustain_30m.csv](./power_trace_sustain_30m.csv)
3. **Full JSON run log:** [serve_sustain_30m.json](./serve_sustain_30m.json)
4. **Idle baseline (~63.65 W):** [idle_power_30s.txt](./idle_power_30s.txt)
5. **GPU inventory / power limit:** [nvidia_smi_power.txt](./nvidia_smi_power.txt)

## One-line honesty

Live power = **single-GPU board `power.draw` via nvidia-smi** (absolute watts, not whole-pod wall plug, not Δ-idle).  
Median under continuous-batch load ≈ **63.74 W** ≈ idle board (~63.65 W) because this serve path is **CPU continuous-batch**, not full CUDA FLOPs.  
City-block **0.0125 / 0.0075 J/token** are **constants** from Phase-1 H100 measurements applied in software — not ∫P dt from this serve run.

## Contact

Eric Waller · e@ewaller.com · RegularJoe-CEO
