# SPDX-FileCopyrightText: 2025 Eric Waller
# SPDX-License-Identifier: LicenseRef-eRock-Business-1.0

### eRock: Power‑Smart Math for Agriculture & Environmental Monitoring

**Get longer field life from your sensors and controllers**

---

**THE CHALLENGE: REMOTE DEVICES RUN ON TINY POWER BUDGETS**

Agricultural IoT and environmental monitoring often rely on battery‑ or solar‑powered nodes. Every extra CPU cycle shortens deployment life or requires bigger batteries, raising costs.

**THE FIT: eROCK – ULTRA‑EFFICIENT CALCULATION FOR THE FIELD**

`eRock` is a tiny Rust service that evaluates formulas and solves for thresholds using SIMD. It runs on low‑power microprocessors embedded in sensors or gateways, enabling longer deployments and smaller hardware.

---

### Where it wins in agri‑tech

- **Irrigation control** – Evaluate moisture and weather formulas locally to decide when to water, without waking a cloud service.
- **Crop and soil monitoring** – Apply nutrient or growth models to sensor data on‑site, filtering out normal fluctuations.
- **Environmental alarms** – Compute air quality or radiation thresholds quickly so sensors can sleep longer between transmissions.
- **Livestock telemetry** – Evaluate health metrics on collars or ear tags without draining small batteries.

---

### Practical field benefits

- **Longer deployment cycles** – Cut compute draw, extending battery life and reducing maintenance visits.
- **Smaller enclosures** – Lower power consumption allows for compact and lightweight sensor packages.
- **Reduced data transmission** – Local filtering means less radio use, further saving battery.

---

### Keep your sensors smart and lasting

Use `eRock` to perform numeric evaluations right at the edge, conserving energy and making your remote systems more efficient.

### Performance Proof

| Metric (100 000 evaluations) | scalar_100k | simd_100k_f64x4 |
| --- | --- | --- |
| Mean time | 7.1040 ms | **0.51743 ms** |
| Throughput gain | — | **≈13.7× faster** |

**Benchmark context:** Apple M1 Pro (8-core CPU), macOS 14.5. Command: `cargo bench --bench simd_vs_scalar -- --sample-size 20`. Sample-size trimmed to 20 for rapid, statistically stable runs.

