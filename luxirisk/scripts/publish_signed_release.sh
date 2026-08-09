#!/usr/bin/env bash
# One-shot: after Apple + Windows signing secrets are set on LuxiDemo,
# build signed binaries via CI and publish luxirisk-v0.2.
#
# Prerequisites (gh secrets on RegularJoe-CEO/LuxiDemo):
#   LUXIRISK_ENGINE_REPO, LUXIRISK_ENGINE_DEPLOY_KEY
#   APPLE_CERTIFICATE_P12_BASE64, APPLE_CERTIFICATE_PASSWORD,
#   APPLE_SIGNING_IDENTITY, APPLE_API_KEY_ID, APPLE_API_ISSUER, APPLE_API_KEY_P8
#   WINDOWS_CERT_P12_BASE64, WINDOWS_CERT_PASSWORD
#
# Usage:
#   ./luxirisk/scripts/publish_signed_release.sh
#   ENGINE_REF=v0.2.0 TAG=luxirisk-v0.2 ./luxirisk/scripts/publish_signed_release.sh
set -euo pipefail

REPO="${REPO:-RegularJoe-CEO/LuxiDemo}"
ENGINE_REF="${ENGINE_REF:-v0.2.0}"
TAG="${TAG:-luxirisk-v0.2}"

need_secrets=(
  LUXIRISK_ENGINE_REPO
  LUXIRISK_ENGINE_DEPLOY_KEY
  APPLE_CERTIFICATE_P12_BASE64
  APPLE_CERTIFICATE_PASSWORD
  APPLE_SIGNING_IDENTITY
  APPLE_API_KEY_ID
  APPLE_API_ISSUER
  APPLE_API_KEY_P8
  WINDOWS_CERT_P12_BASE64
  WINDOWS_CERT_PASSWORD
)

echo "==> Checking required secrets exist on $REPO (names only)"
have=$(gh secret list -R "$REPO" | awk '{print $1}')
missing=0
for s in "${need_secrets[@]}"; do
  if ! grep -qx "$s" <<<"$have"; then
    echo "MISSING secret: $s"
    missing=1
  else
    echo "OK      secret: $s"
  fi
done
if [[ "$missing" -ne 0 ]]; then
  echo ""
  echo "Set secrets per luxirisk/SIGNING.md, then re-run this script."
  exit 1
fi

echo "==> Dispatch luxirisk-release (publish_release=true)"
gh workflow run luxirisk-release.yml -R "$REPO" \
  -f "engine_ref=$ENGINE_REF" \
  -f "tag=$TAG" \
  -f publish_release=true

echo "==> Waiting for latest workflow run…"
sleep 5
RUN_ID=$(gh run list -R "$REPO" --workflow=luxirisk-release.yml --limit 1 --json databaseId -q '.[0].databaseId')
echo "run id: $RUN_ID"
gh run watch "$RUN_ID" -R "$REPO" --exit-status

echo "==> Release view"
gh release view "$TAG" -R "$REPO"

echo ""
echo "Post-sign checks (download assets first):"
echo "  shasum -a 256 -c luxirisk-macos-arm64.sha256"
echo "  codesign --verify --verbose=2 luxirisk-macos-arm64"
echo "  spctl --assess --type execute --verbose=4 luxirisk-macos-arm64"
echo "  # Windows: Get-AuthenticodeSignature .\\luxirisk-windows-x86_64.exe"
echo "  gh attestation verify luxirisk-macos-arm64 -R $REPO"
echo "DONE"
