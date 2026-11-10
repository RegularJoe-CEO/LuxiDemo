#!/usr/bin/env bash
# Pod-side acceptance test of the final luxi-replay-demo 1.0.1 (c2114aa7...) from a clean folder with a clean HOME.
set -u
T=$1; W=/root/lxtest; R=$W/results_$T; rm -rf $R; mkdir -p $R
exec > >(tee -a $R/test_log.txt) 2>&1
say() { echo "$(date -u +%H:%M:%S) $*"; }
E="env -i HOME=/root/lxhome_$T PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin TERM=dumb"
N=luxi-replay-demo-1.0.1-linux-x86_64
cd $W
say "START $T"
nvidia-smi --query-gpu=name,compute_cap,memory.total,driver_version,power.limit --format=csv > $R/gpu.txt; cat $R/gpu.txt
lscpu > $R/lscpu.txt; (nproc; cat /sys/fs/cgroup/cpu.max 2>/dev/null; python3 -c "import os;print(len(os.sched_getaffinity(0)))") > $R/cpu_count.txt
free -g > $R/mem.txt; (cat /etc/os-release; ldd --version | head -1) > $R/os.txt
sha256sum -c $N.tar.gz.sha256 | tee $R/tarball_check.txt
rm -rf pkg /root/lxhome_$T && mkdir pkg && tar xzf $N.tar.gz -C pkg
cd pkg/$N
sha256sum -c SHA256SUMS > $R/sha256sums_check.txt; say "SHA256SUMS rc=$? ok=$(grep -c ': OK$' $R/sha256sums_check.txt)"
t0=$(date +%s); $E ./install.sh --gpu > $R/install.log 2>&1; say "install rc=$? $(( $(date +%s) - t0 )) s"; tail -4 $R/install.log
./.uv/uv pip freeze --python .venv/bin/python > $R/pip_freeze_uv.txt 2>/dev/null || true
t0=$(date +%s); $E ./luxi fetch-model > $R/fetch.log 2>&1; say "fetch-model rc=$? $(( $(date +%s) - t0 )) s"
$E ./luxi inspect --device gpu --check-weights > $R/inspect_gpu.json 2> $R/inspect_gpu.err; say "inspect gpu rc=$?"
# README flow verbatim: default output folder luxi_out
$E ./luxi demo --device gpu > $R/demo_gpu_1.log 2>&1; say "demo gpu #1 rc=$? $(grep RESULT $R/demo_gpu_1.log)"
MEMG=$(free -g | awk '/Mem:/{print $2}')
if [ "$MEMG" -ge 30 ]; then $E ./luxi demo --device cpu > $R/demo_cpu.log 2>&1; say "demo cpu rc=$? $(grep RESULT $R/demo_cpu.log)"; else say "demo cpu SKIPPED: only $MEMG GB RAM"; fi
cp -r luxi_out $R/luxi_out
PK=$(python3 -c "import json,glob;print(json.load(open(sorted(glob.glob('luxi_out/summary_*gpu*.json'))[0]))['signer_public_key'])"); echo "$PK" > $R/install_public_key.txt
$E ./luxi verify luxi_out/bundle_tiny-math500-090_gpu-*.json --trusted-key $PK > $R/readme_verify_own.json 2>&1; say "README verify (pattern) rc=$?"
$E ./luxi compare luxi_out/bundle_tiny-math500-090_gpu-*.json luxi_out/bundle_tiny-math500-090_cpu-*.json > $R/readme_compare_gpu_vs_cpu.json 2>&1; say "README compare gpu vs cpu (pattern) rc=$?"
N9=reference/nov09_h100_nvl_signed/short_replay/bundle_tiny_h100.json
$E ./luxi verify $N9 --trusted-key 42bb6bccd2bc7d0570b44d5e068971707a704a26e8cb0294b9df12f23fdfe418 > $R/verify_nov09_short.json 2>&1; say "verify Nov 9 short record (benchmark key) rc=$?"
$E ./luxi compare $N9 luxi_out/bundle_tiny-math500-090_gpu-*.json > $R/cmp_nov09_short_vs_gpu1.json 2>&1; say "README compare vs Nov 9 record (gpu) rc=$?"
$E ./luxi compare $N9 luxi_out/bundle_tiny-math500-090_cpu-*.json > $R/cmp_nov09_short_vs_podcpu.json 2>&1; say "README compare vs Nov 9 record (pod cpu) rc=$?"
for t in $W/nov09_tiny_cpu/*.json; do b=$(basename $t .json); $E ./luxi compare $t luxi_out/bundle_tiny-math500-090_gpu-*.json > $R/cmp_nov09_${b}__vs_gpu1.json 2>&1; say "compare Nov 9 $b vs this GPU rc=$?"; done
$E ./luxi demo --device gpu --out $R/demo_gpu_2 > $R/demo_gpu_2.log 2>&1; say "demo gpu #2 rc=$? $(grep RESULT $R/demo_gpu_2.log)"
G1=$(ls luxi_out/bundle_*gpu*.json); G2=$(ls $R/demo_gpu_2/bundle_*.json)
$E ./luxi compare $G1 $G2 > $R/cmp_gpu1_gpu2.json; say "compare gpu1 vs gpu2 rc=$?"
for f in $W/ref_v100/*.json; do b=$(basename $f .json); $E ./luxi compare $f $G1 > $R/cmp_v100_${b}__vs_v101_gpu.json; say "compare 1.0.0 $b vs 1.0.1 gpu rc=$?"; done
$E ./luxi verify $G1 --trusted-key reference/keys/luxi_benchmark_key_2026-11.pub > $R/verify_wrongkey.json; say "verify wrong key rc=$? (expect 1)"
$E ./luxi tamper-test $G1 --trusted-key $PK > $R/tamper_own.json; say "tamper-test rc=$? (expect 0 = all edits rejected)"
say "FULL SUITE start"
t0=$(date +%s); $E ./luxi full-suite --device gpu --out $R/full > $R/full.log 2>&1; say "full-suite rc=$? $(( $(date +%s) - t0 )) s $(grep RESULT $R/full.log)"
$E ./luxi compare-dirs $W/nov09_a6000_p17 $R/full/records > $R/cmp_full_vs_nov09_A6000_p17.json 2>&1; say "full suite vs Nov 9 RTX A6000 records (per-machine build) rc=$?"
cd $W && tar czf $W/lxtest_$T.tgz results_$T && say "DONE $T"
