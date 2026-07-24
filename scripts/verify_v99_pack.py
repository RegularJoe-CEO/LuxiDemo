#!/usr/bin/env python3
"""Recompute the public Version 99 headline from the retained per-run CSV."""

from __future__ import annotations

import csv
import statistics
from pathlib import Path


PACK = (
    Path(__file__).resolve().parent.parent
    / "evidence"
    / "h100-qwen2-7b-v99-matched-prefill-2026-07-23"
)


def median(rows: list[dict[str, str]], field: str) -> float:
    return statistics.median(float(row[field]) for row in rows)


with (PACK / "T5_PER_RUN.csv").open(newline="", encoding="utf-8") as handle:
    all_rows = list(csv.DictReader(handle))

groups = {
    arm: [row for row in all_rows if row["arm"] == arm]
    for arm in ("T1_LUXI", "T1_VLLM_DEFAULT", "T2_VLLM_BI")
}

if any(not rows for rows in groups.values()):
    raise SystemExit("missing one or more required evidence arms")

luxi_thr = median(groups["T1_LUXI"], "positions_per_s")
luxi_j = median(groups["T1_LUXI"], "J_per_position")
default_thr = median(groups["T1_VLLM_DEFAULT"], "positions_per_s")
default_j = median(groups["T1_VLLM_DEFAULT"], "J_per_position")
bi_thr = median(groups["T2_VLLM_BI"], "positions_per_s")
bi_j = median(groups["T2_VLLM_BI"], "J_per_position")

default_speed_ratio = luxi_thr / default_thr
default_energy_advantage = (1.0 - luxi_j / default_j) * 100.0
bi_speed_ratio = luxi_thr / bi_thr
bi_energy_advantage = (1.0 - luxi_j / bi_j) * 100.0
bi_thr_loss = (1.0 - bi_thr / default_thr) * 100.0
bi_j_increase = (bi_j / default_j - 1.0) * 100.0

for row in groups["T1_LUXI"]:
    if row["attn_backend"] != "flash" or row["fallback"] != "0":
        raise SystemExit("Luxi backend/fallback evidence check failed")

print("Version 99 public evidence verification")
print(f"LuxiEdge:            {luxi_thr:,.1f} pos/s  {luxi_j:.6f} J/pos")
print(f"vLLM default:        {default_thr:,.1f} pos/s  {default_j:.6f} J/pos")
print(f"vLLM batch-invariant:{bi_thr:,.1f} pos/s  {bi_j:.6f} J/pos")
print(
    f"Luxi/default: {default_speed_ratio * 100:.2f}% throughput, "
    f"{default_energy_advantage:.2f}% lower J/pos"
)
print(
    f"Luxi/batch-invariant: {bi_speed_ratio * 100:.2f}% throughput, "
    f"{bi_energy_advantage:.2f}% lower J/pos"
)
print(
    f"Batch-invariant cost: {bi_thr_loss:.2f}% throughput loss, "
    f"{bi_j_increase:.2f}% J/pos increase"
)
print("Flash backend/fallback: PASS")

assert abs(luxi_thr - 28374.6687) < 0.01
assert abs(luxi_j - 0.01871770568555622) < 1e-12
assert abs(default_speed_ratio - 0.8060) < 0.0001
assert abs(default_energy_advantage - 3.10) < 0.01
assert abs(bi_speed_ratio - 0.9178) < 0.0001
assert abs(bi_energy_advantage - 9.15) < 0.01

print("Headline arithmetic: PASS")

