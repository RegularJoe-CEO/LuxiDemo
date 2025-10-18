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
