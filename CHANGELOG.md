# Changelog

## 2026-11-10: Inference replay demo, November evidence, corrections

New public download and evidence. The closed-binary package was tested from a clean install before these docs advertise it. No existing binary, checksum, release asset, tag or signed test vector changed.

| Area | Change |
|---|---|
| New download | [`downloads/luxi-replay-demo/`](downloads/luxi-replay-demo/): `luxi-replay-demo-1.0.0-linux-x86_64.tar.gz` (sha256 `b1f6cf0b…39c9`), `.sha256`, `LICENSE.txt`, `THIRD_PARTY_NOTICES.md`, `README.md` with exact install, run, verify and compare commands |
| New evidence | [`evidence/r1-distill-7b-gpu-swap-2026-11-09/`](evidence/r1-distill-7b-gpu-swap-2026-11-09/): the recovered November 9 nine-GPU record, the 17 signed H100 NVL records, public key, offline verifier, and a completeness statement. [`evidence/r1-replay-package-2026-11-10/`](evidence/r1-replay-package-2026-11-10/): every replay record, comparison and log from the package test on RTX 5090, A40 and three CPUs |
| README.md, LUXI_SYSTEM.md, REPO_BOUNDARY.md, DEMOS.md | Removed "only commercial product" and "inference is not an offer". Now: one core engine with seven commercial products (luxiedge.com), not equally validated, most without a public binary; a table of what is public per product. Risk Runtime v0.4.2 (the change in PR #12) |
| INFERENCE.md, TEST_SCOPE.md, evidence/README.md, RESULTS.md | New November 2026 sections and rows; September 2026 Qwen2-7B results labelled historical, with a note that they do not transfer to November v27 |
| DEMOS.md | New Demo 1 (inference replay). Demo 2 `luxiedge-serve` relabelled a historical API toy, not the current reasoning engine. v3.0 section marked historical (10-day-expiring demos). LuxiBook matrix line now the v0.2.1 five-GPU August 16 matrix (H200, H100, A100, RTX 5090, RTX 4090; CPU and CUDA; 20 signed receipts across two books). LuxiSize: repository and release v0.2 digests listed side by side. Haplotastic distinguished from the Dorado and PairHMM research studies |
| downloads/README.txt, demo/luxiedge-yc-demo/README.md, luxirisk/README.md | Same corrections: serve demo is a historical toy; LuxiBook receipt matrix; LuxiSize checksum pairing |
| REPO_BOUNDARY.md | Corrected: GitHub Pages is still deployed from `site/` by `.github/workflows/pages.yml` |
| site/index.html | Replaced the legacy page, which linked `releases/latest/download/...` (those resolve to the newest release, currently a data release, and returned 404 when checked on 2026-11-10), with a pointer page to luxiedge.com/demo, DEMOS.md and the pinned v3.0 tag |
| Contact | `eric@luxiedge.com`, matching luxiedge.com |

## 2026-10-28: Risk Runtime v0.4.2 version mention

The LuxiQuant Risk State Engine runs on Risk Runtime v0.4.2, matching luxiedge.com. Documentation only. No code, binaries, checksums, release assets, tags, signed test vectors or workflows changed. HISTORY.md keeps its dated 2026-10-12 note as written.

| Doc | Change |
|---|---|
| README.md, DEMOS.md, LUXI_SYSTEM.md, REPO_BOUNDARY.md | "Risk Runtime v0.4.0" now reads "Risk Runtime v0.4.2" |

## 2026-10-17: LuxiSize folded into LuxiBook

LuxiBook is now the one free download. It includes LuxiSize, the sizing calculator. The LuxiQuant Risk State Engine is the only product with "risk" in its name. Documentation only. No code, binaries, checksums, release assets, tags, signed test vectors or workflows changed.

| Doc | Change |
|---|---|
| README.md | Demos line and download section present LuxiBook with LuxiSize included |
| DEMOS.md | LuxiSize sits under Demo 0 (LuxiBook) as "Part of LuxiBook" |
| downloads/luxibook/README.md | New "Included: LuxiSize sizing calculator" section |
| REPO_BOUNDARY.md, LUXI_SYSTEM.md, docs/quant-finance.md, downloads/README.txt, scripts/README.md, luxirisk/*.md | Product name in prose is LuxiSize |

## 2026-10-15: Align LuxiDemo wording with luxiedge.com

The repo now uses the same product structure, names, and scoped claims as luxiedge.com. No code, binaries, hashes, or measurements changed.

| Topic | Old wording | New wording |
|---|---|---|
| README opening | "LuxiEdge is deterministic numerics: same bits on every chip, with receipts." | "LuxiEdge is deterministic numerics with receipts. Every result here is scoped to a named workload, environment, comparator, and evidence artifact; there is no universal determinism or accuracy guarantee." |
| Receipt definition | "A receipt is the SHA-256 of the bytes from the run that produced them." | "Output hashes identify computation bytes. Signed receipts bind a payload to a key. Reference checks test numerical agreement. None is a substitute for the others." |
| Receipt definition (docs/aerospace, autonomous-systems, robotics) | "A receipt is the SHA-256 of ..." | "The output hash is the SHA-256 of ..." (plus "a signed receipt binds a payload to a key" where relevant) |
| Product | "the paid institutional product of LuxiQuant ... proprietary and evaluation-only" | "LuxiEdge's only commercial product ... available only through a proprietary paid confidential evaluation under NDA, with no public runtime download or public prices." |
| Demos and evidence | (not stated) | "LuxiBook v0.2.1 and LuxiRisk v0.2 are free demos. Inference, numerical primitives, quantization, and the proofs below are evidence and research, not separate paid offers." |
| DEMOS.md LuxiBook row | "Luxi Book (Quant sale)" / "Partner books / design-partner path" | "LuxiBook (free demo)" / "Free demonstration only; not evidence for the Risk State Engine" |
| DEMOS.md inference row | "LuxiEdge" / "Controlled downloadable inference evaluation package" | "Inference evidence (not an offer)" / "Evidence only; inference is not a separate paid offer" |
| Name | "Luxi Book" | "LuxiBook" (HISTORY.md and evidence/ left as historical record) |
| Name | "LuxiPack" | "LuxiPak" |
| LuxiSDG maturity | "Early concept" | "Historical (preserved software-defined-generator work; not a currently available product)" |
| LuxiLoad maturity | "Early concept" | "Roadmap (not on luxiedge.com)" |
| LUXI_SYSTEM.md LuxiEdge row | "LuxiEdge / Execute / Measured" (as a layer) | "LuxiEdge is the company, not a layer." Row renamed "Inference evidence ... Measured evidence, not an offer" |
| LUXI_SYSTEM.md LuxiQuant row | "Working engine · public Luxi Book ... is the Quant try without NDA" | "Paid confidential evaluation under NDA · free LuxiBook v0.2.1 and LuxiRisk v0.2 demos; TestFort Dec 2025 evaluated a defined seven-function numerical-engine workload, not the current Risk State Engine" |
| README inference summary | Led with prefill: "Luxi FA3 32,190 tok/s and 0.02162 J/token" | Leads with generation: "against vLLM 0.25.1 batch-invariant mode, Luxi was 1.5 to 2.7 times faster with 16 to 50% less GPU energy per token; against vLLM 0.25.1 default, the same speed with 2 to 7% more GPU energy per token ..." Prefill rows follow. |
| README structure | Orbital catalogs, Kessler, genomics, attention-v2 mixed into Proof | Proof now matches the site's families (Oasis, GPU sums, SGP4, inference) plus the LuxiBook demo. The rest moved, unchanged, under "Additional evidence", labeled "not additional commercial products". |
| REPO_BOUNDARY.md product order | Started with "Luxi Book" | Starts with "LuxiQuant Risk State Engine ... the only commercial product" |
