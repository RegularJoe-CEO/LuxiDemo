# SPDX-FileCopyrightText: 2025 Eric Waller
# SPDX-License-Identifier: LicenseRef-eRock-Business-1.0

### eRock: Efficient Math for Smarter Transportation

**Compute thresholds and solves without draining vehicle systems**

---

**THE CHALLENGE: EMBEDDED VEHICLE SYSTEMS ARE RESOURCE‑CONSTRAINED**

Connected vehicles and trains rely on on‑board compute for monitoring and safety. Processing limits and battery or alternator capacity leave little room for inefficient numeric algorithms.

**THE FIT: eROCK – SLIM COMPUTE FOR VEHICLE LOGIC**

`eRock` is a compiled Rust microservice that runs on automotive‑grade CPUs to handle numeric evaluations and root finding with minimal compute cost. It helps OEMs reduce hardware requirements and extend electric or hybrid vehicle range.

---

### Where it wins in transportation

- **Range estimation & battery management** – Solve for remaining range and charge thresholds quickly to update drivers and control systems in real time.
- **Engine and drivetrain monitoring** – Evaluate formulas on temperature, vibration and fluid data to trigger service alerts without adding CPU overhead.
- **Rail signaling & braking** – Compute braking curves and stopping distances via root solving on embedded controllers.
- **Driver assistance & ADAS** – Use numeric checks on sensor‑derived metrics as part of pre‑processing before invoking heavier algorithms.

---

### Practical mobility benefits

- **More miles per charge or tank** – Lower compute demand on ECUs translates to marginal energy savings, adding up over large fleets.
- **Smaller ECUs** – Reduced CPU requirements allow for cost‑effective controllers and simplified thermal design.
- **Faster response** – Deterministic processing ensures timely safety decisions without requiring bigger processors.

---

### Lighten the load in your vehicles

With `eRock`, automotive and rail systems can handle numeric logic quickly, saving energy for motion and improving overall efficiency.
