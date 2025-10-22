# SPDX-FileCopyrightText: 2025 Eric Waller
# SPDX-License-Identifier: LicenseRef-eRock-Business-1.0

### eRock: Extend Missions with Fast Math

**Low‑power numeric computation for UAS and robotic edge nodes**

---

**THE CHALLENGE: EVERY WATT COUNTS IN THE FIELD**

Onboard processors for unmanned systems have tight power and weight budgets. Running navigation, sensors and decision logic drains batteries quickly. Using Python or heavy frameworks for simple math wastes precious mission time and payload mass.

**THE FIT: eROCK – MATH WITHOUT THE MISSION PENALTY**

`eRock` is a compiled Rust microservice that evaluates formulas and solves for thresholds in microseconds using SIMD. Its low CPU demand means your companion computer can do more with less energy and smaller hardware.

---

### Where it wins in UAS and robotics

- **Geofence and flight envelope** – Compute distance‑to‑boundary or time‑to‑breach using `erock_bisect_auto` while the flight computer focuses on control loops.
- **Battery and sensor health** – Evaluate real‑time thresholds to manage power and load shedding without writing custom math loops.
- **Payload decision logic** – Use `eRock` to calculate conditions for releasing or activating payloads based on environment factors.
- **Swarm coordination** – Execute simple numeric checks across multiple agents with minimal compute overhead, freeing up CPU for autonomy algorithms.

---

### Practical mission benefits

- **Longer flight time** – Save watts on computation so more battery is available for propulsion.
- **Smaller companion hardware** – Run your numeric guardrails on a less powerful ARM SBC, reducing size and weight.
- **Faster decision cycles** – Microsecond‑level calculations help your autonomy respond quickly to threats or mission changes.

---

### Lightweight math for heavy‑duty missions

`eRock` puts deterministic, low‑power numeric capability next to your sensors and actuators, giving you more endurance and headroom for mission‑critical functions.

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
