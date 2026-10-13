#!/bin/bash
# Verify the LuxiEdge GPU-sum evidence on any machine with python3 + numpy.
# Usage: bash verify.sh data_f32.bin   (data_f32.bin is the release asset)
set -e; D=${1:-data_f32.bin}
python3 - "$D" <<'PY'
import sys,hashlib,math,numpy as np
raw=open(sys.argv[1],'rb').read()
assert hashlib.sha256(raw).hexdigest()=='be485460c3f3f047961254b6ce4df34b6977ecb53f244dc4d32e68f64dfc6e29','input data mismatch'
print('PASS input sha256 be485460c3f3f047...')
x=np.frombuffer(raw,dtype='<f4')
ex=math.fsum(x.astype(np.float64).tolist())
s=0.0;c=0.0
for v in x.tolist():  # Kahan, float64 accumulator, fixed sequential order
    y=v-c;t=s+y;c=(t-s)-y;s=t
h=hashlib.sha256(np.float64(s).tobytes()).hexdigest()[:16]
print('exact (math.fsum)      ', repr(ex))
print('fixed-order Kahan      ', repr(s), 'hash', h)
print('stock float32 (L4)      499894321152.0  error vs exact', round(499894321152.0-ex,2))
print('PASS' if h=='b952a970e4ad4440' else 'FAIL','exact-sum hash, expected b952a970e4ad4440 (L4 GPU, x86, Mac arm64)')
PY
