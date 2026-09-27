## Medians per test (n = metered runs)

### prefill S=32767 B=1
| engine | prompt tok/s (median) | J/token (median) | s/call (median) | board W (median) | SM MHz (median) | n |
|---|---|---|---|---|---|---|
| luxi_c8 | 3.178e+04 | 0.02201 | 1.031 | 698 | 1380 | 6 |
| sglang_det_fp16 | 2.622e+04 | 0.02652 | 1.25 | 695 | 1470 | 6 |
| sglang_det_bf16 | 2.693e+04 | 0.02573 | 1.217 | 693 | 1541 | 6 |
| sglang_normal_fp16 | 3.037e+04 | 0.02294 | 1.079 | 697 | 1418 | 6 |
| vllm_0.25.1_default | 2.948e+04 | 0.02312 | 1.111 | 680 | 1440 | 6 |

| ratio vs luxi_c8 | throughput (luxi_c8 / other) | energy per token (luxi_c8 / other) |
|---|---|---|
| sglang_det_fp16 | 1.212x | 0.830x |
| sglang_det_bf16 | 1.180x | 0.855x |
| sglang_normal_fp16 | 1.046x | 0.959x |
| vllm_0.25.1_default | 1.078x | 0.952x |

### prefill S=8192 B=4
| engine | prompt tok/s (median) | J/token (median) | s/call (median) | board W (median) | SM MHz (median) | n |
|---|---|---|---|---|---|---|
| luxi_c8 | 4.17e+04 | 0.01671 | 0.7858 | 698 | 1365 | 6 |
| sglang_det_fp16 | 3.39e+04 | 0.02053 | 0.9665 | 695 | 1478 | 6 |
| sglang_det_bf16 | 3.495e+04 | 0.0199 | 0.9376 | 695 | 1538 | 6 |
| sglang_normal_fp16 | 4.104e+04 | 0.01701 | 0.7984 | 697 | 1395 | 6 |
| vllm_0.25.1_default | 4.055e+04 | 0.01708 | 0.8082 | 696 | 1395 | 6 |

| ratio vs luxi_c8 | throughput (luxi_c8 / other) | energy per token (luxi_c8 / other) |
|---|---|---|
| sglang_det_fp16 | 1.230x | 0.814x |
| sglang_det_bf16 | 1.193x | 0.839x |
| sglang_normal_fp16 | 1.016x | 0.982x |
| vllm_0.25.1_default | 1.028x | 0.978x |

### prefill S=2048 B=16
| engine | prompt tok/s (median) | J/token (median) | s/call (median) | board W (median) | SM MHz (median) | n |
|---|---|---|---|---|---|---|
| luxi_c8 | 4.532e+04 | 0.0154 | 0.723 | 698 | 1365 | 6 |
| sglang_det_fp16 | 3.658e+04 | 0.01902 | 0.8957 | 696 | 1455 | 6 |
| sglang_det_bf16 | 3.769e+04 | 0.01846 | 0.8693 | 695 | 1530 | 6 |
| sglang_normal_fp16 | 4.476e+04 | 0.01555 | 0.7321 | 697 | 1380 | 6 |
| vllm_0.25.1_default | 4.405e+04 | 0.01574 | 0.7438 | 694 | 1380 | 6 |

| ratio vs luxi_c8 | throughput (luxi_c8 / other) | energy per token (luxi_c8 / other) |
|---|---|---|
| sglang_det_fp16 | 1.239x | 0.810x |
| sglang_det_bf16 | 1.202x | 0.834x |
| sglang_normal_fp16 | 1.013x | 0.990x |
| vllm_0.25.1_default | 1.029x | 0.979x |

### decode-cell prefill-only S=1024 B=1 (gen=1)
| engine | prompt tok/s (median) | J/token (median) | s/call (median) | board W (median) | SM MHz (median) | n |
|---|---|---|---|---|---|---|
| luxi_c8 | 3.774e+04 | 0.01839 | 0.02713 | 694 | 1650 | 6 |
| sglang_det_fp16 | 2.243e+04 | 0.02694 | 0.04565 | 604 | 1905 | 6 |
| sglang_det_bf16 | 2.963e+04 | 0.02304 | 0.03456 | 683 | 1838 | 6 |
| sglang_normal_fp16 | 3.479e+04 | 0.01977 | 0.02944 | 690 | 1699 | 6 |
| vllm_0.25.1_default | 3.755e+04 | 0.01825 | 0.02727 | 686 | 1740 | 6 |

| ratio vs luxi_c8 | throughput (luxi_c8 / other) | energy per token (luxi_c8 / other) |
|---|---|---|
| sglang_det_fp16 | 1.683x | 0.682x |
| sglang_det_bf16 | 1.274x | 0.798x |
| sglang_normal_fp16 | 1.085x | 0.930x |
| vllm_0.25.1_default | 1.005x | 1.007x |

