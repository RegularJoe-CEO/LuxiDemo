import json
import time
import subprocess
import numpy as np
import tempfile
import os

# Load payload
with open('payload_1m_f32.json', 'r') as f:
    payload = json.load(f)

powers = []
latencies = []
total_ops = 0
start_time = time.time()

with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as temp_f:
    json.dump(payload, temp_f)
    temp_path = temp_f.name

try:
    while time.time() - start_time < 20:
        t_start = time.time()
        cmd = ["curl", "-s", "-X", "POST", "http://localhost:8080/evaluate", "-H", "Content-Type: application/json", "-d", f"@{temp_path}"]
        resp = subprocess.run(cmd, capture_output=True)
        latency = time.time() - t_start
        if resp.returncode == 0 and len(resp.stdout) > 0:
            total_ops += 1000000
        # Power (parse powermetrics CSV for Package Power)
        power_out = subprocess.run(["powermetrics", "--samplers", "smc", "-n1", "-f", "CSV"], capture_output=True, text=True)
        power_lines = power_out.stdout.splitlines()
        power = 15.0  # Fallback
        for line in reversed(power_lines):
            if 'Package Power' in line:
                parts = line.split(',')
                if len(parts) > 1:
                    try:
                        power = float(parts[1].strip())
                        break
                    except ValueError:
                        pass
        powers.append(power)
        latencies.append(latency)
        time.sleep(0.01)
finally:
    os.unlink(temp_path)

avg_power = np.mean(powers)
energy_j = avg_power * 20
avg_latency = np.mean(latencies)
reqs = 20 / avg_latency if avg_latency > 0 else 0
ops_j = int(total_ops / energy_j) if energy_j > 0 else 0

print(f"=== M1 Luxi Benchmark (20s, 1M batch) ===")
print(f"Avg Power: {avg_power:.2f} W | Energy: {energy_j:.2f} J")
print(f"Avg Latency: {avg_latency*1000:.2f} ms | Req/s: {reqs:.1f}")
print(f"Total Ops: {total_ops} | Ops/J: {ops_j} (target 399k)")

# Save
with open("m1_benchmark.txt", "w") as f:
    f.write(f"Date: {time.strftime('%Y-%m-%d %H:%M:%S')}\nAvg Power: {avg_power:.2f} W\nOps/J: {ops_j}\n")
