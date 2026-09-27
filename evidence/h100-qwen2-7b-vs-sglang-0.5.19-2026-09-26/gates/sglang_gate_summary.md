#### sglang_det_fp16.json  (sglang 0.5.19, deterministic=True, backend=fa3, dtype=float16, server_info={'attention_backend': 'fa3', 'prefill_attention_backend': None, 'decode_attention_backend': None, 'sampling_backend': 'pytorch', 'chunked_prefill_size': 8192, 'disable_radix_cache': True, 'disable_cuda_graph': False})
- prefill S=2048 B=16: run-to-run alone identical=True, run-to-run batch identical=True, alone==in-batch (2 filler sets)=True, argmax all equal=True (tok 4683)
- prefill S=8192 B=4: run-to-run alone identical=True, run-to-run batch identical=True, alone==in-batch (2 filler sets)=True, argmax all equal=True (tok 17)
- prefill S=32767 B=2: run-to-run alone identical=True, run-to-run batch identical=True, alone==in-batch (2 filler sets)=True, argmax all equal=True (tok 1714)
- decode long1024 greedy256 alone_rep: first token divergence=None, first logprob divergence=None, steps with differing top-5 logprobs=0/256
- decode long1024 greedy256 uniform_batch16: first token divergence=None, first logprob divergence=None, steps with differing top-5 logprobs=0/256
- decode long1024 greedy256 uniform_batch16_rep: first token divergence=None, first logprob divergence=None, steps with differing top-5 logprobs=0/256
- decode long1024 greedy256 uniform_batch64: first token divergence=None, first logprob divergence=None, steps with differing top-5 logprobs=0/256
- decode long1024 greedy256 uniform_batch64_rep: first token divergence=None, first logprob divergence=None, steps with differing top-5 logprobs=0/256
- decode long1024 greedy256 staggered16: first token divergence=None, first logprob divergence=None, steps with differing top-5 logprobs=0/256
- decode long1024 greedy256 staggered16_rep: first token divergence=None, first logprob divergence=None, steps with differing top-5 logprobs=0/256
- prefill2048_alone_vs_staggered10: top-20 logprobs identical=True
- long1024 greedy vs HF-fix: first divergence at 248
- gate p0 'The capital of France is': argmax 12095 vs HF 12095 match=True, top-5 same order=True
- gate p1 'The quick brown fox jumps over the lazy': argmax 5562 vs HF 5562 match=True, top-5 same order=True
- gate p2 'def fibonacci(n):\n    if n <= 1:\n       ': argmax 308 vs HF 308 match=True, top-5 same order=True
- gate p3 'Water boils at a temperature of': argmax 220 vs HF 220 match=True, top-5 same order=True
- gate p4 'In 1969, the first person to walk on the': argmax 33121 vs HF 33121 match=True, top-5 same order=True
- gate greedy64 alone vs batch16 first divergence: [None, None, None, None, None]; vs HF-fix: [None, None, None, None, None]

