# LuxiEdge

Bit-exact deterministic vector math engine (y=f(x)) with SHA-256 verified outputs.

## Download

### Linux x86_64

    curl -LO https://github.com/RegularJoe-CEO/LuxiDemo/releases/download/v2.0.0/luxiedge-linux-x86_64.zip
    unzip luxiedge-linux-x86_64.zip
    chmod +x luxiedge-linux-x86_64

### Linux ARM64

    curl -LO https://github.com/RegularJoe-CEO/LuxiDemo/releases/download/v2.0.0/luxiedge-linux-arm64.zip
    unzip luxiedge-linux-arm64.zip
    chmod +x luxiedge-linux-arm64

## Usage

Start the server:

    ./luxiedge-linux-x86_64

Evaluate an expression:

    curl -X POST http://localhost:9090/evaluate \
      -H "Content-Type: application/json" \
      -d '{"expr":"sin(x)*cos(x)","values":[0.5,1.0,1.57,2.0,3.14],"precision":"f32"}'

## Features

- Bit-exact deterministic vector math (y=f(x))
- SHA-256 verified outputs
- 30-day evaluation period
- 15 math functions, 6 binary operators

## Contact

e@ewaller.com

