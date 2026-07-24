# Public demo catalog

Luxi demos are distributed as **compiled evaluation binaries**. Evaluators can
run the tools, supply inputs, inspect outputs, and compare SHA-256 receipts
without receiving the private implementation source.

## Download and verify

Download the matching files from the
[v3.0 release](https://github.com/RegularJoe-CEO/LuxiDemo/releases/tag/v3.0):

- `luxiedge-demo-<platform>`: 30-operator numerical demo and REST service
- `luxiedge-lite-<platform>`: smaller numerical evaluation surface
- `luxi-tools-<platform>`: ATE, energy, orbital, and robotics demonstrations
- matching `.sha256` files

Example for Linux x86-64:

```bash
chmod +x luxiedge-demo-linux-x86_64 luxi-tools-linux-x86_64
sha256sum luxiedge-demo-linux-x86_64
sha256sum luxi-tools-linux-x86_64
```

Compare each printed digest with the corresponding downloaded `.sha256` file.
The checksum files retain their release-build path prefix, so compare the digest
rather than assuming `sha256sum -c` will resolve that path from every download
directory.

## Demo 1: numerical and quant validation

```bash
./luxiedge-demo-linux-x86_64 validate
./luxiedge-demo-linux-x86_64 list
```

The current demo lists 30 operators, including transcendental functions,
statistics, activation functions, `quant_chain`, GELU, SiLU, and RMSNorm.

The validation suite also exercises Welford mean/variance, online softmax,
LayerNorm, Waller attention, a WNSM transformer block, and model-configuration
presets.

## Demo 2: local REST evaluation

Start the compiled service:

```bash
./luxiedge-demo-linux-x86_64 --port 9090
```

From another terminal:

```bash
curl -X POST http://127.0.0.1:9090/evaluate \
  -H "Content-Type: application/json" \
  -d '{"expr":"sin(x)","values":[0.5,1.0,1.57,2.0,3.14],"precision":"f32"}'
```

On the tested Linux x86-64 v3.0 binary, three repeated requests produced the
same output receipt:

`27365fdff1b57d9a8077acdc40ca9d6183481a11dd5928e9f7b7e4782d2019a9`

That is a receipt for this binary, request, mode, and tested environment. It is
not a universal cross-version or cross-platform guarantee. This smoke confirms
execution and repeatability; numerical error bounds require a separate
operator-accuracy contract.

## Demo 3: ATE / Waller / WNSM

```bash
./luxi-tools-linux-x86_64 ate
```

The command reports NORMAL and WNSM receipts, maximum output difference,
payload bytes avoided, and an estimated energy field. Treat estimated energy as
illustrative unless a corresponding measured evidence pack is cited.

The tested Linux x86-64 build reported identical NORMAL/WNSM receipts and
`0.00e0` maximum output difference.

## Demo 4: energy-path illustration

```bash
./luxi-tools-linux-x86_64 energy
```

This command illustrates payload-avoidance accounting at several shapes. It is
not the Version 99 H100 measurement and is not facility-energy evidence.

## Demo 5: scientific solvers

```bash
./luxi-tools-linux-x86_64 orbital
```

The orbital demonstration emits receipts for Kepler, Vis-Viva, Hohmann,
drag-decay, J2, and Lambert-style calculations.

## Demo 6: edge and robotics math

```bash
./luxi-tools-linux-x86_64 robotics
```

The robotics demonstration emits receipts for inverse kinematics, Kalman
weighting, tire slip, motor torque, clothoid curvature, and lidar distance.

## Demo 7: current inference evidence

The full Version 99 Qwen2-7B engine is not distributed from this public demo
repository. Its current public demonstration is evidence verification:

```bash
python3 scripts/verify_v99_pack.py
```

The verifier recomputes the published throughput and GPU-board-energy ratios
from the retained per-run table. It exposes the evidence contract, not the
proprietary engine source.

## Demo coverage and next gates

| Luxi layer | Public demo today | Next validation step |
|---|---|---|
| LuxiQuant | Binary validation + REST + receipts | Publish platform-by-platform receipt matrix |
| LuxiEdge | Primitives + Version 99 evidence verifier | Controlled downloadable inference evaluation package |
| LuxiPack | None | Admission/placement trace versus a baseline |
| LuxiPhase | None | Synthetic load-shaping trace with SLO and stability checks |
| LuxiLoad | None | Controlled compute/power co-simulation or design engagement |
| LuxiSDG | None | Hardware-in-the-loop/site pilot with safety controls |

Concept illustrations are not labelled as working demos.

## Distribution

The public packages contain compiled evaluation binaries, documentation, and
test evidence. Proprietary implementation source and model weights are not
distributed.
