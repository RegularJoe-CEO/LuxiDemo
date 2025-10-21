# SPDX-FileCopyrightText: 2025 Eric Waller
# SPDX-License-Identifier: LicenseRef-eRock-Business-1.0

### eRock: Fast Rates, Lean Hardware

**High‑speed calculations for modern insurance platforms**

---

**THE CHALLENGE: SLOW MATH HURTS YOUR BOTTOM LINE**

Underwriting decisions and usage‑based pricing rely on formula evaluations. Legacy rating engines written in Python, Excel or SQL chew through CPU cycles, delaying quotes and burning watts in your data center.

**THE FIT: eROCK – NATIVE RATING PERFORMANCE**

`eRock` is a microservice built in compiled Rust. It evaluates rating formulas and solves for thresholds or break‑even points in microseconds. Its efficiency means you handle more quotes on the same servers and reduce power consumption in your rating pipeline.

---

### Where it wins in InsurTech

- **Real‑time quotes** – Compute rating factors instantly on CPU; respond to customers without adding compute nodes.
- **Usage‑based scoring** – Process telematics or IoT features at the edge or server to update premiums without draining device batteries.
- **Catastrophe exposure** – Solve for deductible or capacity limits quickly to respond to rapidly changing events.
- **Fraud signals** – Evaluate numeric red flags immediately so you can flag suspicious claims with minimal infrastructure.

---

### Practical business benefits

- **Higher throughput** – Each server can handle more quotes per second.
- **Lower infrastructure cost** – Use cheaper CPU cores and less power to run your rating logic.
- **Faster market reaction** – Rapid adjustments to models don’t require capacity upgrades.

---

### Make rating your competitive advantage

`eRock` gives you high‑speed, low‑power numeric capability so your underwriting stays lean and responsive.

### Performance Proof

| Metric (100 000 evaluations) | scalar_100k | simd_100k_f64x4 |
| --- | --- | --- |
| Mean time | 7.1040 ms | **0.51743 ms** |
| Throughput gain | — | **≈13.7× faster** |

**Benchmark context:** Apple M1 Pro (8-core CPU), macOS 14.5. Command: `cargo bench --bench simd_vs_scalar -- --sample-size 20`. Sample-size trimmed to 20 for rapid, statistically stable runs.

