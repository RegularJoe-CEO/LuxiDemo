#!/usr/bin/env bash
set -u
W=/tmp/cr101f; R=$W/results; rm -rf $R; mkdir -p $R
exec > >(tee -a $R/test_log.txt) 2>&1
say() { echo "$(date -u +%H:%M:%S) $*"; }
E="env -i HOME=$W/home PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin TERM=dumb"
M=/home/user/scratch/models/m_r1
XM=/home/user/scratch/demo_build/LuxiDemo/evidence/r1-replay-package-2026-11-10/cross-machine
cd $W
echo "c2114aa7bd6c47822bdf94de2f7d77f1aa773a32dcc3268072cfb979c2e5c8b3  luxi-replay-demo-1.0.1-linux-x86_64.tar.gz" | sha256sum -c | tee $R/tarball_check.txt
tar xzf luxi-replay-demo-1.0.1-linux-x86_64.tar.gz && cd luxi-replay-demo-1.0.1-linux-x86_64
sha256sum -c SHA256SUMS > $R/sha256sums_check.txt; say "SHA256SUMS rc=$? ok=$(grep -c ': OK$' $R/sha256sums_check.txt)"
(nproc; python3 -c "import os;print(len(os.sched_getaffinity(0)))"; lscpu | grep -i 'model name') > $R/cpu.txt
t0=$(date +%s); $E ./install.sh --cpu > $R/install.log 2>&1; say "install rc=$? $(( $(date +%s) - t0 )) s"
$E ./luxi inspect --device cpu --model $M --check-weights > $R/inspect_cpu.json 2> $R/inspect_cpu.err; say "inspect cpu rc=$?"
# README flow verbatim (default luxi_out), model via LUXI_MODEL_DIR as the README allows
EM="$E LUXI_MODEL_DIR=$M"
$EM ./luxi demo --device cpu > $R/demo_cpu_1.log 2>&1; say "demo cpu #1 rc=$? | $(grep -A0 RESULT $R/demo_cpu_1.log)"
ls luxi_out > $R/luxi_out_listing.txt
PK=$(python3 -c "import json,glob;print(json.load(open(glob.glob('luxi_out/summary_*cpu*.json')[0]))['signer_public_key'])"); echo "$PK" > $R/install_public_key.txt
cp $XM/bundle_tiny-math500-090_gpu-NVIDIA_GeForce_RTX_5090.json luxi_out/   # a record from another machine, as a reader would
$E ./luxi verify luxi_out/bundle_tiny-math500-090_cpu-*.json --trusted-key $PK > $R/readme3_verify_own.json 2>&1; say "README 3 verify (pattern) rc=$?"
$E ./luxi verify reference/nov09_h100_nvl_signed/full_suite/bundle_a4-math500-090.json --trusted-key reference/keys/luxi_benchmark_key_2026-11.pub > $R/readme3_verify_nov09.json; say "README 3 verify nov09 rc=$?"
$E ./luxi verify reference/nov09_h100_nvl_signed/full_suite/bundle_a4-math500-090.json --trusted-key 0000000000000000000000000000000000000000000000000000000000000001 > $R/readme3_wrongkey.json 2>&1; say "README 3 wrong key rc=$? (expect 1)"
$E ./luxi tamper-test reference/nov09_h100_nvl_signed/full_suite/bundle_a4-math500-090.json --trusted-key reference/keys/luxi_benchmark_key_2026-11.pub > $R/readme3_tamper_nov09.json; say "README 3 tamper-test rc=$?"
$E ./luxi compare luxi_out/bundle_tiny-math500-090_gpu-*.json luxi_out/bundle_tiny-math500-090_cpu-*.json > $R/readme4_compare_5090_vs_cpu.json 2>&1; say "README 4 compare (pattern, RTX 5090 1.0.0 vs this CPU) rc=$?"
# new in the final 1.0.1: the shipped signed Nov 9 H100 NVL short-replay record
N9=reference/nov09_h100_nvl_signed/short_replay/bundle_tiny_h100.json
echo "a2d3cf40119916334126076c606bfe71ca269f27691b8e4e281bf77d4f2036df  $N9" | sha256sum -c > $R/nov09_short_record_sha.txt; say "Nov 9 short record sha256 rc=$?"
$E ./luxi verify $N9 --trusted-key 42bb6bccd2bc7d0570b44d5e068971707a704a26e8cb0294b9df12f23fdfe418 > $R/verify_nov09_short.json 2>&1; say "verify Nov 9 short record (benchmark key) rc=$?"
$E ./luxi compare $N9 luxi_out/bundle_tiny-math500-090_cpu-*.json > $R/cmp_nov09_short_vs_cpu1.json 2>&1; say "README compare vs Nov 9 record rc=$?"
for t in /home/user/scratch/nov09_raw/x/*/swap/out_*/bundle_tiny_cpu_*.json; do b=$(basename $t .json); $E ./luxi compare $t luxi_out/bundle_tiny-math500-090_cpu-*.json > $R/cmp_nov09_${b}__vs_cpu1.json 2>&1; say "compare Nov 9 $b vs this CPU rc=$?"; done
cp $N9 $R/n9.bak; python3 -c "
import json;p='$N9';d=json.load(open(p));d['calls'][0]['fps'][0][5]+=1;json.dump(d,open(p,'w'))"
$EM ./luxi demo --device cpu --out $R/demo_cpu_badref > $R/demo_cpu_badref.log 2>&1; say "demo with an altered Nov 9 reference rc=$? (expect 1): $(grep RESULT $R/demo_cpu_badref.log | cut -c1-110)"
cp $R/n9.bak $N9; rm $R/n9.bak; sha256sum -c SHA256SUMS > /dev/null 2>&1; say "SHA256SUMS after restoring the reference rc=$?"
$EM ./luxi demo --device cpu --out $R/demo_cpu_2 > $R/demo_cpu_2.log 2>&1; say "demo cpu #2 rc=$?"
C1=$(ls luxi_out/bundle_*cpu*.json); C2=$(ls $R/demo_cpu_2/bundle_*.json)
$E ./luxi compare $C1 $C2 > $R/cmp_cpu1_cpu2.json; say "compare cpu1 vs cpu2 rc=$?"
for g in $XM/bundle_tiny-math500-090_gpu-*.json $XM/bundle_tiny-math500-090_cpu-*.json /home/user/scratch/demo_build/sandbox_cpu_record.json; do
  case $g in *.summary.json) continue;; esac
  b=$(basename $g .json); $E ./luxi compare $g $C1 > $R/cmp_v100_${b}__vs_v101cpu.json; say "compare 1.0.0 $b vs 1.0.1 cpu rc=$?"; done