### decode-cell prefill-only S=1024 B=16 (gen=1)
| engine | prompt tok/s (median) | J/token (median) | s/call (median) | board W (median) | SM MHz (median) | n |
|---|---|---|---|---|---|---|
| luxi_c8 | 4.526e+04 | 0.01541 | 0.362 | 698 | 1365 | 6 |
| sglang_det_fp16 | 3.637e+04 | 0.01916 | 0.4505 | 695 | 1485 | 6 |
| sglang_det_bf16 | 3.784e+04 | 0.01836 | 0.4329 | 695 | 1538 | 6 |
| sglang_normal_fp16 | 4.511e+04 | 0.01549 | 0.3632 | 697 | 1388 | 6 |
| vllm_0.25.1_default | 4.186e+04 | 0.01626 | 0.3914 | 678 | 1440 | 6 |

| ratio vs luxi_c8 | throughput (luxi_c8 / other) | energy per token (luxi_c8 / other) |
|---|---|---|
| sglang_det_fp16 | 1.245x | 0.804x |
| sglang_det_bf16 | 1.196x | 0.839x |
| sglang_normal_fp16 | 1.003x | 0.995x |
| vllm_0.25.1_default | 1.081x | 0.948x |

### decode-cell prefill-only S=1024 B=64 (gen=1)
| engine | prompt tok/s (median) | J/token (median) | s/call (median) | board W (median) | SM MHz (median) | n |
|---|---|---|---|---|---|---|
| luxi_c8 | 4.593e+04 | 0.0152 | 1.427 | 698 | 1365 | 6 |
| sglang_det_fp16 | 3.745e+04 | 0.01859 | 1.75 | 697 | 1440 | 6 |
| sglang_det_bf16 | 3.825e+04 | 0.01818 | 1.713 | 696 | 1522 | 6 |
| sglang_normal_fp16 | 4.549e+04 | 0.0153 | 1.441 | 697 | 1380 | 6 |
| vllm_0.25.1_default | 4.48e+04 | 0.01544 | 1.463 | 693 | 1395 | 6 |

| ratio vs luxi_c8 | throughput (luxi_c8 / other) | energy per token (luxi_c8 / other) |
|---|---|---|
| sglang_det_fp16 | 1.226x | 0.818x |
| sglang_det_bf16 | 1.201x | 0.836x |
| sglang_normal_fp16 | 1.010x | 0.994x |
| vllm_0.25.1_default | 1.025x | 0.985x |

### generation S=1024 B=1 gen=256 (end-to-end)
| engine | output tok/s (median) | J/token (median) | s/call (median) | board W (median) | SM MHz (median) | n |
|---|---|---|---|---|---|---|
| luxi_c8 | 168.4 | 2.894 | 1.521 | 487 | 1980 | 6 |
| sglang_det_fp16 | 58.84 | 5.92 | 4.35 | 345 | 1980 | 6 |
| sglang_det_bf16 | 135.7 | 3.266 | 1.887 | 446 | 1980 | 6 |
| sglang_normal_fp16 | 171 | 2.7 | 1.497 | 463 | 1980 | 6 |
| vllm_0.25.1_default | 164.7 | 2.729 | 1.554 | 451 | 1980 | 6 |

| ratio vs luxi_c8 | throughput (luxi_c8 / other) | energy per token (luxi_c8 / other) |
|---|---|---|
| sglang_det_fp16 | 2.862x | 0.489x |
| sglang_det_bf16 | 1.241x | 0.886x |
| sglang_normal_fp16 | 0.985x | 1.072x |
| vllm_0.25.1_default | 1.022x | 1.060x |

### generation S=1024 B=16 gen=256 (end-to-end)
| engine | output tok/s (median) | J/token (median) | s/call (median) | board W (median) | SM MHz (median) | n |
|---|---|---|---|---|---|---|
| luxi_c8 | 2130 | 0.2739 | 1.923 | 586 | 1980 | 6 |
| sglang_det_fp16 | 863.6 | 0.4593 | 4.743 | 362 | 1980 | 6 |
| sglang_det_bf16 | 1695 | 0.3136 | 2.417 | 516 | 1980 | 6 |
| sglang_normal_fp16 | 2043 | 0.2642 | 2.005 | 541 | 1980 | 6 |
| vllm_0.25.1_default | 2030 | 0.2647 | 2.018 | 536 | 1980 | 6 |

