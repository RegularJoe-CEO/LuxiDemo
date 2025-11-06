<!-- SPDX-FileCopyrightText: 2025 Eric Waller -->
<!-- SPDX-License-Identifier: LicenseRef-Luxi-Business-1.0 -->

# Luxi Edge — Docker Quick Start

## Pull and Run

```bash
# Pull the latest image
docker pull ghcr.io/regularjoe-ceo/luxi-edge:latest

# Run the container
docker run -d -p 8080:8080 --name luxi-edge ghcr.io/regularjoe-ceo/luxi-edge:latest

# Or run with automatic restart
docker run -d -p 8080:8080 --name luxi-edge --restart unless-stopped \
  ghcr.io/regularjoe-ceo/luxi-edge:latest
```

## Verify It's Running

```bash
# Health check
curl -s http://127.0.0.1:8080/health | python3 -m json.tool

# Evaluate an expression
curl -s http://127.0.0.1:8080/evaluate \
  -H 'Content-Type: application/json' \
  -d '{"expr":"x*x + 2*x + 1","x":[0,1,2,3]}' | python3 -m json.tool
```

## Using Docker Compose

Create `docker-compose.yml`:
```yaml
version: '3.8'
services:
  luxi-edge:
    image: ghcr.io/regularjoe-ceo/luxi-edge:latest
    ports:
      - "8080:8080"
    restart: unless-stopped
```

Then run:
```bash
docker-compose up -d
```

## Container Details

- **Image**: `ghcr.io/regularjoe-ceo/luxi-edge:latest`
- **Port**: 8080
- **Precision**: default f64 (double precision)
- **Size**: ~50MB (multi-stage build)
- **Platforms**: linux/amd64, linux/arm64

## Environment Variables

Currently none required. The service runs with sensible defaults.

## Troubleshooting

**Port already in use:**
```bash
# Use a different port
docker run -p 8081:8080 ghcr.io/regularjoe-ceo/luxi-edge:latest
```

**View logs:**
```bash
docker logs luxi-edge
docker logs -f luxi-edge  # Follow logs
```

**Stop and remove:**
```bash
docker stop luxi-edge
docker rm luxi-edge
```

## Building Locally

If you want to build your own image:
```bash
git clone https://github.com/RegularJoe-CEO/LuxiEdge
cd LuxiEdge
docker build -t luxi-edge:local .
docker run -p 8080:8080 luxi-edge:local
```
