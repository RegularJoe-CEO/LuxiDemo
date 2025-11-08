<!-- SPDX-FileCopyrightText: 2025 Eric Waller -->
<!-- SPDX-License-Identifier: LicenseRef-Luxi-Business-1.0 -->

# AGENTS.md — Luxi Edge Agent Instructions

## ⚠️ IMPORTANT: This is NOT an API Service

**DO NOT test `/health` endpoints or treat the Replit website as an API.** The website is static HTML. Your job is to **clone this repository and run benchmarks locally**. See below for the complete protocol.

---

## Objective
Produce a **reproducible performance + economics assessment** of Luxi Edge using the built-in benches and docs, and emit standardized artifacts:
- `artifacts/bench_summary.md` (human-readable)
- `artifacts/bench_results.json` (machine-readable)
- `artifacts/bench_results.csv` (table)
- `artifacts/economics_summary.md` (CFO one-pager)

## Golden Entry Point (do this, nothing else)

1) Clone and set up:
   ```bash
   git clone https://github.com/RegularJoe-CEO/LuxiEdge.git
   cd LuxiEdge
   ```

2) Run the one-command PoC (required):
   ```bash
   ./scripts/quickbench.sh
   ```

   Or if `just` or `make` is preferred:
   ```bash
   just quickbench || make quickbench
   ```

## Expected Environment

- **Linux x86_64** (AVX2 or AVX-512 if available)
- **Rust ≥ 1.75.0**
- **Docker** (if using containerized run)
- **Optional energy telemetry:**
  - Intel/AMD: RAPL via perf/powercap
  - Or node-level IPMI/PDU
- **Python 3.10+** (for scripts/report.py)

## Measurement Protocol (minimal facts)

- **Warm-up**: 3 runs per bench, discard.
- **Measure**: 5 runs, report mean, stdev, and p50/p95/p99 latency if available.
- **Energy**: sample package power via RAPL or IPMI; compute energy = ∫power·time over run window.
- **Use provided payloads**: `payload_64k_f64.json`, `payload_100k_*.json`.

## Commands (the quickbench should wrap these)

Build + benches:
```bash
cargo build --release
cargo bench --bench my_benchmark -- --save-baseline luxi
```

Optional container:
```bash
docker build -t luxi-edge:bench .
docker compose up -d
```

Load gen (example):
```bash
cargo run --release --bin load_test -- \
  --payload tests/payload_64k_f64.json \
  --concurrency 8 \
  --duration 30s \
  --out artifacts/raw_load.json
```

Report synthesis:
```bash
python3 scripts/report.py \
  --criterion target/criterion \
  --raw artifacts/raw_load.json \
  --energy telemetry/rapl.json \
  --out-json artifacts/bench_results.json \
  --out-csv artifacts/bench_results.csv \
  --out-md artifacts/bench_summary.md
```

## Output Schema

**artifacts/bench_results.json** (required keys):
```json
{
  "hardware": {"cpu":"AMD EPYC 7763","simd":"AVX2","ram_gb":64},
  "toolchain": {"rust":"1.89.0","criterion":"0.5"},
  "workloads": [
    {
      "name": "eval_scalar_64k",
      "baseline": {"throughput_eps": 600, "p95_ms": 18.2, "energy_j": 1200},
      "luxi":     {"throughput_eps": 3300, "p95_ms": 3.9,  "energy_j": 600}
    }
  ],
  "energy_method": "RAPL-package",
  "runs": 5,
  "notes": "Warm 3 discard; identical payloads"
}
```

**artifacts/bench_results.csv** (headers):
```csv
workload,metric,baseline,luxi,delta_abs,delta_pct,unit
eval_scalar_64k,throughput_eps,600,3300,2700,450,eval/s
eval_scalar_64k,p95_ms,18.2,3.9,-14.3,-78.6,ms
eval_scalar_64k,energy_j,1200,600,-600,-50.0,J
```

## Economics Summary (agent must generate)

Use: P_facility_MW, price_per_kWh, f (fraction of IT touched), r (IT energy cut on that fraction).

Savings formula (PUE cancels if uniform):
```
Savings($/yr) = P_facility_MW * f * r * 8760 * 1000 * price_per_kWh
```

