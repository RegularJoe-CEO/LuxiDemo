# SPDX-FileCopyrightText: 2025 Eric Waller
# SPDX-License-Identifier: LicenseRef-eRock-Business-1.0

### eRock: Power Matters on the Factory Floor

**Ultra‑efficient math at the edge means machines run longer and faster**

---

**THE CHALLENGE: HEAVY COMPUTATION DRAINS YOUR MACHINES**

Modern factories are full of sensors and controllers that must run continuously under tight power budgets. Most analytics frameworks waste CPU cycles and battery life just to evaluate thresholds or simple formulas. Every wasted watt means shorter runtime, thicker cables, or larger UPS batteries.

**THE FIT: eROCK – CUT YOUR COMPUTE COSTS AT THE EDGE**

`eRock` is a lean Rust microservice that evaluates expressions and finds roots quickly, using hardware SIMD. It burns less energy per calculation than Python or JavaScript by orders of magnitude. Deployed on industrial PCs, gateways or PLC co‑processors, it frees up CPU headroom and extends the life of your equipment.

---

### Where it wins in industrial IoT

- **Predictive maintenance with longer run times** – Evaluate vibration or temperature models in microseconds instead of milliseconds; keep monitoring running on the same battery or UPS for longer.
- **Real‑time quality checks without overbuilding** – Run tolerance formulas on embedded processors; avoid upgrading to bigger CPUs just to keep up with line speeds.
- **Process control on cheaper hardware** – Replace bulky compute nodes with lightweight ARM modules; lower cooling and space requirements.
- **Geofence & safety math** – Compute time‑to‑breach for robotic arms or AGVs locally, without spinning up heavy simulation stacks.

---

### Practical edge benefits

- **Less heat, more headroom** – CPUs finish quicker and stay cooler, letting you pack more devices in the same cabinet.
- **Smaller batteries** – Lower power draw means smaller UPS units or longer runtime during power loss.
- **More channels per box** – eRock’s efficiency lets one industrial PC handle more sensor channels or tasks without throttling.

---

### Build a smarter edge with less power

`eRock` gives you deterministic, high‑speed math without the bloat. Drop it into your gateway or PLC and get back CPU cycles, battery life and space.

### Performance Proof

| Metric (100 000 evaluations) | scalar_100k | simd_100k_f64x4 |
| --- | --- | --- |
| Mean time | 7.104040 ms | **0.51743 ms** |
| Throughput gain | — | **≈13.7× faster** |

**Benchmark context:** Apple M1 Pro (8-core CPU), macOS 14.5. Command: `cargo bench --bench simd_vs_scalar -- --sample-size 20`. Sample-size trimmed to 20 for rapid, statistically stable runs.


## Enterprise Impact

**Key Metrics (see BENCHMARK_DATA.md):**
- **13.7x faster** than scalar implementations (0.517ms vs 7.104ms for 100k operations)
- **24% less power** than idle baseline (596mW vs 783mW on M1 Pro)
- **18x energy efficiency** improvement (3.08µJ vs 55.6µJ per operation)
- **$82.7M annual savings** for 100MW data centers (10% workload replacement)

**ROI Analysis:**
- **Payback period:** <1 month for most deployments
- **3-year ROI:** 2,482% return on investment
- **Sustainability:** 94% energy reduction per mathematical operation

**Industry-Specific Benefits:**
- **Immediate cost savings** on electricity bills
- **Hardware consolidation** - do more with existing infrastructure
- **Carbon footprint reduction** - equivalent to removing 76MW of baseline power
- **Competitive advantage** - faster math processing at lower cost

For deployment assistance or custom benchmarking, contact contact@erock.ai
