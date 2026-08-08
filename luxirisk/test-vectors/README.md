# LuxiRisk test vectors

Public **input → expected output + receipt** examples for independent checks.

| File | Purpose |
|------|---------|
| [`vectors.json`](vectors.json) | Machine-readable suite (all v0.1 vectors) |
| [`verify_receipts.py`](verify_receipts.py) | Recompute SHA-256 of each canonical payload (no binary needed) |

## Quick verify (no binary)

```bash
python3 verify_receipts.py
```

## Verify with the binary

Download the platform binary from the
[luxirisk-v0.1 release](https://github.com/RegularJoe-CEO/LuxiDemo/releases/tag/luxirisk-v0.1),
check its `.sha256`, then:

```bash
./luxirisk-macos-arm64 vectors
./luxirisk-macos-arm64 liq --side long --entry 65000 --leverage 10
# short receipt must be: a896b6f35054
```

See [FORMULAS.md](../FORMULAS.md) for exact math and receipt canonicalization.