| ratio vs luxi_c8 | throughput (luxi_c8 / other) | energy per token (luxi_c8 / other) |
|---|---|---|
| sglang_det_fp16 | 2.467x | 0.596x |
| sglang_det_bf16 | 1.257x | 0.873x |
| sglang_normal_fp16 | 1.043x | 1.037x |
| vllm_0.25.1_default | 1.049x | 1.035x |

### generation S=1024 B=64 gen=256 (end-to-end)
| engine | output tok/s (median) | J/token (median) | s/call (median) | board W (median) | SM MHz (median) | n |
|---|---|---|---|---|---|---|
| luxi_c8 | 4888 | 0.1366 | 3.352 | 673 | 1965 | 6 |
| sglang_det_fp16 | 2843 | 0.18 | 5.764 | 432 | 1980 | 6 |
| sglang_det_bf16 | 4050 | 0.151 | 4.045 | 602 | 1965 | 6 |
| sglang_normal_fp16 | 4777 | 0.1351 | 3.43 | 647 | 1965 | 6 |
| vllm_0.25.1_default | 4811 | 0.1342 | 3.405 | 647 | 1965 | 6 |

| ratio vs luxi_c8 | throughput (luxi_c8 / other) | energy per token (luxi_c8 / other) |
|---|---|---|
| sglang_det_fp16 | 1.720x | 0.759x |
| sglang_det_bf16 | 1.207x | 0.905x |
| sglang_normal_fp16 | 1.023x | 1.011x |
| vllm_0.25.1_default | 1.016x | 1.018x |

## Engine metadata
```
luxi_c8 {"engine": "luxi", "bin": "long/bin_P4", "load_s": 114.74746441841125, "rc": 0, "env_fa3": "/root/ab/libluxi_fa3.so", "ts_utc": "2026-09-26T21:48:47.132084+00:00"}
sglang_det_fp16 {"engine": "sglang", "sglang": "0.5.19", "torch": "2.13.0+cu129", "load_s": 40.99314022064209, "dtype": "float16", "deterministic": true, "attention_backend_requested": "fa3", "max_model_len": 32768, "prefix_caching": false, "ts_utc": "2026-09-26T21:34:16.985662+00:00", "server_info": {"attention_backend": "fa3", "prefill_attention_backend": null, "decode_attention_backend": null, "sampling_backend": "pytorch", "chunked_prefill_size": 8192, "disable_radix_cache": true, "disable_cuda_graph": false, "enable_deterministic_inference": true, "dtype": "float16", "mem_fraction_static": 0.85, "max_running_requests": null, "max_prefill_tokens": 16384, "context_length": 32800, "enable_torch_compile": false, "version": "0.5.19", "max_total_num_tokens": 981125}}
sglang_det_bf16 {"engine": "sglang", "sglang": "0.5.19", "torch": "2.13.0+cu129", "load_s": 116.25845217704773, "dtype": "bfloat16", "deterministic": true, "attention_backend_requested": "fa3", "max_model_len": 32768, "prefix_caching": false, "ts_utc": "2026-09-26T21:51:07.473090+00:00", "server_info": {"attention_backend": "fa3", "prefill_attention_backend": null, "decode_attention_backend": null, "sampling_backend": "pytorch", "chunked_prefill_size": 8192, "disable_radix_cache": true, "disable_cuda_graph": false, "enable_deterministic_inference": true, "dtype": "bfloat16", "mem_fraction_static": 0.85, "max_running_requests": null, "max_prefill_tokens": 16384, "context_length": 32800, "enable_torch_compile": false, "version": "0.5.19", "max_total_num_tokens": 981125}}
sglang_normal_fp16 {"engine": "sglang", "sglang": "0.5.19", "torch": "2.13.0+cu129", "load_s": 36.794769525527954, "dtype": "float16", "deterministic": false, "attention_backend_requested": "fa3", "max_model_len": 32768, "prefix_caching": false, "ts_utc": "2026-09-26T22:23:23.252935+00:00", "server_info": {"attention_backend": "fa3", "prefill_attention_backend": null, "decode_attention_backend": null, "sampling_backend": "flashinfer", "chunked_prefill_size": 8192, "disable_radix_cache": true, "disable_cuda_graph": false, "enable_deterministic_inference": false, "dtype": "float16", "mem_fraction_static": 0.85, "max_running_requests": null, "max_prefill_tokens": 16384, "context_length": 32800, "enable_torch_compile": false, "version": "0.5.19", "max_total_num_tokens": 981125}}
vllm_0.25.1_default {"engine": "vllm", "vllm": "0.25.1", "torch": "2.11.0+cu129", "load_s": 112.91345071792603, "dtype": "float16", "max_model_len": 32768, "prefix_caching": false, "ts_utc": "2026-09-26T22:31:44.378180+00:00", "max_num_batched_tokens": 16384, "chunked_prefill": true}
```
