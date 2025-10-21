# SPDX-FileCopyrightText: 2025 Eric Waller
# SPDX-License-Identifier: LicenseRef-eRock-Business-1.0

### eRock: Lean Math for Energy & Utilities

**Cut compute power to keep the power flowing**

---

**THE CHALLENGE: POWER INFRASTRUCTURE NEEDS MATH, NOT HEAT**

Electric grids, oil pipelines and gas networks rely on real‑time calculations to balance loads, detect anomalies and optimize flows. Running Python or JavaScript on gateway devices for thresholding or leak detection wastes energy and reduces reliability.

**THE FIT: eROCK – ULTRA‑EFFICIENT CALCULATION ENGINE**

`eRock` is a Rust microservice that evaluates numeric expressions and finds roots using SIMD. It runs on substation gateways, pipeline sensors and smart meters with minimal CPU cycles. Lower compute consumption means less heat, longer service life and smaller hardware.

---

### Where it wins in energy & utilities

- **Grid balancing** – Calculate load factors and thresholds in microseconds to avoid overloading transformers or feeders without oversizing your processors.
- **Leak detection** – Apply pressure‑drop formulas locally on pipeline sensors and trigger alarms without sending streams to the cloud.
- **Demand response** – Evaluate tariffs and consumption formulas quickly so you can adjust load shedding decisions on edge devices.
- **Microgrid control** – Solve for equilibrium points in battery and generation formulas using `bisect` in real time.

---

### Practical network benefits

- **Increased uptime** – Reduced CPU heat yields more reliable equipment in remote sites.
- **Less infrastructure cost** – Use smaller, more affordable controllers since math overhead is lower.
- **Faster response** – Real‑time calculations on‑site mean quicker actions in critical scenarios.

---

### Empower your energy systems with lean math

Deploy `eRock` at substations, along pipelines and in smart meters to perform critical calculations quickly and efficiently.

### Performance Proof

| Metric (100 000 evaluations) | scalar_100k | simd_100k_f64x4 |
| --- | --- | --- |
| Mean time | 7.1040 ms | **0.51743 ms** |
| Throughput gain | — | **≈13.7× faster** |

**Benchmark context:** Apple M1 Pro (8-core CPU), macOS 14.5. Command: `cargo bench --bench simd_vs_scalar -- --sample-size 20`. Sample-size trimmed to 20 for rapid, statistically stable runs.

