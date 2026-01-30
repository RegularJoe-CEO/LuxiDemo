# LuxiEdge: World's Fastest Deterministic JSON Math Engine

Bit-exact deterministic vector math (y=f(x)) with SHA256 audit trails.

## Gold Master Release v1.0 (Port 10000)

### Download Binaries
| Platform | Binary | Size | 
|----------|--------|------|
| Linux x86_64 (CPU) | `luxiedge-linux-x86_64` | **1.9 MB** |
| Linux x86_64 (GPU) | `luxiedge-linux-x86_64-gpu` | **2.5 MB** |
| macOS ARM64 (M1/M2) | `luxiedge-macos-arm64` | **2.1 MB** |
| macOS x86_64 (Intel) | `luxiedge-macos-x86_64` | **1.8 MB** |
| Linux ARM64 (Edge) | `luxiedge-edge-arm64` | **1.6 MB** |

### Quick Start
```bash
chmod +x luxiedge-macos-arm64
./luxiedge-macos-arm64 &
curl http://localhost:10000/health
