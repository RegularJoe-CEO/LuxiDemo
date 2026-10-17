# Changelog

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