$E ./luxi tamper-test $C1 --trusted-key $PK > $R/tamper_own.json; say "tamper-test own rc=$?"
S100=$(ls $XM/*.summary.json 2>/dev/null | head -1); [ -z "$S100" ] && S100=$(ls /home/user/scratch/demo_build/LuxiDemo/evidence/r1-replay-package-2026-11-10/a40/*.summary.json | head -1)
$E ./luxi compare $S100 $C1 > $R/cmp_summaryfile.out 2>&1; say "compare given a 1.0.0 summary file rc=$? (expect 2): $(tail -1 $R/cmp_summaryfile.out)"
mkdir -p $R/dirA $R/dirB; cp luxi_out/bundle_*cpu*.json $R/dirA/; cp $S100 $R/dirA/; cp $C2 $R/dirB/$(basename $C1); $E ./luxi compare-dirs $R/dirA $R/dirB > $R/cmp_dirs.json 2>&1; say "compare-dirs with a stray 1.0.0 summary rc=$?"
$EM ./luxi demo --device gpu > $R/demo_gpu_nogpu.log 2>&1; say "demo gpu on no-GPU host rc=$? (expect 3): $(tail -1 $R/demo_gpu_nogpu.log | cut -c1-120)"
$EM ./luxi full-suite --device cpu > $R/full_cpu.log 2>&1; say "full-suite cpu rc=$? (expect 2)"
$E ./luxi demo --device cpu --model /nonexistent > $R/missing_model.log 2>&1; say "missing model rc=$? (expect 4)"
python3 - <<'PY' > $R/outputs_vs_v100.txt
import json,glob
a=json.load(open(glob.glob('/tmp/cr101f/luxi-replay-demo-1.0.1-linux-x86_64/luxi_out/bundle_*cpu*.json')[0]))
b=json.load(open('/home/user/scratch/demo_build/sandbox_cpu_record.json'))
print('tokens_equal', a['calls'][0]['outputs']==b['calls'][0]['outputs'], 'fps_equal', a['calls'][0]['fps']==b['calls'][0]['fps'])
print('build_equal', a['build']==b['build'], a['build'])
print('engine_sha_equal', a['engine_sha256']==b['engine_sha256'])
PY
cat $R/outputs_vs_v100.txt
say "DONE"
