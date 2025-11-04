<!-- SPDX-FileCopyrightText: 2025 Eric Waller -->
<!-- SPDX-License-Identifier: Proprietary -->

# Luxi™ (Luxi Edge™ / Luxi Core™)

High-performance microservice for ultra-fast numeric calculations. The same codebase works optimally on edge devices and data centers.

## What it does
- Expression evaluation with SIMD vectorization
- Root-finding algorithms (bisection with auto-bracketing)
- HTTP API for numeric calculations
- 13.7× faster than baseline, 18× better energy efficiency

## Optimized for Edge
- Runs exceptionally well on edge devices
- Low power consumption (596mW under load vs 783mW idle)
- 193k operations/second on edge hardware
- Minimal footprint

## Works Everywhere
- Edge devices (IoT, embedded systems)
- Data centers (computational offload)
- GPU-to-CPU offload scenarios
- Any platform needing rapid calculations

## API Endpoints
- `/evaluate` - Expression evaluation over vectors
- `/bisect` - Root finding with known bracket
- `/bisect_auto` - Root finding with automatic bracket search
- `/health` - Service health check
