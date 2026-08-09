# Public demo catalog

Luxi demos are distributed as **compiled evaluation binaries**. Evaluators can
run the tools, supply inputs, inspect outputs, and compare SHA-256 receipts
without receiving the private implementation source.

## Demo −1: LuxiRisk v0.2 — offline trader risk calculator (signed receipts)

**Closed binary only — no engine source.** Three calculations for leveraged
crypto / retail traders, each with an **Ed25519-signed** `lxr1_…` receipt
(per-install identity) for posting in Discord, Telegram, and X.

| Calculation | Example |
|-------------|---------|
| Liquidation price (isolated approx.) | `liq --side long --entry 65000 --leverage 10` → **58825** |
| Position size from risk % | `size --balance 10000 --risk 1 --entry 65000 --stop 63000` → **0.05** base |
| Max $ loss at stop | `risk --size 0.5 --entry 65000 --stop 63000` → **1000** |

| Platform | Binary in repo |
|----------|----------------|
| macOS ARM64 | [`luxirisk/dist/luxirisk-macos-arm64`](luxirisk/dist/luxirisk-macos-arm64) |
| Linux x86_64 | [`luxirisk/dist/luxirisk-linux-x86_64`](luxirisk/dist/luxirisk-linux-x86_64) |
| Windows x86_64 | [`luxirisk/dist/luxirisk-windows-x86_64.exe`](luxirisk/dist/luxirisk-windows-x86_64.exe) |

```bash
chmod +x luxirisk/dist/luxirisk-macos-arm64
shasum -a 256 -c luxirisk/dist/luxirisk-macos-arm64.sha256
./luxirisk/dist/luxirisk-macos-arm64 liq --side long --entry 65000 --leverage 10
# receipt starts with lxr1_  ·  fingerprint is per-install
./luxirisk/dist/luxirisk-macos-arm64 verify 'lxr1_…' liq \
  --side long --entry 65000 --leverage 10 --expect-liq 58825
pip install cryptography && python3 luxirisk/test-vectors/verify_receipts.py
```

- Home: [`luxirisk/`](luxirisk/) · formulas: [`luxirisk/FORMULAS.md`](luxirisk/FORMULAS.md)
- Test vectors: [`luxirisk/test-vectors/`](luxirisk/test-vectors/)
- Signing / CI: [`luxirisk/SIGNING.md`](luxirisk/SIGNING.md)
- Planned release tag: `luxirisk-v0.2` (published only after Developer ID + Authenticode signing)
- CI: [luxirisk-release workflow](https://github.com/RegularJoe-CEO/LuxiDemo/actions/workflows/luxirisk-release.yml) · offline by default · optional `--stamp` (drand) · CLI only

## Demo 0: version-100 commercial serve + GTM scoreboard (current)

**Binary only — no engine source.** OpenAI-shaped HTTP API with locked H100 thr/J/det on `GET /v1/gtm`.

| Platform | Binary in repo |
|----------|----------------|
| macOS ARM64 | [`site/downloads/luxiedge-serve-macos-arm64`](site/downloads/luxiedge-serve-macos-arm64) |
| Linux x86_64 | [`site/downloads/luxiedge-serve-linux-x86_64`](site/downloads/luxiedge-serve-linux-x86_64) |

```bash
chmod +x luxiedge-serve-macos-arm64
./luxiedge-serve-macos-arm64 --bind 127.0.0.1:8787
curl -s http://127.0.0.1:8787/v1/gtm | python3 -m json.tool
curl -s -X POST http://127.0.0.1:8787/v1/audit -d '{}'
```

- Page: [`site/demo.html`](site/demo.html) · package: [`demo/luxiedge-yc-demo/`](demo/luxiedge-yc-demo/)
- Evidence: [`evidence/version-100-h100-gtm/`](evidence/version-100-h100-gtm/)
- Local generate path is a toy engine for instant API demos; thr/J numbers on `/v1/gtm` are the **measured H100 multi-run lock**.

## Download and verify (numerical v3.0)

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
| LuxiRisk | Offline risk CLI + Ed25519 `lxr1_` receipts + verify + public vectors | v0.2 on main + CI provenance; **signed GitHub release waits on Apple/Windows certs** |
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
