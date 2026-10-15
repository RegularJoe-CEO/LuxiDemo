# Repository boundary - LuxiDemo

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
| Official marketing website HTML/CSS/assets | **Replit** project that deploys [luxiedge.com](https://luxiedge.com) |
| Repository `site/` tree | Secondary GitHub Pages demo and evidence index. It is not luxiedge.com and must follow the official site's product language |
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

## Product order for public surfaces

1. **LuxiEdge** is the company.
2. **LuxiQuant** is the primary commercial division. **LuxiEnergy** is the secondary division.
3. **LuxiQuant Risk State Engine, Risk Runtime v0.4.0**, is LuxiEdge's only sellable product. It is offered only through a paid confidential evaluation under NDA, with no public download or public price.
4. **LuxiBook v0.2.1** and **LuxiRisk v0.2** are the free demos.
5. Inference and the other measured families are evidence. Market applications and LuxiEnergy work beyond the named product are hypotheses or roadmap work, not shipping products.

## Link targets from docs

| Need | Path |
|------|------|
| Book binaries | [`downloads/luxibook/`](downloads/luxibook/) |
| Serve binaries | [`downloads/`](downloads/) |
| LuxiRisk legacy demo | [`luxirisk/`](luxirisk/) |
| How to run | [`DEMOS.md`](DEMOS.md) |
| Measured tables | [`RESULTS.md`](RESULTS.md) |
| Evidence index | [`evidence/README.md`](evidence/README.md) |
