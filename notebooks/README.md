# Luxi Edge Orbital Mechanics Notebooks

Reproducible Jupyter notebooks demonstrating orbital ensemble generation and convergence analysis for synthetic benchmarks.

## Overview

This directory contains Python notebooks that generate and analyze synthetic orbital ensembles with J2 perturbations, providing reproducible benchmarks for the Luxi Edge orbital mechanics capabilities.

## Notebooks

### 1. `orbit_convergence_analysis.py`
**Convergence Curves vs Scalar Baseline**

Demonstrates SIMD-optimized performance compared to scalar baseline for orbital propagation:
- Performance scaling with swarm size (10-1000 satellites)
- SIMD speedup factors (3-4× typical)
- Real-time capability analysis (<1ms threshold)
- J2 perturbation overhead quantification

**Outputs:**
- `convergence_analysis.png` - Performance comparison plots
- `realtime_analysis.png` - Real-time capability visualization
- `performance_summary.csv` - Tabular performance data

### 2. `leo_swarm_benchmark.py`
**LEO Swarm Ensemble Generation**

Generates diverse Low Earth Orbit satellite ensembles for testing:
- 1000 satellite synthetic swarm
- Altitude: 200-2000 km
- Inclination: 0-100°
- Eccentricity: 0-0.05 (near-circular)
- J2 perturbation analysis

**Outputs:**
- `leo_swarm_distributions.png` - Orbital parameter distributions
- `leo_swarm_3d.png` - 3D visualization of swarm
- `j2_perturbation_analysis.png` - J2 effects on orbital precession
- `leo_swarm_ensemble.csv` - Full swarm dataset
- `leo_swarm_summary.json` - Summary statistics

## Setup

### Install Dependencies

```bash
# From repository root
pip install -r notebooks/requirements.txt
```

### Run Notebooks

**Option 1: As Python Scripts**
```bash
python notebooks/orbit_convergence_analysis.py
python notebooks/leo_swarm_benchmark.py
```

**Option 2: Convert to Jupyter Notebooks**
```bash
# Install jupytext if needed
pip install jupytext

# Convert to .ipynb
jupytext --to ipynb notebooks/orbit_convergence_analysis.py
jupytext --to ipynb notebooks/leo_swarm_benchmark.py

# Launch Jupyter
jupyter notebook notebooks/
```

**Option 3: Use JupyterLab**
```bash
# JupyterLab can execute .py files directly
jupyter lab notebooks/
```

## Integration with Rust Benchmarks

The notebooks integrate with Criterion benchmarks:

```bash
# Run orbital ensemble benchmarks
cargo bench --bench orbit_ensemble_benchmark

# Results stored in target/criterion/
# Notebooks can parse and visualize these results
```

## Reproducibility

All notebooks use fixed random seeds for reproducible results:
- `np.random.seed(42)` for synthetic data generation
- Criterion benchmark saves baselines for comparison
- CSV/JSON exports enable cross-validation

## Use Cases

### SpaceX Starlink
- Collision avoidance for 5000+ satellite constellation
- Multi-revolution Lambert transfers
- Orbital precession modeling with J2

### Tesla Autopilot/FSD
- Multi-agent trajectory optimization
- Real-time swarm coordination (<10ms)
- Probabilistic motion planning

### Optimus
- Formation control for robot swarms
- 1kHz control loops (<1ms timesteps)
- Energy-aware planning with battery constraints

### Drone Swarms
- Real-time coordination for 100s of UAVs
- Collision avoidance algorithms
- Mission planning under uncertainty

## Performance Targets

| Swarm Size | SIMD Time | Real-time? | Application |
|-----------|-----------|------------|-------------|
| 10 sats | ~100 μs | ✓ <1ms | Robot formations |
| 50 sats | ~300 μs | ✓ <1ms | Drone swarms |
| 100 sats | ~600 μs | ✓ <1ms | LEO constellation subset |
| 500 sats | ~3 ms | ✗ >1ms | Full constellation (batch) |
| 1000 sats | ~10 ms | ✗ >1ms | Offline analysis |

## Documentation

See main documentation for implementation details:
- [BENCHMARK_DATA.md](../BENCHMARK_DATA.md) - Performance metrics
- [docs/XAI_EXECUTIVE_SUMMARY.md](../docs/XAI_EXECUTIVE_SUMMARY.md) - xAI use cases
- [IMPLEMENTATION_SUMMARY.md](../IMPLEMENTATION_SUMMARY.md) - Technical details

## Citation

If using these benchmarks in research:

```bibtex
@software{luxiedge2025,
  title = {Luxi Edge Orbital Mechanics Benchmarks},
  author = {Waller, Eric},
  year = {2025},
  url = {https://github.com/RegularJoe-CEO/LuxiEdge},
  note = {Synthetic benchmarks for orbital ensemble propagation}
}
```

## License

SPDX-License-Identifier: LicenseRef-Luxi-Business-1.0

---

**Last Updated:** 2025-11-10  
**Status:** Production-ready synthetic benchmarks  
**Contact:** GitHub Issues for questions/feedback
