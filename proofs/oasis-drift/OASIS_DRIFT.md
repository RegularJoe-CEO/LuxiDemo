
# LuxiDemo: Oasis LMF reruns that give the same answer

**One portfolio, one model, ten runs on two machines. Stock Oasis gave a different output every time we ran it. With Luxi's deterministic reduction, all ten runs gave the same output, byte for byte.**

LuxiEdge tested the open-source Oasis LMF loss engine (oasislmf 2.5.8) on the public OasisPiWind model. As far as we can tell, we are the first to report this run-to-run drift, put a dollar figure on it, and show a single fixed result across different machines and worker counts. All inputs are public, so anyone can reproduce the stock drift.

We also ran a harsher test on a portfolio 8 times larger with extreme TIVs, and checked every portfolio loss total against an exact, correctly rounded reference. Stock Oasis got 97% of those totals wrong. Luxi got none wrong, and it produced the same output hash on the tested Linux x86-64 and Apple M1 Pro Docker/arm64 configurations and stated worker counts ([details below](#harsher-test-8-portfolio-extreme-tivs-checked-against-an-exact-reference)).

## Results at a glance

| | Stock oasislmf 2.5.8 (defaults) | With Luxi deterministic reduction |
|---|---|---|
| Distinct output hashes, 10 runs (2 machines × 5 runs) | 10 of 10 different | **1** (`fc7daea9d3eb5ad5`) |
| Same command rerun on the same machine | Different output | Identical |
| 1 vs 2 vs 4 workers | Different output | Identical |
| Linux x86 vs Apple M1 (Docker, linux/arm64) | Different output | Identical |
| Largest drift between stock runs, 100/250-yr OEP/AEP | $1,152 (250-yr GU AEP, on $1.74B) | $0 |
| Largest drift between stock runs, AAL | $12 (IL/RI, on $11.19M) | $0 |
| Warm runtime, 4 workers | 39 s (x86) / 39.4 s (M1) | 38 s (x86) / 33.4 s (M1) |

These drifts are small, around one part in a million, and they won't change a pricing or capital decision. What they do break is bit-for-bit reproducibility. Under stock defaults you can't show that a rerun matches the original exactly, so hash-based audit trails, regression tests and model-change sign-offs all have to fall back on tolerances.

## Test setup

- **Engine:** oasislmf 2.5.8 (pip `oasislmf[extra]`), Python 3.13.5, numpy 2.5.4, numba 0.68.0, pandas 3.0.6. This is the current Python kernel (gulmc, fmpy, summarypy, eltpy, pltpy, aalpy, lecpy).
- **Model:** OasisPiWind, commit `661272420f6c7552a28953b226cf2f029343ac4b`, run with its own `oasislmf.json` and `analysis_settings.json`. GUL, IL and RI are all on, with ORD outputs ELT/PLT/ALT/EPT/PSEPT, 10 samples, and the Philox random generator.
- **Exposure:** the full public sample `tests/inputs/SourceLocOEDPiWind.csv` (12,598 locations, 38,709 items), plus PiWind's account and RI files.
- **Machines:** Linux x86-64 (Intel Xeon, 8 vCPU), and an Apple M1 Pro running Docker linux/arm64 with the same Python and package versions.
- **Runs per configuration and machine:** `-n 1`, `-n 1` again, `-n 2`, `-n 4`, `-n 4` again.
- **Comparison:** sha256 over all 30 output CSVs (first 16 hex shown). Where files differed, we sorted rows by their key columns and compared again, to separate pure reordering from real numeric changes. In every stock default run the differences were numeric.

Random sampling isn't the cause. Oasis derives each random draw from (group_id, event_id), so sampling is fixed by design.

## Output hashes

| Configuration | Machine | n=1 | n=1 (repeat) | n=2 | n=4 | n=4 (repeat) |
|---|---|---|---|---|---|---|
| Stock, defaults | Linux x86 | d92c7399b80cdaed | 94a568beae70828e | 9474f92ef3795712 | ab46eddb5194932d | f824688a4c2e1ee1 |
| Stock, defaults | M1 (Docker arm64) | 55130cea150fb184 | e42c444c3f31f464 | df8ac6719746fbef | 8f3cd86722df7539 | b82db7bcf27617c0 |
| Stock, `--lookup-multiprocessing False` | Linux x86 | f1dd57047f7392bb | f1dd57047f7392bb | a51d367f11cd134c | 59b2fa4efdc8105b | 59b2fa4efdc8105b |
| Stock, `--lookup-multiprocessing False` | M1 (Docker arm64) | f1dd57047f7392bb | f1dd57047f7392bb | a51d367f11cd134c | 59b2fa4efdc8105b | 59b2fa4efdc8105b |
| **Luxi deterministic reduction** | **Linux x86** | **fc7daea9d3eb5ad5** | **fc7daea9d3eb5ad5** | **fc7daea9d3eb5ad5** | **fc7daea9d3eb5ad5** | **fc7daea9d3eb5ad5** |
| **Luxi deterministic reduction** | **M1 (Docker arm64)** | **fc7daea9d3eb5ad5** | **fc7daea9d3eb5ad5** | **fc7daea9d3eb5ad5** | **fc7daea9d3eb5ad5** | **fc7daea9d3eb5ad5** |

What the table shows:

1. **Stock defaults: every run differs.** The keys lookup runs in parallel and returns locations in a different order each time. That order sets `item_id`, and the engine sums losses in item order using float32 arithmetic. A new order gives new rounding, and so new numbers. An earlier x86 batch of five default runs also produced five new hashes, so stock defaults gave 15 distinct hashes from 15 runs.
2. **Stock with lookup multiprocessing turned off:** repeats now match, and x86 and M1 agree with each other. But each worker count still produces its own hash (n=1, n=2 and n=4 all differ), from last-digit (1 float32 ULP) differences in a few AEP rows.
3. **Luxi: one hash** across both machines, all worker counts and all repeats, with parallel lookup left on.

## Size of the stock drift (10 stock default runs, both machines)

| Metric | Largest difference between runs | On a value of | Relative |
|---|---|---|---|
| 250-yr GU AEP (mean damage) | $1,152 | $1.74B | ~7e-7 |
| 100-yr GU OEP (full uncertainty), x86 runs | $1,024 | $1.11B | 9.2e-7 |
| 100-yr RI AEP (mean damage) | $128 | $63.0M | 2.0e-6 |
| IL / RI AAL (analytical) | $12 | $11.19M | 1.1e-6 |
| GU AAL (analytical), x86 runs | $16 | $73.2M | 2.2e-7 |
| Single event loss (GU ELT), x86 runs | $4,352 | — | ~9e-6 |

In each stock default rerun, 27 of the 30 output files changed. Only the three summary-info files stayed the same.

## Speed

Times are wall-clock seconds for a full `oasislmf model run`. The first n=1 run includes numba compilation.

| | x86: n1 / n1 rep / n2 / n4 / n4 rep | M1 Docker: n1 / n1 rep / n2 / n4 / n4 rep |
|---|---|---|
| Stock defaults | 162 / 75 / 48 / 39 / 39 | 117.0 / 51.3 / 37.7 / 39.4 / 38.2 |
| Luxi deterministic reduction | 161 / 74 / 49 / 38 / 39 | 120.0 / 51.2 / 37.1 / 33.4 / 35.4 |

On warm runs the two are about equal on both machines. The differences are within run-to-run noise. Determinism here costs nothing measurable in speed.

## How Luxi's result compares with stock values

Luxi's fixed output isn't numerically identical to any stock run. Stock Oasis adds up item losses in float32, and Luxi's result differs from stock (lookup multiprocessing off, n=1) by:

- IL analytical AAL: $446 on $11.19M (4.0e-5). RI is similar.
- GU analytical AAL: $376 on $73.2M (5.1e-6).
- EP: up to $3,570 at the 250-yr IL OEP (mean damage), 1.1e-4 relative.
- Sampled AAL: within 1e-6.

**On this base PiWind portfolio, we haven't checked either set against an exact reference.** The harsher test below did make that check, on a larger and more extreme portfolio. There, the gap between stock and Luxi turned out to be stock's own float32 accumulation error: stock was wrong in 97% of portfolio totals, and Luxi matched the exact answer in every one. That makes it likely the base PiWind gap above is also stock error, but we haven't verified that on the base portfolio, so we don't claim it there.

Either way, teams moving to a deterministic, wide-accumulator reduction should expect a one-time shift of this size against their stock baseline.

## Reproduce the stock drift yourself

You need only public inputs and the stock engine. Luxi isn't required.

```bash
# 1. Environment (we used Python 3.13.5; full pinned package list available on request)
python3.13 -m venv venv && . venv/bin/activate
pip install "oasislmf[extra]==2.5.8"

# 2. Model and public exposure
git clone https://github.com/OasisLMF/OasisPiWind.git
cd OasisPiWind && git checkout 661272420f6c7552a28953b226cf2f029343ac4b

# 3. Run the same command twice
for r in a b; do
  oasislmf model run --config oasislmf.json \
    --oed-location-csv tests/inputs/SourceLocOEDPiWind.csv \
    -r ../run_$r -n 4
done

# 4. Hash the outputs
python3 - <<'PY'
import glob, hashlib, os
for r in ["../run_a", "../run_b"]:
    h = hashlib.sha256()
    for f in sorted(glob.glob(r + "/output/*.csv")):
        h.update(os.path.basename(f).encode()); h.update(open(f, "rb").read())
    print(r, h.hexdigest()[:16])
PY
```

Under defaults, the two hashes should differ. Add `--lookup-multiprocessing False` and repeats will match, but `-n 1`, `-n 2` and `-n 4` will still each give a different hash.

## Closed engine, open evidence

- **Luxi's deterministic reduction is proprietary.** We don't publish its method or source code.
- **The evidence is open.** The model, the exposure, the stock commands and the hashing method are all public. Anyone can confirm the stock drift independently, and anyone evaluating Luxi can check the `fc7daea9d3eb5ad5` result on their own hardware.
- **Evaluation:** insurers, reinsurers and cat-modelling teams can request an evaluation at **luxiedge.com**, on PiWind or on your own model and portfolio.
- **Commercial evaluation:** the **LuxiQuant Risk State Engine, Risk Runtime v0.4.0,** is LuxiEdge's only sellable product. It is offered only through a paid confidential evaluation under NDA. This Oasis result is evidence, not a claim that the public Oasis test is the Risk State Engine.

We don't claim LuxiEdge is the only way to make Oasis deterministic. We found the issue, measured it in dollars, and proved a single fixed result across machines. We've also reported the stock behaviour to the Oasis LMF maintainers, along with the public workaround.

---

## Harsher test: 8× portfolio, extreme TIVs, checked against an exact reference

The base test showed that stock Oasis drifts. The harsher test asks a stronger question: which answer is actually correct?

**Setup**
- **Exposure:** PiWind ×8, giving 100,784 locations. Every TIV was multiplied by a random power of ten (fixed seed), so building TIVs run from $100 to $5.0B, with tiny and huge values mixed in the same portfolio. That makes float32 rounding worse.
- **Engine and model:** the same oasislmf 2.5.8 and PiWind model, GUL/IL/RI, 10 samples.
- **Exact reference:** we streamed the full ground-up item-loss stream (9.3 GB) and recomputed every portfolio loss total, per event and sample, in Oasis's own summation order. Each total was computed three ways: stock float32 summation, Luxi's reduction, and an exact sum correctly rounded to float32, which is the precision Oasis writes. We validated the exact-sum harness against Python's exact rational arithmetic (`fractions`) on a synthetic stream. That gave 6,006 totals, each made up of up to 186,624 item losses.

**Correctness against the exact reference**

| | Published (float32) totals that differ from the exact answer |
|---|---|
| Stock oasislmf 2.5.8 | **5,851 of 6,006 (97%)** |
| Luxi deterministic reduction | **0 of 6,006** |

**What stock gets wrong, in dollars** (stock vs Luxi, x86, 4 processes; the exact check above shows the gap is stock's float32 accumulation error)

| Metric | Stock error | On a value of | Relative |
|---|---|---|---|
| GU analytical AAL | $61.0M | $364.7B | 1.67e-4 |
| GU sampled AAL | $47.9M | — | 1.3e-4 |
| IL AAL | $3,779 | $14.4M | 2.6e-4 |
| 100-yr IL/RI AEP (worst EP point) | $31,696 | — | 3.4e-4 |

**Same answer on every tested configuration**

| Run | Output hash (30 CSVs) |
|---|---|
| Luxi, Linux x86, 1 process | f07a3dbefad5b6f9 |
| Luxi, Linux x86, 4 processes | f07a3dbefad5b6f9 |
| Luxi, Apple M1 Docker (linux/arm64), 1 process | f07a3dbefad5b6f9 |

The M1 run at 4 processes ran out of Docker memory (Docker had 7.65 GB; the job was killed, exit 137), so the M1 result is at 1 process only.

**Internal precision**

Inside the reduction, Luxi's running totals were within $0.00006 of the exact double-precision sum in the worst case. Plain float64 accumulation was off by up to $0.154. Oasis writes its outputs in float32, though, and both errors are far smaller than one float32 step, so both round to the same published numbers. In fact, a fixed order with plain float64 accumulation gave the same `f07a3dbefad5b6f9` hash. To be clear: what fixes Oasis's published outputs is a fixed order plus a wide accumulator. Luxi's extra internal precision only shows up if Oasis ever writes double-precision outputs, which 2.5.8 can't do with PiWind's float32 model files.

**Speed (x86, 4 processes, warm)**

| | Seconds |
|---|---|
| Stock oasislmf 2.5.8 | 199 |
| Luxi deterministic reduction | 182 |

On the same portfolio, Luxi was correct, identical across machines, and no slower than stock.

---

## Scope and limits

- Two machines: Linux x86, and Apple M1 under Docker (linux/arm64). Native macOS wasn't part of these results.
- One public demo model (PiWind, 10 samples) and one Oasis version (2.5.8). Worker counts tested were 1, 2 and 4.
- Harsher test: the PiWind ×8 portfolio with scaled TIVs is a synthetic stress case, not a real book. The exact reference covers the 6,006 ground-up portfolio totals (per event and sample). The M1 harsh run is at 1 process only (4 processes ran out of Docker memory). The harsh timing comparison is x86 at 4 processes.
- The base PiWind values have not been checked against an exact reference.
- Differences were measured against the n=1 run in each set.
- Default-mode drift is fully explained by item ordering from the parallel lookup. With lookup multiprocessing off, the only remaining drift is in the EP (AEP) step.

*LuxiEdge. Test date 12 October 2026.*
