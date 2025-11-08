#!/usr/bin/env python3
import requests
import json
import time
import numpy as np

# Try to initialize NVIDIA GPU monitoring (production environment)
GPU_AVAILABLE = False
try:
    from pynvml import *
    nvmlInit()
    handle = nvmlDeviceGetHandleByIndex(0)
    GPU_AVAILABLE = True
    print("✅ GPU detected - power monitoring enabled")
except Exception as e:
    print(f"⚠️  GPU not available - power monitoring disabled ({e})")
    print("   (This is expected in dev containers - deploy to GPU for full metrics)")
    handle = None

def get_power_watts():
    if GPU_AVAILABLE and handle:
        power_mw = nvmlDeviceGetPowerUsage(handle)
        return power_mw / 1000.0
    return 0.0  # Fallback when no GPU

def generate_payload():
    np.random.seed(42)
    values = np.random.uniform(-10, 10, 4000000).astype(np.float32)
    return values.tolist()

print("🚀 Luxi Edge 4M Benchmark")
print("=" * 50)

# Generate your seed=42 payload
print("📦 Generating 4M payload (seed=42)...")
values = generate_payload()
payload = {
    "expr": "sin(x)*cos(x)",
    "values": values,
    "precision": "f16"
}
print(f"   Payload: {len(values):,} elements ({len(values)*4/1e6:.1f} MB)")

# Baseline power measurement
baseline_power = get_power_watts()
print(f"   Baseline power: {baseline_power:.1f}W")

# Run 20s benchmark at 10Hz (200 requests)
print("\n🔄 Starting 20s benchmark (10Hz, 200 requests)...")
start_time = time.time()
total_ops = 0
total_latency = 0
successful_requests = 0

for i in range(200):
    request_start = time.time()
    
    try:
        response = requests.post(
            "http://127.0.0.1:8080/evaluate",
            json=payload,
            headers={"Content-Type": "application/json"},
            timeout=300  # 5min timeout for 4M payload
        )
        
        if response.status_code == 200:
            data = response.json()
            latency_ms = data["latency_ms"]
            ops_per_sec = data["ops_per_sec"]
            
            total_ops += len(values)
            total_latency += latency_ms
            successful_requests += 1
            
            if i % 20 == 0:
                current_power = get_power_watts()
                print(f"   Iteration {i+1}: {ops_per_sec:,.0f} ops/sec, {latency_ms:.0f}ms, {current_power:.1f}W")
        
        # Throttle to 10Hz (100ms between requests)
        elapsed = (time.time() - request_start) * 1000
        if elapsed < 100:
            time.sleep((100 - elapsed) / 1000)
            
    except Exception as e:
        print(f"   Request {i+1} failed: {e}")

duration = time.time() - start_time
avg_latency = total_latency / successful_requests if successful_requests > 0 else 0
avg_ops_per_sec = total_ops / duration if duration > 0 else 0
req_per_sec = successful_requests / duration if duration > 0 else 0
final_power = get_power_watts()

# Calculate efficiency
avg_power = (baseline_power + final_power) / 2
ops_per_joule = avg_ops_per_sec / avg_power if avg_power > 0 else 0
simd_gap = 30000000 / avg_ops_per_sec if avg_ops_per_sec > 0 else float('inf')
power_gap = 600000000 / ops_per_joule if ops_per_joule > 0 else float('inf')

print("\n" + "="*60)
print("🎯 LUXI EDGE 4M PRODUCTION BENCHMARK RESULTS")
print("="*60)
print(f"Total operations: {total_ops:,}")
print(f"Benchmark duration: {duration:.1f}s")
print(f"Successful requests: {successful_requests}")
print(f"Average latency: {avg_latency:.0f}ms")
print(f"Average ops/sec (Rhai): {avg_ops_per_sec:,.0f}")
print(f"Requests/sec: {req_per_sec:.1f}")
print(f"Average power: {avg_power:.1f}W")
print(f"ops/J (Rhai): {ops_per_joule:,.0f}")
print(f"SIMD gap (vs 30M ops/sec): {simd_gap:.0f}x")
print(f"Power gap (vs 600M ops/J): {power_gap:.0f}x")
print(f"GPU speedup needed: {simd_gap:.0f}x")
print(f"Efficiency improvement needed: {power_gap:.0f}x")
print("="*60)

# Cost analysis
pytorch_cost_per_million = 0.001  # $0.001 per 1M ops (typical cloud pricing)
luxi_cost_per_million = pytorch_cost_per_million / (simd_gap / 1140)  # Scale by your gap
savings_percent = (1 - luxi_cost_per_million / pytorch_cost_per_million) * 100

print(f"\n💰 Cost Analysis (vs PyTorch):")
print(f"PyTorch: ${pytorch_cost_per_million:.4f} per 1M ops")
print(f"Luxi Edge: ${luxi_cost_per_million:.4f} per 1M ops")
print(f"Cost savings: {savings_percent:.1f}%")

if GPU_AVAILABLE:
    nvmlShutdown()
else:
    print(f"\n📝 Note: Deploy to GPU instance for full power metrics")
    print(f"   Expected on L4 GPU: ~{avg_power + 50:.0f}W under load")