#### sglang_det_bf16.json  (sglang 0.5.19, deterministic=True, backend=fa3, dtype=bfloat16, server_info={'attention_backend': 'fa3', 'prefill_attention_backend': None, 'decode_attention_backend': None, 'sampling_backend': 'pytorch', 'chunked_prefill_size': 8192, 'disable_radix_cache': True, 'disable_cuda_graph': False})
- prefill S=2048 B=16: run-to-run alone identical=True, run-to-run batch identical=True, alone==in-batch (2 filler sets)=True, argmax all equal=True (tok 198)
- prefill S=8192 B=4: run-to-run alone identical=True, run-to-run batch identical=True, alone==in-batch (2 filler sets)=True, argmax all equal=True (tok 17)
- prefill S=32767 B=2: run-to-run alone identical=True, run-to-run batch identical=True, alone==in-batch (2 filler sets)=True, argmax all equal=True (tok 1714)
- decode long1024 greedy256 alone_rep: first token divergence=None, first logprob divergence=None, steps with differing top-5 logprobs=0/256
- decode long1024 greedy256 uniform_batch16: first token divergence=None, first logprob divergence=None, steps with differing top-5 logprobs=0/256
- decode long1024 greedy256 uniform_batch16_rep: first token divergence=None, first logprob divergence=None, steps with differing top-5 logprobs=0/256
- decode long1024 greedy256 uniform_batch64: first token divergence=None, first logprob divergence=None, steps with differing top-5 logprobs=0/256
- decode long1024 greedy256 uniform_batch64_rep: first token divergence=None, first logprob divergence=None, steps with differing top-5 logprobs=0/256
- decode long1024 greedy256 staggered16: first token divergence=None, first logprob divergence=None, steps with differing top-5 logprobs=0/256
- decode long1024 greedy256 staggered16_rep: first token divergence=None, first logprob divergence=None, steps with differing top-5 logprobs=0/256
- prefill2048_alone_vs_staggered10: top-20 logprobs identical=True
- long1024 greedy vs HF-fix: first divergence at 128
- gate p0 'The capital of France is': argmax 12095 vs HF 12095 match=True, top-5 same order=True
- gate p1 'The quick brown fox jumps over the lazy': argmax 5562 vs HF 5562 match=True, top-5 same order=True
- gate p2 'def fibonacci(n):\n    if n <= 1:\n       ': argmax 308 vs HF 308 match=True, top-5 same order=True
- gate p3 'Water boils at a temperature of': argmax 220 vs HF 220 match=True, top-5 same order=True
- gate p4 'In 1969, the first person to walk on the': argmax 33121 vs HF 33121 match=True, top-5 same order=True
- gate greedy64 alone vs batch16 first divergence: [None, None, None, None, None]; vs HF-fix: [56, None, None, None, None]

#### sglang_normal_fp16.json  (sglang 0.5.19, deterministic=False, backend=fa3, dtype=float16, server_info={'attention_backend': 'fa3', 'prefill_attention_backend': None, 'decode_attention_backend': None, 'sampling_backend': 'flashinfer', 'chunked_prefill_size': 8192, 'disable_radix_cache': True, 'disable_cuda_graph': False})
- prefill S=2048 B=16: run-to-run alone identical=True, run-to-run batch identical=True, alone==in-batch (2 filler sets)=True, argmax all equal=True (tok 4683)
- prefill S=8192 B=4: run-to-run alone identical=True, run-to-run batch identical=True, alone==in-batch (2 filler sets)=True, argmax all equal=True (tok 17)
- prefill S=32767 B=2: run-to-run alone identical=True, run-to-run batch identical=True, alone==in-batch (2 filler sets)=True, argmax all equal=True (tok 1714)
- decode long1024 greedy256 alone_rep: first token divergence=None, first logprob divergence=None, steps with differing top-5 logprobs=0/256
- decode long1024 greedy256 uniform_batch16: first token divergence=None, first logprob divergence=1, steps with differing top-5 logprobs=254/256
- decode long1024 greedy256 uniform_batch16_rep: first token divergence=None, first logprob divergence=None, steps with differing top-5 logprobs=0/256
- decode long1024 greedy256 uniform_batch64: first token divergence=None, first logprob divergence=1, steps with differing top-5 logprobs=250/256
- decode long1024 greedy256 uniform_batch64_rep: first token divergence=None, first logprob divergence=None, steps with differing top-5 logprobs=0/256
- decode long1024 greedy256 staggered16: first token divergence=None, first logprob divergence=1, steps with differing top-5 logprobs=254/256
- decode long1024 greedy256 staggered16_rep: first token divergence=None, first logprob divergence=None, steps with differing top-5 logprobs=0/256
- prefill2048_alone_vs_staggered10: top-20 logprobs identical=True
- long1024 greedy vs HF-fix: first divergence at 248
- gate p0 'The capital of France is': argmax 12095 vs HF 12095 match=True, top-5 same order=True
- gate p1 'The quick brown fox jumps over the lazy': argmax 5562 vs HF 5562 match=True, top-5 same order=True
- gate p2 'def fibonacci(n):\n    if n <= 1:\n       ': argmax 308 vs HF 308 match=True, top-5 same order=True
- gate p3 'Water boils at a temperature of': argmax 220 vs HF 220 match=True, top-5 same order=True
- gate p4 'In 1969, the first person to walk on the': argmax 33121 vs HF 33121 match=True, top-5 same order=True
- gate greedy64 alone vs batch16 first divergence: [None, None, None, 16, None]; vs HF-fix: [None, None, None, None, None]

