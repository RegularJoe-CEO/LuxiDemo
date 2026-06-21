#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BUILD="$ROOT/build"
OUT="$ROOT/release-assets"
PLATFORM="${1:-macos-arm64}"

mkdir -p "$OUT"

export LUXI_BUILD_EPOCH="${LUXI_BUILD_EPOCH:-$(date +%s)}"
cd "$BUILD"

case "$PLATFORM" in
  macos-arm64)
    cargo build --release
    TARGET_DIR="$BUILD/target/release"
    ;;
  linux-x86_64)
    cargo build --release --target x86_64-unknown-linux-gnu
    TARGET_DIR="$BUILD/target/x86_64-unknown-linux-gnu/release"
    ;;
  linux-arm64)
    cargo build --release --target aarch64-unknown-linux-gnu
    TARGET_DIR="$BUILD/target/aarch64-unknown-linux-gnu/release"
    ;;
  windows-x86_64)
    cargo build --release --target x86_64-pc-windows-gnu
    TARGET_DIR="$BUILD/target/x86_64-pc-windows-gnu/release"
    ;;
  *)
    echo "Unknown platform: $PLATFORM"
    exit 1
    ;;
esac

BINS=(luxiedge-lite luxiedge-demo luxi-tools)
EXT=""
if [[ "$PLATFORM" == windows-x86_64 ]]; then
  EXT=".exe"
fi

for bin in "${BINS[@]}"; do
  src="$TARGET_DIR/${bin}${EXT}"
  dst="$OUT/${bin}-${PLATFORM}${EXT}"
  cp "$src" "$dst"
  if command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$dst" > "$OUT/${bin}-${PLATFORM}.sha256"
  else
    sha256sum "$dst" > "$OUT/${bin}-${PLATFORM}.sha256"
  fi
  echo "Packaged $dst"
done

# GPU variant is same CPU binary labeled for GPU hosts
if [[ "$PLATFORM" == linux-x86_64 ]]; then
  cp "$OUT/luxiedge-demo-linux-x86_64" "$OUT/luxiedge-demo-linux-x86_64-gpu"
  cp "$OUT/luxiedge-demo-linux-x86_64.sha256" "$OUT/luxiedge-demo-linux-x86_64-gpu.sha256"
  cp "$OUT/luxiedge-lite-linux-x86_64" "$OUT/luxiedge-lite-linux-x86_64-gpu"
  cp "$OUT/luxiedge-lite-linux-x86_64.sha256" "$OUT/luxiedge-lite-linux-x86_64-gpu.sha256"
fi

echo "Done. Assets in $OUT"