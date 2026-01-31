# LuxiEdge

Deterministic vector math. JSON in, results out, SHA256 hash on every response.

Same input gives identical output across CPU, GPU, ARM, x86. Bit-for-bit. No floating point drift between platforms.

## Download

Go to [Releases](../../releases) and download the binary for your platform.

| Platform | Binary | Size |
|----------|--------|------|
| Linux x86_64 CPU | luxiedge-linux-x86_64 | 1.9 MB |
| Linux x86_64 GPU | luxiedge-linux-x86_64-gpu | 2.5 MB |
| macOS ARM64 | luxiedge-macos-arm64 | 2.1 MB |
| macOS Intel | luxiedge-macos-x86_64 | 1.8 MB |
| Linux ARM64 | luxiedge-edge-arm64 | 1.6 MB |

## Run

```bash
chmod +x luxiedge-macos-arm64
./luxiedge-macos-arm64
```

Starts on port 10000.

## Your First Request

```bash
curl -X POST http://localhost:10000/evaluate \
  -H "Content-Type: application/json" \
  -d '{"expr":"sin(x)","values":[0.5,1.0,1.57],"precision":"f32"}'
```

You get back:

```json
{
  "expr": "sin(x)",
  "results": [0.479425, 0.841470, 0.999999],
  "count": 3,
  "precision": "f32",
  "sha256": "6aa5d30189d51808836ac5760daa9781b79889b46b0086614bafe6a4dab86713"
}
```

That SHA256 hash is identical on every platform. That's the point.

## Chaining Expressions

You can combine operations in a single expression. The engine parses left to right with standard operator precedence.

**Compound trig:**
```bash
curl -X POST http://localhost:10000/evaluate \
  -H "Content-Type: application/json" \
  -d '{"expr":"sin(x)*cos(x)","values":[0.5,1.0,2.0],"precision":"f32"}'
```

**Polynomial:**
```bash
curl -X POST http://localhost:10000/evaluate \
  -H "Content-Type: application/json" \
  -d '{"expr":"x^2 + 3*x + 2","values":[0,1,2,3],"precision":"f32"}'
```

**Nested functions:**
```bash
curl -X POST http://localhost:10000/evaluate \
  -H "Content-Type: application/json" \
  -d '{"expr":"sqrt(x^2 + sin(x))","values":[0.5,1.0,2.0],"precision":"f32"}'
```

**Probability density:**
```bash
curl -X POST http://localhost:10000/evaluate \
  -H "Content-Type: application/json" \
  -d '{"expr":"normpdf(x)*exp(-x)","values":[0,0.5,1.0],"precision":"f32"}'
```

## Available Functions

sin, cos, exp, ln, sqrt, x^2, x^3, erf, normcdf, normpdf, gamma

All 15 produce identical SHA256 hashes across all platforms.

## Python Example

```python
import requests

r = requests.post(
    "http://localhost:10000/evaluate",
    json={
        "expr": "gamma(x) * exp(-x)",
        "values": [0.5, 1.0, 2.5],
        "precision": "f32"
    }
)
print(r.json()["sha256"])  # Same hash everywhere
```

## Gold Master Hashes

These hashes verify your binary is working correctly. Run the test script or compare manually.

| Expression | SHA256 |
|------------|--------|
| sin(x)*cos(x) | e4f0bae37c4150f642e7ecb0983e72e35731fce3085457ec2459eff8a19f338d |
| sin(x) | 6aa5d30189d51808836ac5760daa9781b79889b46b0086614bafe6a4dab86713 |
| cos(x) | b7c1f8996c8ac9ccd78152c9059de701e0ad9b92cd3b34a6bc3aad6e29118920 |
| exp(x) | 850398cb9aa7804013779dfbe9f3e3af8626dfccd8861d7b04bc407b5ca85425 |
| ln(x) | 5c9f7a02bf2b9495332e4a8c55d56d4cf402c4d542aaeca621bd4962dbbedd65 |
| sqrt(x) | 2d1b204c60f1e52f4ff35d720a164aa4c7a088aa903f8c4ca1c9ef00609b8033 |
| x^2 | 65a7d42468848d2103de850716f0fbcf99ec512d929899fdb42fc15323f1a882 |
| x^3 | 4c176c9025305f8a581e212d2e51ba3ee7092ff7cb54f86e7faa99609e56f83c |
| erf(x) | a3c9114b35331843254ee92e00697f5a8f79fc8f854b2f2e186c41b0c3dd6729 |
| normcdf(x) | 466726456b025149e1f51aeb037fd1b1d74ef35ad50caa3ce4600afad62f518e |
| normpdf(x) | ca921dc59661a1372923f7851d8d3afdeb0725070769a4986aad41174405a57a |
| gamma(x) | 64af395a9d8c995404f56254fae96d1dc4a5c678262875fa00e89ef1a1fd963e |

## Support

e@ewaller.com
## Gold Checksums
| Platform | Binary | Size | SHA256 |
|----------|--------|------|--------|
| linux-x86_64-gpu | luxiedge-linux-x86_64-gpu | 1.8M | ed45907f3a3cdf1c03ef114fd231d1da84148b6484dadd49b836ffade591e249 |
| macos-arm64 | luxiedge-macos-arm64 | 2.1M | $(shasum -a 256 binaries/luxiedge-macos-arm64 | cut -d' ' -f1) |
