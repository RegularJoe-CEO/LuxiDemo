#!/usr/bin/env python3
"""
Client to trigger RunPod serverless benchmark
"""
import requests
import json
import time

RUNPOD_ENDPOINT = "https://api.runpod.ai/v2/iaa1ysld1fk8zw/run"
RUNPOD_API_KEY = "YOUR_API_KEY_HERE"  # Replace with your RunPod API key

def trigger_benchmark(benchmark_type="quick", expr="sin(x)*cos(x)", num_requests=10):
    """
    Trigger benchmark on RunPod
    
    Args:
        benchmark_type: "quick" (1k elements) or "4m" (4M elements)
        expr: Expression to evaluate
        num_requests: Number of requests to run
    """
    
    payload = {
        "input": {
            "benchmark_type": benchmark_type,
            "expr": expr,
            "num_requests": num_requests,
            "payload_size": 4000000 if benchmark_type == "4m" else 1000
        }
    }
    
    headers = {
        "Content-Type": "application/json",
        "Authorization": f"Bearer {RUNPOD_API_KEY}"
    }
    
    print(f"🚀 Triggering {benchmark_type} benchmark on RunPod...")
    print(f"   Expression: {expr}")
    print(f"   Requests: {num_requests}")
    
    # Trigger the job
    response = requests.post(RUNPOD_ENDPOINT, json=payload, headers=headers)
    
    if response.status_code != 200:
        print(f"❌ Failed to trigger job: {response.status_code}")
        print(response.text)
        return None
    
    job_data = response.json()
    job_id = job_data.get("id")
    
    print(f"✅ Job triggered: {job_id}")
    print(f"   Waiting for results...")
    
    # Poll for results
    status_url = f"https://api.runpod.ai/v2/iaa1ysld1fk8zw/status/{job_id}"
    
    while True:
        time.sleep(2)
        status_response = requests.get(status_url, headers=headers)
        
        if status_response.status_code != 200:
            print(f"❌ Failed to get status: {status_response.status_code}")
            break
        
        status_data = status_response.json()
        status = status_data.get("status")
        
        print(f"   Status: {status}")
        
        if status == "COMPLETED":
            results = status_data.get("output", {})
            print("\n" + "="*60)
            print("🎯 RUNPOD GPU BENCHMARK RESULTS")
            print("="*60)
            print(json.dumps(results, indent=2))
            print("="*60)
            return results
        
        elif status in ["FAILED", "CANCELLED"]:
            print(f"❌ Job {status}")
            print(json.dumps(status_data, indent=2))
            break

if __name__ == "__main__":
    import sys
    
    if RUNPOD_API_KEY == "YOUR_API_KEY_HERE":
        print("⚠️  Please set your RunPod API key in this file")
        print("   Find it at: https://www.runpod.io/console/user/settings")
        sys.exit(1)
    
    # Run quick test first
    print("\n📊 Running quick test (1k elements)...")
    trigger_benchmark(benchmark_type="quick", expr="sin(x)*cos(x)", num_requests=5)
    
    # Optionally run full 4M benchmark
    run_full = input("\n🔥 Run full 4M benchmark? (y/N): ").lower()
    if run_full == 'y':
        print("\n📊 Running full 4M benchmark...")
        trigger_benchmark(benchmark_type="4m", expr="sin(x)*cos(x)", num_requests=200)
