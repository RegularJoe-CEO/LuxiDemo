# Scripts

Public helpers that recompute or check published evidence. They do **not**
contain private engine source.

| Script | What it does |
|--------|----------------|
| [`verify_v99_pack.py`](verify_v99_pack.py) | Recomputes Version 99 matched-prefill medians and thr/J ratios from the retained per-run CSV under `evidence/h100-qwen2-7b-v99-matched-prefill-2026-07-23/`. |

```bash
python3 scripts/verify_v99_pack.py
```

## Related verification (not under scripts/)

| Check | How |
|-------|-----|
| Luxi Book output hash | Run a `luxi-book` binary on `downloads/luxibook/example_book.csv`; expect `output_vector_sha256` `4a21b1e708fa5c694bf48237df5e5bd3b94599e6273d07986283c6c6b8e3c97a` (see [`RESULTS.md`](../RESULTS.md)). The `lxq2_…` seal is per-install. |
| Book binary digests | `shasum -a 256 -c downloads/luxibook/*.sha256` |
| LuxiRisk vectors | `python3 luxirisk/test-vectors/verify_receipts.py` |
| LuxiRisk binary digests | `shasum -a 256 -c luxirisk/dist/*.sha256` |

Prefill thr/J packs under `evidence/prefill_*` and `evidence/version-100-h100-gtm/`
are JSON/markdown locks — open the pack `START_HERE` or `PUBLIC_*` briefs; no
single thr/J recompute script is published yet for those locks.
