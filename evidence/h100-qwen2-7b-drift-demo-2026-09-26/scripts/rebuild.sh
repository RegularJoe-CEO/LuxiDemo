#!/bin/bash
# Rebuild the large files of this folder from what is committed, then check every file against MANIFEST.sha256.
# usage (from this folder): bash scripts/rebuild.sh TOKENIZER_DIR
#   TOKENIZER_DIR holds tokenizer.json from Qwen/Qwen2-7B-Instruct (sha256 f7c9b2dba4a296b1aa76c16a34b8225c0c118978400d4bb66bff0902d702f5b8).
# Needs python3 with tokenizers (0.23.2 was used) and matplotlib (3.11.2 was used; other versions can give a PNG with different bytes).
set -e
TOK=${1:?usage: bash scripts/rebuild.sh TOKENIZER_DIR}
PY=${PY:-python3}
cd "$(dirname "$0")/.."
T=$(mktemp -d)
# 1. raw per-engine records: join the line-boundary parts
for e in luxi_c8 vllm_normal vllm_det sglang_normal sglang_det; do
  cat raw/results_$e.part*.jsonl > raw/results_$e.jsonl
done
# 2. data/filler_pool.json: build_data.py is deterministic (fixed seed); run it on a copy so data/prompts.json stays as committed
mkdir -p $T/data && cp data/build_data.py data/declaration_of_independence_gutenberg1.txt $T/data/
$PY $T/data/build_data.py "$TOK" > /dev/null && cp $T/data/filler_pool.json data/
# 3. drift_results_*.json, drift_chart.png, summary.md, summary.json, examples.md from the raw records
mkdir -p $T/raw $T/out && cp raw/results_luxi_c8.jsonl raw/results_luxi_c8_p20_separate_process.jsonl raw/results_vllm_normal.jsonl \
  raw/results_vllm_det.jsonl raw/results_sglang_normal.jsonl raw/results_sglang_det.jsonl raw/luxi_sanity.json $T/raw/
$PY scripts/analyze.py $T/raw $T/out --tokenizer "$TOK" > /dev/null
cp $T/out/drift_results_*.json $T/out/drift_chart.png .
for f in summary.md summary.json examples.md; do cmp $T/out/$f $f && echo "reproduced $f"; done
# 4. per_prompt.csv from the drift results
$PY scripts/per_prompt.py $T/out $T/per_prompt.csv && cmp $T/per_prompt.csv per_prompt.csv && echo "reproduced per_prompt.csv"
rm -rf $T
# 5. every file, committed and rebuilt
sha256sum -c --quiet MANIFEST.sha256 && echo "MANIFEST check passed"
