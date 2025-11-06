#!/usr/bin/env bash
set -euo pipefail
IMG="pytorch/pytorch:2.4.1-cuda12.1-cudnn9-runtime"
docker pull "$IMG"
docker run --rm -it --gpus all -v "$PWD":/work -w /work "$IMG" bash -lc '
pip install --no-cache-dir pynvml numpy
python3 - <<PY
import torch
print("CUDA available:", torch.cuda.is_available())
if torch.cuda.is_available():
    print("GPU:", torch.cuda.get_device_name(0))
PY
bash scripts/run_gpu_powered_bench.sh 20 1048576 0.2
'
