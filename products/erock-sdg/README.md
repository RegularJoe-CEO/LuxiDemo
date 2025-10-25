<!-- SPDX-FileCopyrightText: 2025 Eric Waller -->
<!-- SPDX-License-Identifier: Proprietary -->

# Erock SDG™ — Software-Defined Generator

**Scope:** **Includes SMBs** and scales to mid-sized enterprises.

Make a site behave like a generator at the meter using software:
- Optimize compute timing and loads
- Enforce safety/comfort constraints (anti-short-cycle, temp guardrails)
- Commit dispatch profiles with generator-grade M&V

**SMB example (illustrative):**
- 25 kW peak; ~20 kW flexible
- ~$2,800/year; ~$7,000 install; ~2.5-year payback

**Enterprise example (illustrative):**
- 50 MW site; 5 MW flexible; optional 1.2 MW WHP
- ~$1.2M/year; <2-year payback

Architecture: market-agnostic dispatch core + thin adapters for price/baseline/settlement APIs.
