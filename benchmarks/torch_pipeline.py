import os, time, argparse, numpy as np, torch, math
from torch.utils.data import DataLoader, TensorDataset
from typing import List
from concurrent.futures import ThreadPoolExecutor
from common_bench import BenchLogger
from luxi_client import LuxiClient

def baseline_op(xs: torch.Tensor, a: float) -> torch.Tensor:
    x = xs.detach().cpu().numpy()
    y = np.where(
        x < 0.0,
        np.sin(x) + a * x * x,
        np.log1p(np.clip(x, -0.999999, None)) - np.sqrt(np.abs(x)) + 0.1 * x * x * x
    )
    return torch.from_numpy(y).to(xs.device)

class LuxiOp(torch.nn.Module):
    def __init__(self, expr: str, a: float, client: LuxiClient, concurrency: int):
        super().__init__(); self.expr, self.a, self.client, self.concurrency = expr, a, client, max(1, concurrency)
    def forward(self, xs: torch.Tensor) -> torch.Tensor:
        xs_np = xs.detach().cpu().numpy().astype(np.float32).ravel(); xs_np = np.around(xs_np, 6)
        n = xs_np.shape[0]
        if self.concurrency <= 1 or n < 2:
            ys, _ = self.client.evaluate_batch(self.expr, xs_np.tolist(), vars={"a": float(self.a)})
            return torch.tensor(ys, dtype=xs.dtype, device=xs.device)
        c = min(self.concurrency, n)
        chunk = int(math.ceil(n / c))
        slices = [(i*chunk, min(n, (i+1)*chunk)) for i in range(c) if i*chunk < n]
        def call(lo, hi):
            return self.client.evaluate_batch(self.expr, xs_np[lo:hi].tolist(), vars={"a": float(self.a)})[0]
        ys_parts = []
        with ThreadPoolExecutor(max_workers=c) as ex:
            futs = [ex.submit(call, lo, hi) for (lo, hi) in slices]
            for f in futs: ys_parts.append(f.result())
        ys = [y for part in ys_parts for y in part]
        return torch.tensor(ys, dtype=xs.dtype, device=xs.device)

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--mode", choices=["baseline","luxi"], default="luxi")
    ap.add_argument("--expr", default="if x < 0.0 { sin(x) + a*x*x } else { log(1.0 + x) - sqrt(abs(x)) + 0.1*x*x*x }")
    ap.add_argument("--a", type=float, default=0.2)
    ap.add_argument("--batch-size", type=int, default=8192)
    ap.add_argument("--batches", type=int, default=200)
    ap.add_argument("--duration-s", type=float, default=0.0)
    ap.add_argument("--num-workers", type=int, default=0)
    ap.add_argument("--threads", type=int, default=1)
    ap.add_argument("--concurrency", type=int, default=1)
    ap.add_argument("--seed", type=int, default=1337)
    ap.add_argument("--csv", default="docs/benchmarks/torch_luxi.csv")
    args = ap.parse_args()

    torch.set_num_threads(args.threads)
    os.environ["OMP_NUM_THREADS"] = str(args.threads)
    os.environ["MKL_NUM_THREADS"] = str(args.threads)

    rng = np.random.default_rng(args.seed)
    total_items = max(args.batch_size * args.batches, args.batch_size * 100)
    data = rng.normal(0.0, 5.0, size=(total_items, 1)).astype(np.float64)
    ds = TensorDataset(torch.from_numpy(data))
    dl = DataLoader(ds, batch_size=args.batch_size, shuffle=False, num_workers=args.num_workers, pin_memory=False)

    meta_base = {
        "framework":"pytorch","mode":args.mode,"expr":args.expr,"a":args.a,
        "batch_size":args.batch_size,"batches":args.batches,"threads":args.threads,
        "num_workers":args.num_workers,"concurrency":args.concurrency,
        "endpoint": os.environ.get("LUXI_URL","unset"), "duration_s": args.duration_s
    }

    if args.mode == "baseline":
        model = baseline_op
        meta = dict(meta_base); meta["transport"]="inproc"
    else:
        client = LuxiClient()
        code, text = client.health()
        if not (200 <= code < 300): raise RuntimeError(f"Luxi /health failed: {code} {text}")
        model = LuxiOp(args.expr, args.a, client, args.concurrency)
        meta = dict(meta_base); meta["transport"]=client.transport

    logger = BenchLogger(args.csv, meta=meta)

    # Warmup
    it = iter(dl)
    for _ in range(3):
        batch = next(it)[0].squeeze(-1).double()
        _ = baseline_op(batch, args.a) if args.mode == "baseline" else model(batch)

    # Timed
    tot = 0
    if args.duration_s > 0.0:
        it = iter(dl); t_start = time.perf_counter()
        while True:
            xb = next(it, None)
            if xb is None: it = iter(dl); xb = next(it)
            xb = xb[0].squeeze(-1).double()
            t0 = time.perf_counter()
            _ = baseline_op(xb, args.a) if args.mode == "baseline" else model(xb)
            t1 = time.perf_counter()
            logger.log(sample_count=xb.shape[0], elapsed_s=(t1 - t0))
            tot += xb.shape[0]
            if (t1 - t_start) >= args.duration_s: break
        print(f"Steady-state: {tot} samples in {time.perf_counter()-t_start:.3f}s => {(tot/max(1e-9,time.perf_counter()-t_start)):.1f} samples/s")
    else:
        t0 = time.perf_counter()
        for (xb,) in dl:
            xb = xb.squeeze(-1).double()
            t1 = time.perf_counter()
            _ = baseline_op(xb, args.a) if args.mode == "baseline" else model(xb)
            t2 = time.perf_counter()
            logger.log(sample_count=xb.shape[0], elapsed_s=(t2 - t1))
            tot += xb.shape[0]
        t3 = time.perf_counter()
        print(f"Done: {tot} samples in {t3-t0:.3f}s => {(tot/(t3-t0)):.1f} samples/s")
    logger.close()

if __name__ == "__main__":
    main()
