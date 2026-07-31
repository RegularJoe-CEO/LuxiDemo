# Formal determinism — version-100 GTM multi-run

**Lane:** TRADE (Flash + device-resident + FP16)  
**GPU:** H100 80GB HBM3  
**Model:** qwen2-7b-instruct-luxi  
**Seq:** 128 prefill positions  

## Multi-run token agreement (5-run campaign)

Source: `MULTI_RUN_LOCK_SLIM.json`

| Batch | Runs | Determinism score | First token id (all runs) |
|------:|-----:|------------------:|--------------------------:|
| 16 | 5 | **1.0** | 17861 |
| 32 | 5 | **1.0** | 17861 |

**Score definition:** fraction of runs whose first completion token id matches the mode across the campaign (here, unanimous).

## Serve-layer dual-run (HTTP AUDIT)

`POST /v1/audit` re-runs greedy generate twice on the serve engine and checks token-stream receipt equality.

```bash
curl -s -X POST http://127.0.0.1:8787/v1/audit -d '{}'
# {"audit":"deterministic_generate","ok":true,...}
```

Toy path: boot-time AUDIT gate in `serve_v05`.  
Full HF path: use completion probe (boot AUDIT skipped for cost).

## Dual-lane honesty

| Lane | Determinism meaning |
|------|---------------------|
| **AUDIT** | Bit-exact / receipt gold (CPU / disclosed paths) |
| **TRADE** | Fixed-prompt token agreement on GPU energy path (multi-run det=1.0 above) |

TRADE det=1.0 is **not** a claim of AUDIT bit-exact vs CPU gold on Flash kernels.

## Contact

e@ewaller.com
