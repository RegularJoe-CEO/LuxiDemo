# LuxiRisk test vectors (v0.2)

Public **input → expected output + Ed25519-signed `lxr1_` receipt** examples.

| File | Purpose |
|------|---------|
| [`vectors.json`](vectors.json) | Machine-readable suite |
| [`verify_receipts.py`](verify_receipts.py) | Offline signature check (Python + cryptography/PyNaCl) |

## Quick verify (no LuxiRisk binary)

```bash
pip install cryptography
python3 verify_receipts.py
```

## Verify with the binary

```bash
./luxirisk-macos-arm64 vectors
./luxirisk-macos-arm64 verify 'lxr1_…' liq \
  --side long --entry 65000 --leverage 10 --expect-liq 58825
```

## Test-vector identity

Vectors are signed with a **fixed documentation seed**, not a user install key:

```text
seed = SHA-256("luxirisk-v0.2-test-vector-identity")
```

Fingerprint and public key are recorded in `vectors.json` under
`test_vector_identity`.

See [FORMULAS.md](../FORMULAS.md) for the full receipt byte layout.
