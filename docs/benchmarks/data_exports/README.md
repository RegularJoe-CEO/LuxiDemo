# Criterion Export Artifacts

This directory stores raw benchmark baselines produced with:

```bash
cargo bench -- --save-baseline current
```

Each baseline creates `criterion/` JSON summaries and HTML reports under this folder. Commit only the curated snapshots needed for
regressions or published analyses.
