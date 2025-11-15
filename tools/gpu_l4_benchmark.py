#!/usr/bin/env python3
"""
NVIDIA L4 throughput & energy benchmark for eRock / Luxi.

- Uses CuPy for GPU compute.
- Uses NVML (pynvml) for power sampling.
- Reports ops/sec and ops/J (element-wise operations).

Run on a CUDA box (e.g., RunPod L4) after installing:
    pip install "cupy-cuda11x" pynvml
"""

import argparse
import statistics as stats
import time

import cupy as cp
import pynvml


def parse_args() -> argparse.Namespace:
    p = argparse.ArgumentParser(
        description="Measure NVIDIA L4 throughput and energy (ops/sec, ops/J)."
    )
    p.add_argument(
        "--elements",
        type=int,
        default=50_000_000,
        help="Number of elements in the vector for each kernel launch.",
    )
    p.add_argument(
        "--duration",
        type=float,
        default=20.0,
        help="Target measurement duration in seconds (excluding warmup).",
    )
    p.add_argument(
        "--warmup-iters",
        type=int,
        default=5,
        help="Number of warmup iterations before measurement.",
    )
    p.add_argument(
        "--device",
        type=int,
        default=0,
        help="CUDA device index for CuPy/NVML (default: 0).",
    )
    p.add_argument(
        "--dtype",
        choices=["fp16", "fp32"],
        default="fp16",
        help="Data type for the benchmark vector.",
    )
    p.add_argument(
        "--op",
        choices=["fma", "sin"],
        default="fma",
        help="Kernel to run: fused multiply-add (fma) or sin(x).",
    )
    return p.parse_args()


def fma_inplace(x: cp.ndarray) -> None:
    """In-place fused multiply-add: x = x * 1.5 + 2.0."""
    if x.dtype == cp.float16:
        a = cp.float16(1.5)
        b = cp.float16(2.0)
    else:
        a = cp.float32(1.5)
        b = cp.float32(2.0)
    x *= a
    x += b


def sin_inplace(x: cp.ndarray) -> None:
    """In-place sin kernel (overwrite x with sin(x))."""
    x[...] = cp.sin(x)


def main() -> None:
    args = parse_args()

    # Select device
    cp.cuda.Device(args.device).use()

    # Choose dtype
    dtype = cp.float16 if args.dtype == "fp16" else cp.float32

    # Allocate data
    n = args.elements
    x = cp.linspace(0, 1, n, dtype=dtype)

    print("=== L4 GPU Benchmark (eRock) ===")
    print(f"Device index : {args.device}")
    print(f"Elements     : {n:,}")
    print(f"DType        : {args.dtype}")
    print(f"Kernel       : {args.op}")
    print(f"Warmup iters : {args.warmup_iters}")
    print(f"Target time  : {args.duration:.1f} s")

    # Warmup (get GPU out of idle, JIT compile kernels, etc.)
    for _ in range(args.warmup_iters):
        if args.op == "fma":
            fma_inplace(x)
        else:
            sin_inplace(x)
        cp.cuda.Stream.null.synchronize()

    # Initialize NVML
    pynvml.nvmlInit()
    handle = pynvml.nvmlDeviceGetHandleByIndex(args.device)
    name = pynvml.nvmlDeviceGetName(handle).decode("utf-8")
    print(f"NVML device  : {name}")

    # Measurement loop
    start = time.time()
    power_samples = []
    total_ops = 0  # element-wise operations

    while True:
        now = time.time()
        if now - start >= args.duration:
            break

        # Sample power (mW → W)
        p_mw = pynvml.nvmlDeviceGetPowerUsage(handle)
        power_samples.append(p_mw / 1000.0)

        # Run one kernel
        if args.op == "fma":
            fma_inplace(x)
        else:
            sin_inplace(x)
        cp.cuda.Stream.null.synchronize()

        total_ops += n

    end = time.time()
    duration = end - start
    avg_power = stats.mean(power_samples) if power_samples else 0.0
    energy_j = avg_power * duration if avg_power > 0 else float("nan")
    ops_per_sec = total_ops / duration if duration > 0 else float("nan")
    ops_per_j = total_ops / energy_j if energy_j > 0 else float("nan")

    print("\n=== Results ===")
    print(f"Measured time (s) : {duration:.4f}")
    print(f"Power samples     : {len(power_samples)}")
    print(f"Avg power   (W)   : {avg_power:.2f}")
    print(f"Total ops         : {total_ops:.3e} (element evaluations)")
    print(f"Throughput  (ops/s): {ops_per_sec:.3e}")
    print(f"Energy eff. (ops/J): {ops_per_j:.3e}")


if __name__ == "__main__":
    main()

