import os, time, argparse, numpy as np
from common_bench import BenchLogger
from luxi_client import LuxiClient

def main():
    try:
        import tensorflow as tf
    except Exception as e:
        raise SystemExit(f"TensorFlow not available: {e}")

    ap = argparse.ArgumentParser()
    ap.add_argument("--mode", choices=["baseline","luxi"], default="luxi")
    ap.add_argument("--expr", default="if x < 0.0 { sin(x) + a*x*x } else { log(1.0 + x) - sqrt(abs(x)) + 0.1*x*x*x }")
    ap.add_argument("--a", type=float, default=0.2)
    ap.add_argument("--batch-size", type=int, default=8192)
    ap.add_argument("--batches", type=int, default=200)
    ap.add_argument("--duration-s", type=float, default=0.0)  # steady-state mode
    ap.add_argument("--threads", type=int, default=1)
    ap.add_argument("--seed", type=int, default=1337)
    ap.add_argument("--csv", default="docs/benchmarks/tf_luxi.csv")
    args = ap.parse_args()

    os.environ["TF_NUM_INTRAOP_THREADS"] = str(args.threads)
    os.environ["TF_NUM_INTEROP_THREADS"] = str(max(1, args.threads // 2))
    try:
        tf.config.threading.set_intra_op_parallelism_threads(args.threads)
        tf.config.threading.set_inter_op_parallelism_threads(max(1, args.threads // 2))
    except Exception:
        pass

    rng = np.random.default_rng(args.seed)
    total_items = max(args.batch_size * args.batches, args.batch_size * 100)
    xs = rng.normal(0.0, 5.0, size=(total_items,)).astype(np.float64)
    ds = tf.data.Dataset.from_tensor_slices(xs).batch(args.batch_size, drop_remainder=False)

    def np_branch(xb):
        xb = xb.astype(np.float64)
        return np.where(
            xb < 0.0,
            np.sin(xb) + args.a * xb * xb,
            np.log1p(np.clip(xb, -0.999999, None)) - np.sqrt(np.abs(xb)) + 0.1 * xb * xb * xb
        )

    if args.mode == "baseline":
        def map_fn(x):
            return tf.numpy_function(np_branch, [x], tf.float64)
    else:
        client = LuxiClient()
        code, text = client.health()
        if not (200 <= code < 300): raise RuntimeError(f"Luxi /health failed: {code} {text}")
        def _call(xb):
            x_np = xb.numpy().astype(np.float64).ravel().tolist()
            ys, _ = client.evaluate_batch(args.expr, x_np, vars={"a": float(args.a)})
            return tf.convert_to_tensor(ys, dtype=xb.dtype)
        def map_fn(x):
            y = tf.py_function(lambda t: _call(t), [x], tf.float64)
            y.set_shape(x.shape)
            return y

    ds = ds.map(map_fn, num_parallel_calls=1)
    # Warmup 3 batches
    for _ in ds.take(3):
        pass

    # Timed
    logger = BenchLogger(args.csv, meta={
        "framework":"tensorflow","mode":args.mode,"expr":args.expr,"a":args.a,
        "batch_size":args.batch_size,"batches":args.batches,"threads":args.threads,
        "endpoint": os.environ.get("LUXI_URL","unset"), "duration_s": args.duration_s
    })

    if args.duration_s > 0.0:
        ds2 = ds.repeat()
        it = iter(ds2)
        t_start = time.perf_counter()
        total = 0
        while True:
            batch = next(it)
            t1 = time.perf_counter()
            _ = batch.numpy()
            t2 = time.perf_counter()
            logger.log(sample_count=batch.shape[0], elapsed_s=(t2 - t1))
            total += batch.shape[0]
            if (t2 - t_start) >= args.duration_s:
                break
        print(f"Steady-state: {total} samples in {time.perf_counter()-t_start:.3f}s => {total/max(1e-9,time.perf_counter()-t_start):.1f} samples/s")
    else:
        total = 0
        t0 = time.perf_counter()
        for batch in ds:
            t1 = time.perf_counter()
            _ = batch.numpy()
            t2 = time.perf_counter()
            logger.log(sample_count=batch.shape[0], elapsed_s=(t2 - t1))
            total += batch.shape[0]
        t3 = time.perf_counter()
        print(f"Done: {total} samples in {t3-t0:.3f}s => {(total/(t3-t0)):.1f} samples/s")
    logger.close()

if __name__ == "__main__":
    main()
