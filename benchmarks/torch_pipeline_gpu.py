import time, argparse, json
import numpy as np
import torch

class CSVLogger:
    def __init__(self, csv_path, meta=None):
        self.csv_path = csv_path
        self.meta_path = csv_path.replace(".csv",".meta.json")
        self.f = open(csv_path,"w")
        self.f.write("elapsed_s,samples\n")
        with open(self.meta_path,"w") as mf:
            json.dump(meta or {}, mf)
    def log(self, samples, elapsed): self.f.write(f"{elapsed:.9f},{int(samples)}\\n")
    def close(self): self.f.flush(); self.f.close()

def phi_gpu(x, a):
    x_abs = torch.abs(x)
    neg = x < 0.0
    pos = ~neg
    y = torch.empty_like(x)
    y[neg] = torch.sin(x[neg]) + a * x[neg] * x[neg]
    xc = torch.clamp(x[pos], min=-0.999999)
    y[pos] = torch.log1p(xc) - torch.sqrt(x_abs[pos]) + 0.1 * x[pos] * x[pos] * x[pos]
    return y

def main():
    ap=argparse.ArgumentParser()
    ap.add_argument("--a", type=float, default=0.2)
    ap.add_argument("--batch-size", type=int, default=1048576)
    ap.add_argument("--duration-s", type=float, default=20.0)
    ap.add_argument("--csv", default="docs/benchmarks/torch_gpu_baseline_power.csv")
    args=ap.parse_args()
    if not torch.cuda.is_available(): raise SystemExit("CUDA not available")
    device = torch.device("cuda:0"); torch.backends.cudnn.benchmark=True
    meta = {"framework":"pytorch","device":"cuda","mode":"baseline","a":args.a,
            "batch_size":args.batch_size,"batches":0,"threads":1,"concurrency":1,
            "transport":"inproc","duration_s":args.duration_s}
    logger=CSVLogger(args.csv, meta)
    rng = np.random.default_rng(1337)
    with torch.inference_mode():
        xb = torch.from_numpy(rng.normal(0.0,5.0,size=(args.batch_size,)).astype(np.float32)).to(device)
        for _ in range(5): _ = phi_gpu(xb, args.a)
        torch.cuda.synchronize()
        total=0; t0=time.perf_counter()
        while True:
            xb = torch.from_numpy(rng.normal(0.0,5.0,size=(args.batch_size,)).astype(np.float32)).to(device, non_blocking=True)
            t1=time.perf_counter(); _=phi_gpu(xb, args.a); torch.cuda.synchronize(); t2=time.perf_counter()
            logger.log(xb.numel(), t2-t1); total += xb.numel()
            if (t2 - t0) >= args.duration_s: break
    logger.close(); dur=time.perf_counter()-t0
    print(f"Steady-state: {total} samples in {dur:.3f}s => {total/max(1e-9,dur):.1f} samples/s")
if __name__=="__main__": main()
