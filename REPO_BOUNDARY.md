# Repository boundary - LuxiDemo

This is a **public evaluation** repository. It is deliberately limited.

## Allowed here

| Kind | Examples |
|------|----------|
| Closed binaries | `downloads/luxi-replay-demo/*` (inference replay package), `downloads/luxibook/*`, `downloads/luxiedge-serve-*` (historical), `luxirisk/dist/*` |
| Checksums | `*.sha256`, `SHA256SUMS` |
| Example inputs | `example_book.csv`, public test vectors |
| Evidence packs | `evidence/**` measured tables, JSON locks, attestations |
| Markdown docs | README, DEMOS, RESULTS, HISTORY, formulas (public math) |
| Public verifiers | `evidence/r1-distill-7b-gpu-swap-2026-11-09/verify/luxi_verify.py`, `scripts/verify_v99_pack.py`, `luxirisk/test-vectors/verify_receipts.py` |
| Legacy Pages page | `site/` (a pointer page only; see below) |

Evaluators can **run** binaries and **check** receipts. They cannot see proprietary
implementation source.

## Not allowed here

| Kind | Where it lives instead |
|------|------------------------|
| Marketing website HTML/CSS/assets | **Replit** project that deploys [luxiedge.com](https://luxiedge.com) |
| Proprietary engine source (quant, risk, inference) | **Private** repos only |
| Signing private keys | Never (public keys only) |
| Model weights, credentials, SSH/pod addresses | Never |
| “Universal bit-exact everywhere” claims without measured scope | Do not publish |

## GitHub Pages (still deployed)

GitHub Pages is **not** removed. `.github/workflows/pages.yml` deploys the `site/` folder to
https://regularjoe-ceo.github.io/LuxiDemo/ on every push to `main`. That page is a legacy pointer: it sends visitors to
[luxiedge.com/demo](https://luxiedge.com/demo) and to [`DEMOS.md`](DEMOS.md), and links the historical v3.0 release by
its pinned tag. It must not use `releases/latest/download/...` links, which resolve to whichever release is newest.
It does not host luxiedge.com.

## Deploy model (do not confuse)

```text
Private engines  ──build──►  closed binaries  ──publish──►  LuxiDemo (GitHub)
                                                              │
                                                              ├── downloads/  (incl. luxi-replay-demo/)
                                                              ├── luxirisk/   (LuxiSize)
                                                              └── evidence/

Replit  ──edit & publish──►  https://luxiedge.com
         (marketing site; may deep-link to GitHub binaries)
```

Updating the **website** is a **Replit agent / Replit deploy** job.  
Updating **public binaries and evidence** is a **LuxiDemo / GitHub** job.

## Products and what is public (for any public surface)

LuxiEdge is one core engine with seven commercial products (luxiedge.com): Inference, Haplotastic, LuxiQuant Risk State
Engine, Orbital, Catastrophe, Edge and LuxiEnergy. They are not equally validated, and most have no public binary. Never
imply that all seven have public downloads.

1. **Inference** - public closed-binary replay package in `downloads/luxi-replay-demo/` plus November evidence; engine
   source private; own-workload testing through a private GPU-swap evaluation. The `luxiedge-serve` binaries are a
   historical API toy, not the current engine  
2. **LuxiQuant Risk State Engine** (Risk Runtime v0.4.2) - no public download or public prices; confidential evaluation  
3. **LuxiBook** - free public receipt and determinism demonstration (CSV BS/Black-76 + output-vector hash + Ed25519 `lxq2_` receipt)  
   Includes **LuxiSize**, the sizing calculator, in `luxirisk/` (not the LuxiQuant Risk State Engine)  
4. **Haplotastic, Orbital, Catastrophe, Edge, LuxiEnergy** - no public package here; related research proofs only  

## Link targets from docs

| Need | Path |
|------|------|
| Inference replay package | [`downloads/luxi-replay-demo/`](downloads/luxi-replay-demo/) |
| Book binaries | [`downloads/luxibook/`](downloads/luxibook/) |
| Serve binaries (historical) | [`downloads/`](downloads/) |
| LuxiSize (part of LuxiBook) | [`luxirisk/`](luxirisk/) |
| How to run | [`DEMOS.md`](DEMOS.md) |
| Measured tables | [`RESULTS.md`](RESULTS.md) |
| Evidence index | [`evidence/README.md`](evidence/README.md) |
