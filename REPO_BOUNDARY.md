# Repository boundary — LuxiDemo

This is a **public evaluation** repository. It is deliberately limited.

## Allowed here

| Kind | Examples |
|------|----------|
| Closed binaries | `downloads/luxibook/*`, `downloads/luxiedge-serve-*`, `luxirisk/dist/*` |
| Checksums | `*.sha256`, `SHA256SUMS` |
| Example inputs | `example_book.csv`, public test vectors |
| Evidence packs | `evidence/**` measured tables, JSON locks, attestations |
| Markdown docs | README, DEMOS, RESULTS, HISTORY, formulas (public math) |
| Public verifiers | `scripts/verify_v99_pack.py`, `luxirisk/test-vectors/verify_receipts.py` |

Evaluators can **run** binaries and **check** receipts. They cannot see proprietary
implementation source.

## Not allowed here

| Kind | Where it lives instead |
|------|------------------------|
| Marketing website HTML/CSS/assets | **Replit** project that deploys [luxiedge.com](https://luxiedge.com) |
| GitHub Pages deploy of a site tree | **Removed** — this repo does not host luxiedge.com |
| Proprietary engine source (quant, risk, inference) | **Private** repos only |
| Model weights, credentials, SSH/pod addresses | Never |
| “Universal bit-exact everywhere” claims without measured scope | Do not publish |

## Deploy model (do not confuse)

```text
Private engines  ──build──►  closed binaries  ──publish──►  LuxiDemo (GitHub)
                                                              │
                                                              ├── downloads/
                                                              ├── luxirisk/
                                                              └── evidence/

Replit  ──edit & publish──►  https://luxiedge.com
         (marketing site; may deep-link to GitHub binaries)
```

Updating the **website** is a **Replit agent / Replit deploy** job.  
Updating **public binaries and evidence** is a **LuxiDemo / GitHub** job.

## Product order (for any public surface)

1. **Luxi Book** — Quant sale try (CSV BS/Black-76 + output-vector hash + Ed25519 `lxq2_` receipt)  
2. **LuxiRisk** — freebie retail/crypto CLI  
3. **Inference thr/J + serve** — evidence / demoted demos  

## Link targets from docs

| Need | Path |
|------|------|
| Book binaries | [`downloads/luxibook/`](downloads/luxibook/) |
| Serve binaries | [`downloads/`](downloads/) |
| Risk freebie | [`luxirisk/`](luxirisk/) |
| How to run | [`DEMOS.md`](DEMOS.md) |
| Measured tables | [`RESULTS.md`](RESULTS.md) |
| Evidence index | [`evidence/README.md`](evidence/README.md) |
