#!/usr/bin/env python3
import argparse, time, os
try:
    import pynvml
    NVML=True
except Exception:
    NVML=False
def read_cpu_energy_uj():
    base="/sys/class/powercap"
    tot=None
    for root,_,files in os.walk(base):
        if "energy_uj" in files:
            p=os.path.join(root,"energy_uj")
            try:
                v=int(open(p).read().strip())
                tot = v if tot is None else tot+v
            except Exception:
                pass
    return tot
def main():
    ap=argparse.ArgumentParser()
    ap.add_argument("duration_s", type=float)
    ap.add_argument("out_path")
    args=ap.parse_args()
    with open(args.out_path,"w") as out:
        out.write("linux power sampler (NVML+RAPL)\n")
        if NVML:
            try: pynvml.nvmlInit()
            except Exception: pass
        prev=read_cpu_energy_uj(); tprev=time.time()
        end=time.time()+args.duration_s+1.5
        while time.time()<end:
            # CPU power (W)
            cpu_w=0.0
            now=read_cpu_energy_uj(); tnow=time.time()
            if prev is not None and now is not None:
                du=max(0, now-prev)  # microjoules
                dt=max(1e-3, tnow-tprev)
                cpu_w=(du/1e6)/dt
                prev=now; tprev=tnow
            # GPU power (W)
            gpu_w=0.0
            if NVML:
                try:
                    n=pynvml.nvmlDeviceGetCount()
                    for i in range(n):
                        h=pynvml.nvmlDeviceGetHandleByIndex(i)
                        gpu_w += pynvml.nvmlDeviceGetPowerUsage(h)/1000.0
                except Exception:
                    gpu_w=0.0
            out.write(f"CPU Power: {cpu_w:.2f} W\n")
            out.write(f"GPU Power: {gpu_w:.2f} W\n")
            out.flush()
            time.sleep(1.0)
        if NVML:
            try: pynvml.nvmlShutdown()
            except Exception: pass
if __name__=="__main__":
    import time
    main()
