#!/bin/bash
# clean-room CPU test of the final tarball in the sandbox (no private source on PATH, fresh HOME)
C=/home/user/scratch/clean2; cd $C; L=$C/log.txt; : > $L
say(){ echo "$(date -u +%H:%M:%S) $*" | tee -a $L; }
rm -rf home pkg; mkdir -p home pkg
cp /home/user/scratch/demo_build/dist/luxi-replay-demo-1.0.0-linux-x86_64.tar.gz* .
sha256sum -c luxi-replay-demo-1.0.0-linux-x86_64.tar.gz.sha256 >> $L 2>&1; say "tarball sha rc=$?"
tar xzf luxi-replay-demo-1.0.0-linux-x86_64.tar.gz -C pkg; cd pkg/luxi-replay-demo-1.0.0-linux-x86_64
sha256sum -c SHA256SUMS > $C/sums.txt 2>&1; say "SHA256SUMS rc=$? files=$(grep -c ': OK' $C/sums.txt)"
E="env -i HOME=$C/home PATH=/usr/bin:/bin LUXI_MODEL_DIR=/home/user/scratch/models/m_r1"
t0=$(date +%s); $E ./install.sh --cpu > $C/install.log 2>&1; say "install --cpu rc=$? $(( $(date +%s)-t0 )) s"
$E ./luxi inspect --device cpu --check-weights > $C/inspect.json 2>$C/inspect.err; say "inspect rc=$?"
$E ./luxi demo --device cpu --out $C/demo_cpu > $C/demo_cpu.log 2>&1; say "demo cpu rc=$? $(grep RESULT $C/demo_cpu.log)"
$E ./luxi demo --device cpu --out $C/demo_cpu2 > $C/demo_cpu2.log 2>&1; say "demo cpu #2 rc=$? $(grep RESULT $C/demo_cpu2.log)"
$E ./luxi demo --device gpu --out $C/demo_gpu > $C/demo_gpu.log 2>&1; say "demo gpu on no-GPU box rc=$? (expect 3)"
R=$(ls $C/demo_cpu/bundle_*.json | grep -v summary); K=$(cat .luxi_state/demo_signing_key.hex >/dev/null; $E ./luxi inspect --device cpu 2>/dev/null | python3 -c "import json,sys;print(json.load(sys.stdin).get('installation_public_key',''))")
say "install pub $K"
PK=$(python3 -c "import json;print(json.load(open('$R'))['signer_public_key'] if 'signer_public_key' in json.load(open('$R')) else '')")
$E ./luxi verify $R --trusted-key $(grep -o '"signer_public_key": "[0-9a-f]*"' $C/demo_cpu.log | head -1 | cut -d'"' -f4) > $C/verify_own.json 2>&1; say "verify own rc=$?"
$E ./luxi verify $R --trusted-key 0000000000000000000000000000000000000000000000000000000000000001 > $C/verify_wrong.json 2>&1; say "verify wrong key rc=$? (expect 1)"
$E ./luxi tamper-test $R --trusted-key $(grep -o '"signer_public_key": "[0-9a-f]*"' $C/demo_cpu.log | head -1 | cut -d'"' -f4) > $C/tamper_own.json 2>&1; say "tamper own rc=$?"
$E ./luxi compare $R $(ls $C/demo_cpu2/bundle_*.json | grep -v summary) > $C/cmp_run1_run2.json 2>&1; say "compare run1 run2 rc=$?"
$E ./luxi compare /home/user/scratch/demo_build/sandbox_cpu_record.json $R > $C/cmp_rc1sandbox_final.json 2>&1; say "compare rc1 sandbox vs final sandbox rc=$?"
$E ./luxi verify reference/nov09_h100_nvl_signed/full_suite/bundle_a4-math500-090.json --trusted-key reference/keys/luxi_benchmark_key_2026-11.pub > $C/verify_nov09.json 2>&1; say "verify nov09 ref rc=$?"
$E ./luxi tamper-test reference/nov09_h100_nvl_signed/full_suite/bundle_a4-math500-090.json --trusted-key reference/keys/luxi_benchmark_key_2026-11.pub > $C/tamper_nov09.json 2>&1; say "tamper nov09 rc=$?"
$E ./luxi full-suite --device cpu > $C/fullsuite_cpu.log 2>&1; say "full-suite cpu rc=$? (expect 2)"
$E ./luxi demo --device cpu --model /nonexistent > $C/missing.log 2>&1; say "missing model rc=$? (expect 4)"
cp $R $C/sandbox_cpu_record_final.json
say DONE
