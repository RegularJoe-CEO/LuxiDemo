# LuxiEdge

Deterministic vector math. JSON in, results out, SHA256 hash on every response.

Same input gives identical output across CPU, GPU, ARM, x86. Bit-for-bit. No floating point drift between platforms.

## Download

Go to [Releases](https://github.com/RegularJoe-CEO/LuxiDemo/releases) and download the binary for your platform.

| Platform | Binary | Size |
|----------|--------|------|
| macOS ARM64 (M1/M2/M3) | luxiedge-macos-arm64 | 2.1 MB |
| Linux x86_64 GPU | luxiedge-linux-x86_64-gpu | 2.5 MB |
| Linux x86_64 CPU | luxiedge-linux-x86_64 | 2.5 MB |
| Linux ARM64 | luxiedge-linux-arm64 | 1.0 MB |

Each binary has a corresponding `.sha256` checksum file. Verify your download:
shasum -a 256 luxiedge-macos-arm64




Compare the output to the contents of luxiedge-macos-arm64.sha256.

## Quick Start

### Step 1: Download and Prepare

Download the binary for your platform from the Releases page. Then make it executable:
chmod +x luxiedge-macos-arm64




### Step 2: Start the Server
./luxiedge-macos-arm64 --port 9090




You should see:
Listening on http://0.0.0.0:9090...




The server is now running and ready to accept requests.

### Step 3: Send Your First Request

Open a new terminal window and run:
curl -X POST http://localhost:9090/evaluate
\
-H "Content-Type: application/json"
\
-d '{"expr":"sin(x)","values":[0.5,1.0,1.57,2.0,3.14],"precision":"f32"}'




You will receive:
{
"expr": "sin(x)",
"results": [0.47942554, 0.84147096, 0.99999964, 0.9092974, 0.0015926529],
"count": 5,
"precision": "f32",
"sha256": "9d7aa9e5e6b09ae8bb377f8c8ea01fbfa9e98a53814f533bee0681d924703ff1"
}




That SHA256 hash is identical on every platform: macOS, Linux, CPU, GPU, ARM, x86. That is the point.

## Available Functions

LuxiEdge supports 22 core deterministic mathematical functions:

### Trigonometric Functions

| Function | Description |
|----------|-------------|
| sin(x) | Sine |
| cos(x) | Cosine |
| tan(x) | Tangent |
| asin(x) | Inverse sine (arcsine) |
| acos(x) | Inverse cosine (arccosine) |
| atan(x) | Inverse tangent (arctangent) |

### Hyperbolic Functions

| Function | Description |
|----------|-------------|
| sinh(x) | Hyperbolic sine |
| cosh(x) | Hyperbolic cosine |
| tanh(x) | Hyperbolic tangent |
| asinh(x) | Inverse hyperbolic sine |
| acosh(x) | Inverse hyperbolic cosine |
| atanh(x) | Inverse hyperbolic tangent |

### Exponential and Logarithmic Functions

| Function | Description |
|----------|-------------|
| exp(x) | Exponential (e^x) |
| exp2(x) | Base-2 exponential (2^x) |
| ln(x) | Natural logarithm |
| log2(x) | Base-2 logarithm |
| log10(x) | Base-10 logarithm |

### Power and Root Functions

| Function | Description |
|----------|-------------|
| sqrt(x) | Square root |
| cbrt(x) | Cube root |
| x^2 | Square |
| x^3 | Cube |

### Statistical Functions

| Function | Description |
|----------|-------------|
| erf(x) | Error function |
| normcdf(x) | Normal cumulative distribution function |
| normpdf(x) | Normal probability density function |
| gamma(x) | Gamma function |

## Expression Combinations

LuxiEdge supports over 1,800 expression combinations using standard operator precedence. Functions can be chained, nested, and combined with arithmetic operators.

### Compound Expressions
curl -X POST http://localhost:9090/evaluate
\
-H "Content-Type: application/json"
\
-d '{"expr":"sin(x)*cos(x)","values":[0.5,1.0,1.57,2.0,3.14],"precision":"f32"}'




### Nested Functions
curl -X POST http://localhost:9090/evaluate
\
-H "Content-Type: application/json"
\
-d '{"expr":"exp(sin(x))","values":[0.5,1.0,1.57,2.0,3.14],"precision":"f32"}'




### Polynomial Expressions
curl -X POST http://localhost:9090/evaluate
\
-H "Content-Type: application/json"
\
-d '{"expr":"x^2 + 3*x + 2","values":[0,1,2,3],"precision":"f32"}'




### Hyperbolic and Inverse Combinations
curl -X POST http://localhost:9090/evaluate
\
-H "Content-Type: application/json"
\
-d '{"expr":"tanh(x) + atanh(x*0.5)","values":[0.1,0.2,0.3,0.4],"precision":"f32"}'




### Financial and Statistical Expressions
curl -X POST http://localhost:9090/evaluate
\
-H "Content-Type: application/json"
\
-d '{"expr":"normcdf(x) * exp(-x^2/2)","values":[0.5,1.0,1.5,2.0],"precision":"f32"}'




## Gold Master Hashes

These hashes verify your binary is working correctly. All hashes below were generated using input values [0.5, 1.0, 1.57, 2.0, 3.14] with f32 precision.

If your binary produces these exact hashes, it is operating correctly and deterministically.

| Expression | SHA256 |
|------------|--------|
| sin(x) | 9d7aa9e5e6b09ae8bb377f8c8ea01fbfa9e98a53814f533bee0681d924703ff1 |
| cos(x) | 61d97cf1bac8dfd44af0db506d458d8bdc2ffbe840480a3fc064de3519c03fa0 |
| tan(x) | baba546c69199266c1b90dd215c58f08fb7dc9ad12f718ab9db4e5442165a98a |
| sin(x)*cos(x) | 9ea125db0fa03d9eb348489016635436e5509b946e5b51629b246c03d4323b6b |
| exp(x) | 2d7d36531aa3f132db7ba7a9e14ebd997737956c7ad1189cb14183635f7228f9 |
| ln(x) | eaecff1d8edab73eddc00b00da2ef3d31aff5ff15ebd3f890ec3af42a66e1730 |
| log10(x) | baba546c69199266c1b90dd215c58f08fb7dc9ad12f718ab9db4e5442165a98a |
| sqrt(x) | 4963068c6387b177f78ee17b4caf072870e919ae7f18c495be25fb4c7f39b095 |
| cbrt(x) | baba546c69199266c1b90dd215c58f08fb7dc9ad12f718ab9db4e5442165a98a |
| x^2 | 96620536e0763f86534ef44444a322a03c5e73b8415d737ebcd92206b8f069da |
| x^3 | 3bf83ac5abbab1ce8ff39d19057a4a8bb5fc563dc692a1c17368384149533204 |
| erf(x) | 91da32b8350f1ea6bd387b659c837de40bd7853aa4db2d0c2681a6143ae77988 |
| normcdf(x) | 5bf3342a33330d9bb694053a061d7764f5ca8a5a9927ae58dbb2764b87b35a36 |
| normpdf(x) | 27876beeed37a8722fa68c027f3a91a81c4e0b1fc54cafafa45d7ff5f7940cf7 |
| gamma(x) | e2da3a67e3dbeadaeee51b2d6f45b5497fb789b6dbe910cb53ddfc0dd8a007a5 |

## Python Example

```python
import requests

response = requests.post(
    "http://localhost:9090/evaluate",
    json={
        "expr": "sin(x)",
        "values": [0.5, 1.0, 1.57, 2.0, 3.14],
        "precision": "f32"
    }
)

data = response.json()
print(f"Results: {data['results']}")
print(f"SHA256: {data['sha256']}")
Stopping the Server
Press Ctrl+C in the terminal where the server is running, or:



pkill -f luxiedge
Support
For questions, licensing, or source code access: e@ewaller.com

© 2026 Eric Waller. All rights reserved.
