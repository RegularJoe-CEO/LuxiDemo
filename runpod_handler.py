#!/usr/bin/env python3
"""
RunPod Serverless Handler for Luxi Edge GPU Benchmark
Deploy this to RunPod to run benchmarks on L4 GPU
"""
import runpod
import subprocess
import json
import time
import numpy as np

# Try GPU monitoring
try:
    from pynvml import *
    nvmlInit()
    GPU_AVAILABLE = True
except:
    GPU_AVAILABLE = False

def run_luxi_benchmark(job):
    """
    Run Luxi Edge benchmark on RunPod GPU
    
    Input job format:
    {
        "input": {
            "benchmark_type": "4m",  # or "quick"
            "expr": "sin(x)*cos(x)",
            "num_requests": 200,
            "payload_size": 4000000
        }
    }
    """
    job_input = job.get("input", {})
    benchmark_type = job_input.get("benchmark_type", "quick")
    expr = job_input.get("expr", "sin(x)*cos(x)")
    num_requests = job_input.get("num_requests", 10)
    payload_size = job_input.get("payload_size", 1000)
    
    results = {
        "benchmark_type": benchmark_type,
        "gpu_available": GPU_AVAILABLE,
        "timestamp": time.time()
    }
    
    # Get GPU info if available
    if GPU_AVAILABLE:
        try:
            handle = nvmlDeviceGetHandleByIndex(0)
            gpu_name = nvmlDeviceGetName(handle)
            power_limit = nvmlDeviceGetPowerManagementLimit(handle) / 1000.0
            results["gpu_name"] = gpu_name
            results["gpu_power_limit"] = f"{power_limit}W"
        except Exception as e:
            results["gpu_error"] = str(e)
    
    # Check if Luxi server is running
    try:
        import requests
        health_check = requests.get("http://127.0.0.1:8080/health", timeout=2)
        server_running = health_check.status_code == 200
    except:
        server_running = False
    
    results["server_running"] = server_running
    
    if not server_running:
        return {
            "status": "error",
            "message": "Luxi Edge server not running. Start with: ./target/debug/l4_benchmark",
            "results": results
        }
    
    # Run benchmark
    if benchmark_type == "4m":
        benchmark_results = run_4m_benchmark(expr, num_requests, payload_size)
    else:
        benchmark_results = run_quick_benchmark(expr)
    
    results.update(benchmark_results)
    
    return {
        "status": "success",
        "results": results
    }

def run_quick_benchmark(expr):
    """Quick 1k element benchmark"""
    import requests
    
    np.random.seed(42)
    values = np.random.uniform(-10, 10, 1000).astype(np.float32).tolist()
    
    payload = {
        "expr": expr,
        "values": values,
        "precision": "f16"
    }
    
    start = time.time()
    response = requests.post(
        "http://127.0.0.1:8080/evaluate",
        json=payload,
        timeout=30
    )
    duration = time.time() - start
    
    if response.status_code == 200:
        data = response.json()
        return {
            "test_type": "quick",
            "elements": len(values),
            "latency_ms": data["latency_ms"],
            "ops_per_sec": data["ops_per_sec"],
            "total_duration_ms": duration * 1000,
            "expr": expr,
            "sample_results": data["results"][:5]
        }
    else:
        return {
            "test_type": "quick",
            "error": f"HTTP {response.status_code}"
        }

def run_4m_benchmark(expr, num_requests, payload_size):
    """Full 4M production benchmark"""
    import requests
    
    # Generate payload
    np.random.seed(42)
    values = np.random.uniform(-10, 10, payload_size).astype(np.float32).tolist()
    
    payload = {
        "expr": expr,
        "values": values,
        "precision": "f16"
    }
    
    # Get baseline power
    baseline_power = 0.0
    if GPU_AVAILABLE:
        try:
            handle = nvmlDeviceGetHandleByIndex(0)
            baseline_power = nvmlDeviceGetPowerUsage(handle) / 1000.0
        except:
            pass
    
    # Run benchmark
    total_ops = 0
    total_latency = 0
    successful_requests = 0
    request_times = []
    
    start_time = time.time()
    
    for i in range(num_requests):
        req_start = time.time()
        
        try:
            response = requests.post(
                "http://127.0.0.1:8080/evaluate",
                json=payload,
                timeout=300
            )
            
            if response.status_code == 200:
                data = response.json()
                latency_ms = data["latency_ms"]
                
                total_ops += len(values)
                total_latency += latency_ms
                successful_requests += 1
                request_times.append(time.time() - req_start)
        except Exception as e:
            pass
    
    duration = time.time() - start_time
    
    # Get final power
    final_power = 0.0
    if GPU_AVAILABLE:
        try:
            handle = nvmlDeviceGetHandleByIndex(0)
            final_power = nvmlDeviceGetPowerUsage(handle) / 1000.0
        except:
            pass
    
    # Calculate metrics
    avg_latency = total_latency / successful_requests if successful_requests > 0 else 0
    avg_ops_per_sec = total_ops / duration if duration > 0 else 0
    req_per_sec = successful_requests / duration if duration > 0 else 0
    avg_power = (baseline_power + final_power) / 2
    ops_per_joule = avg_ops_per_sec / avg_power if avg_power > 0 else 0
    simd_gap = 30000000 / avg_ops_per_sec if avg_ops_per_sec > 0 else 0
    
    return {
        "test_type": "4m_production",
        "total_operations": total_ops,
        "benchmark_duration_s": duration,
        "successful_requests": successful_requests,
        "avg_latency_ms": avg_latency,
        "avg_ops_per_sec": avg_ops_per_sec,
        "requests_per_sec": req_per_sec,
        "baseline_power_w": baseline_power,
        "final_power_w": final_power,
        "avg_power_w": avg_power,
        "ops_per_joule": ops_per_joule,
        "simd_gap": simd_gap,
        "power_gap": 600000000 / ops_per_joule if ops_per_joule > 0 else 0,
        "expr": expr,
        "payload_size": payload_size
    }

# RunPod serverless handler
runpod.serverless.start({"handler": run_luxi_benchmark})
