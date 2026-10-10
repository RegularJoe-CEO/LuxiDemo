"""Drive repro_logged.py across the issue's grid.

Usage: run_grid.py ENGINE MODE RUNS OUTDIR
ENGINE is a label such as vllm-0.27.1. MODE is standard or batch_invariant.
A cell whose requests hit a transport error is discarded and retried.
"""
import json, os, subprocess, sys

CELLS = [(8, 400), (16, 400), (32, 500), (1, 100)]
HERE = os.path.dirname(os.path.abspath(__file__))
LOGGED = os.path.join(HERE, "repro_logged.py")


def main():
    engine, mode, runs, outdir = sys.argv[1], sys.argv[2], int(sys.argv[3]), sys.argv[4]
    os.makedirs(outdir, exist_ok=True)
    summary = []
    for run in range(1, runs + 1):
        for conc, n in CELLS:
            meta = {"engine": engine, "mode": mode, "conc": conc, "n": n, "run": run}
            dest = os.path.join(outdir, f"{engine}-{mode}-c{conc}-n{n}-r{run}.jsonl")
            ok = False
            for attempt in range(1, 4):
                print(f"START {dest} attempt={attempt}", flush=True)
                proc = subprocess.run(
                    [sys.executable, LOGGED, str(n), str(conc), dest, json.dumps(meta)],
                    text=True,
                )
                if proc.returncode == 0 and os.path.exists(dest):
                    ok = True
                    break
                if os.path.exists(dest):
                    os.remove(dest)
                print(f"RETRY {dest} rc={proc.returncode}", flush=True)
            if not ok:
                print(f"FAILED {dest}", flush=True)
                sys.exit(1)
            # Pull the DISTINCT line from the jsonl so the driver log stands alone.
            from collections import Counter
            outs = [json.loads(l)["completion"] for l in open(dest)]
            counts = Counter(outs)
            minority = sum(c for _, c in counts.most_common()[1:])
            row = {**meta, "distinct": len(counts), "minority": minority,
                   "minority_pct": 100.0 * minority / len(outs), "n_ok": len(outs)}
            summary.append(row)
            print("CELL", json.dumps(row), flush=True)
    path = os.path.join(outdir, f"summary-{engine}-{mode}.json")
    json.dump(summary, open(path, "w"), indent=1)
    print("WROTE", path)


if __name__ == "__main__":
    main()