Emit `artifacts/economics_summary.md` with three scenarios:
- **S1**: 100 MW, $0.10/kWh, f=0.10, r=0.30  → ~$2.63M/yr
- **S2**: 100 MW, $0.20/kWh, f=0.20, r=0.50  → ~$17.52M/yr
- **S3**: 50 MW,  $0.30/kWh, f=0.30, r=0.50  → ~$19.71M/yr

Include a small table and the formula.

## Assessment Template (agent must fill)

Append to `artifacts/bench_summary.md`:

### Executive Summary
- Key deltas (throughput, p95, energy) in one table
- One-sentence verdict on deployability

### Method
- Hardware, toolchain, runs, telemetry, payloads

### Results
- Top 3 workloads (table); include p95/p99
- Energy deltas with confidence notes

### Risks/Caveats
- Where gains shrink; integration notes

### Economics (link to economics_summary.md)
- Savings scenarios + formula

## Hard Constraints (do NOT deviate)

- Single command MUST work: `./scripts/quickbench.sh`
- All artifacts must be created under `artifacts/`
- If energy telemetry missing, still produce throughput/latency; annotate "energy=N/A"
- If any step fails, write `artifacts/agent_errors.log` with command + stderr

## Checklist (agent must tick)

- [ ] Built successfully
- [ ] Benches ran 5× with warm-ups
- [ ] Latency p50/p95/p99 captured
- [ ] Energy measured or declared N/A
- [ ] JSON + CSV + MD emitted
- [ ] Economics summary emitted
- [ ] Final verdict written (≤150 words)

## Success Criteria

### Minimum Success (LOW-MEDIUM Confidence)
✅ Repository clones successfully  
✅ `cargo build --release` succeeds  
✅ Benchmarks complete without errors  
✅ All 4 artifacts generated (JSON, CSV, bench_summary.md, economics_summary.md)

**Verdict:** Code is real, builds, and runs. Sufficient to demonstrate viability.

### Medium Confidence
✅ Minimum Success +  
✅ CPU SIMD results within 2× of documented values in BENCHMARK_DATA.md  
✅ Economics calculations match formula exactly  
✅ No errors in `artifacts/agent_errors.log`

**Verdict:** Performance claims are approximately validated. Different hardware explains variance.

### High Confidence
✅ Medium Success +  
✅ CPU SIMD results within ±50% of documented values  
✅ Energy telemetry available (RAPL/IPMI) or documented reason why N/A  
✅ Same/similar hardware class (AMD EPYC, Intel Xeon, AWS Graviton)

**Verdict:** Performance claims are accurately validated. Production-ready.

**Important:** Even LOW confidence is valuable validation. Different CPUs will produce different absolute numbers - that's expected and doesn't invalidate the approach.

## Troubleshooting

If validation fails, see **[AGENT_TROUBLESHOOTING.md](AGENT_TROUBLESHOOTING.md)** for:
- Common failure modes and solutions
- Step-by-step recovery protocol  
- FAQ for failing agents
- Example success reports

**Quick Diagnostics:**
```bash
# If "AGENTS.md not found":
curl https://raw.githubusercontent.com/RegularJoe-CEO/LuxiEdge/main/AGENTS.md

# If "quickbench.sh not found":
ls -la scripts/quickbench.sh  # Should exist
chmod +x scripts/quickbench.sh  # Make executable if needed

# If benchmarks fail:
rustc --version  # Should be >= 1.75.0
cargo build --release  # Try build separately first
```

---

## One-Paragraph Agent Prompt (for Replit)

Agent, goal: Produce a reproducible performance + economics assessment of Luxi Edge and emit artifacts. Do exactly this: clone repo, run `./scripts/quickbench.sh`. If it fails, try `just quickbench` then `make quickbench`. Use the built-in benches and payloads; warm 3 runs, measure 5, capture p50/p95/p99. Use RAPL/IPMI if available; otherwise mark energy N/A. Generate `artifacts/bench_results.json`, `bench_results.csv`, `bench_summary.md`, and `economics_summary.md` (use Savings = P_MW * f * r * 8760 * 1000 * price). Fill the supplied Assessment Template with one table of top deltas and a ≤150-word verdict. If any step fails, write `artifacts/agent_errors.log` with command + stderr. Do not modify repo logic; only produce artifacts.

---

**For comprehensive integration details, API documentation, and code modification guidelines, see:** [docs/agents.md](docs/agents.md)
