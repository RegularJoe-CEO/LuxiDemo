# Third-party components

Shipped inside this package:

| Component | Where | License |
|---|---|---|
| NVIDIA CUDA runtime (cudart 12.8, statically linked) | inside `engine/liblx_cuda.so` | NVIDIA CUDA Toolkit EULA (cudart is listed as redistributable) |
| GNU OpenMP runtime `libgomp.so.1` (GCC 11.4, Ubuntu 22.04) | `engine/libgomp.so.1`, used by `engine/liblx_cpu.so` | GPLv3 with the GCC Runtime Library Exception 3.1 |
| Cython runtime support code (Cython 3.3.1) | compiled into `engine/lx_*.so` | Apache License 2.0 |

Installed by `install.sh` from package indexes (not shipped here), each under its own license: PyTorch 2.13.0 (BSD-3-Clause;
the GPU build pulls NVIDIA CUDA libraries under NVIDIA's license), NumPy (BSD-3-Clause), safetensors (Apache-2.0),
PyNaCl with libsodium (Apache-2.0 / ISC), huggingface_hub and hf-xet (Apache-2.0), tokenizers (Apache-2.0),
nvidia-ml-py (BSD-3-Clause, GPU install only), and uv (Apache-2.0 / MIT) when no `uv` is already installed.

Downloaded by `./luxi fetch-model` (not shipped here): RedHatAI/DeepSeek-R1-Distill-Qwen-7B-quantized.w8a8 at revision
925259de674c80799e9455bfcd2bf50f999022cb, MIT license, an INT8 quantization by Red Hat AI of
deepseek-ai/DeepSeek-R1-Distill-Qwen-7B (MIT).

Fixture prompts: problems from MATH-500 (MIT, from the MATH dataset by Hendrycks et al.) and AIME 2024 questions, as used
in the November 2026 Luxi R1 evaluation.
