# luxi-sglang race: DeepGEMM (used by SGLang deterministic bf16 matmuls) JIT-compiles with nvcc from CUDA_HOME;
# the image's CUDA 12.4 nvcc fails ('NVCC compilation failed'), so bf16 runs use the CUDA 12.9 toolkit. fp16 runs unchanged.
import os
if os.environ.get('LXR_DTYPE') == 'bfloat16' and 'CUDA_HOME' not in os.environ:
    os.environ['CUDA_HOME'] = '/usr/local/cuda-12.9'
