# SPDX-FileCopyrightText: 2025 Eric Waller
# SPDX-License-Identifier: LicenseRef-eRock-Business-1.0

### eRock: More Compute, Less Energy

**A numeric sidecar that cuts power usage in your racks**

---

**THE CHALLENGE: SERVERS CHEW POWER ON TINY TASKS**

Data centers run countless microservices that spend CPU time on simple math—thresholds, conversions and small equations. Interpreted runtime overhead multiplies across your fleet, wasting watts and generating heat.

**THE FIT: eROCK – NATIVE SPEED, MINIMAL POWER**

`eRock` is compiled Rust code with SIMD acceleration. It evaluates formulas on arrays and finds roots so quickly that CPUs return to idle faster. Across thousands of servers, these seconds add up to significant power and cooling savings, not to mention extended hardware lifespan.

---

### Where it wins in the data center

- **Real‑time pricing & bidding** – Compute ad targeting scores or pricing factors in microseconds; handle more bids per core.
- **Risk and portfolio engines** – Evaluate thousands of formulas quickly on CPU; slash per‑request latency and energy draw.
- **Telemetry & stream gateways** – Filter sensor or log data before queuing; reduce CPU budgets for log ingest pipelines.

---

### Practical facility benefits

- **Lower energy bills per rack** – Less CPU load equals less power and less cooling; translate savings directly to operating budget.
- **Higher density** – With cooler CPUs, you can pack more compute nodes in the same footprint.
- **Longer hardware life** – Reduced thermal stress extends component longevity, cutting replacement costs.

---

### Unlock efficiency at scale

Deploy `eRock` as a sidecar or service to handle the numeric chores and reclaim wasted cycles across your fleet.

### Performance Proof

| Metric (100 000 evaluations) | scalar_100k | simd_100k_f64x4 |
| --- | --- | --- |
| Mean time | 7.1040 ms | **0.51743 ms** |
| Throughput gain | — | **≈13.7× faster** |

**Benchmark context:** Apple M1 Pro (8-core CPU), macOS 14.5. Command: `cargo bench --bench simd_vs_scalar -- --sample-size 20`. Sample-size trimmed to 20 for rapid, statistically stable runs.

