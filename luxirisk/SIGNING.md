# Binary signing, CI secrets & provenance (v0.2)

## Private engine

| Item | Value |
|------|--------|
| Repo | `RegularJoe-CEO/luxirisk-engine` (**private**) |
| Engine tag | `v0.2.0` |
| Public demo repo | `RegularJoe-CEO/LuxiDemo` |
| Workflow | [`.github/workflows/luxirisk-release.yml`](../.github/workflows/luxirisk-release.yml) |

## Secrets on LuxiDemo (exact names)

### Required for CI engine checkout

| Secret | Purpose |
|--------|---------|
| `LUXIRISK_ENGINE_REPO` | `RegularJoe-CEO/luxirisk-engine` |
| `LUXIRISK_ENGINE_DEPLOY_KEY` | Read-only SSH **private** key (PEM) for a deploy key on the engine repo |

**Optional HTTPS alternative** (if not using deploy key):

| Secret | Purpose |
|--------|---------|
| `LUXIRISK_ENGINE_TOKEN` | PAT with `contents:read` on `luxirisk-engine` |

### Required for Gatekeeper / SmartScreen-clean release

**Apple (Developer ID Application + notarization)**

| Secret | Purpose |
|--------|---------|
| `APPLE_CERTIFICATE_P12_BASE64` | Base64 of `.p12` Developer ID Application cert |
| `APPLE_CERTIFICATE_PASSWORD` | P12 password |
| `APPLE_SIGNING_IDENTITY` | e.g. `Developer ID Application: Your Name (TEAMID)` |
| `APPLE_API_KEY_ID` | App Store Connect API key id |
| `APPLE_API_ISSUER` | App Store Connect issuer UUID |
| `APPLE_API_KEY_P8` | Full `.p8` private key PEM text |

**Windows (Authenticode — OV minimum, EV preferred)**

| Secret | Purpose |
|--------|---------|
| `WINDOWS_CERT_P12_BASE64` | Base64 of code-signing `.p12` / `.pfx` |
| `WINDOWS_CERT_PASSWORD` | P12 password |

### Set secrets with `gh`

```bash
# Engine (already usable once deploy key exists)
gh secret set LUXIRISK_ENGINE_REPO -R RegularJoe-CEO/LuxiDemo -b 'RegularJoe-CEO/luxirisk-engine'
gh secret set LUXIRISK_ENGINE_DEPLOY_KEY -R RegularJoe-CEO/LuxiDemo < deploy_key_private

# Apple (when certs issued)
base64 -i DeveloperID.p12 | gh secret set APPLE_CERTIFICATE_P12_BASE64 -R RegularJoe-CEO/LuxiDemo
gh secret set APPLE_CERTIFICATE_PASSWORD -R RegularJoe-CEO/LuxiDemo -b '…'
gh secret set APPLE_SIGNING_IDENTITY -R RegularJoe-CEO/LuxiDemo -b 'Developer ID Application: …'
gh secret set APPLE_API_KEY_ID -R RegularJoe-CEO/LuxiDemo -b '…'
gh secret set APPLE_API_ISSUER -R RegularJoe-CEO/LuxiDemo -b '…'
gh secret set APPLE_API_KEY_P8 -R RegularJoe-CEO/LuxiDemo < AuthKey_XXX.p8

# Windows
base64 -i codesign.pfx | gh secret set WINDOWS_CERT_P12_BASE64 -R RegularJoe-CEO/LuxiDemo
gh secret set WINDOWS_CERT_PASSWORD -R RegularJoe-CEO/LuxiDemo -b '…'
```

## Local signing scripts (private engine)

```bash
# macOS Developer ID + notarytool + staple
export APPLE_CERTIFICATE_P12_BASE64=…
export APPLE_CERTIFICATE_PASSWORD=…
export APPLE_SIGNING_IDENTITY='Developer ID Application: …'
export APPLE_API_KEY_ID=… APPLE_API_ISSUER=… APPLE_API_KEY_P8="$(cat AuthKey.p8)"
./scripts/macos_sign_notarize.sh dist/luxirisk-macos-arm64

# Verify macOS
codesign --verify --verbose=2 dist/luxirisk-macos-arm64
xcrun stapler validate dist/luxirisk-macos-arm64
spctl --assess --type execute --verbose=4 dist/luxirisk-macos-arm64

# Windows Authenticode
export WINDOWS_CERT_P12_BASE64=… WINDOWS_CERT_PASSWORD=…
./scripts/windows_sign.sh dist/luxirisk-windows-x86_64.exe
# Verify (Windows): Get-AuthenticodeSignature .\luxirisk-windows-x86_64.exe
# or: osslsigncode verify -in luxirisk-windows-x86_64.exe
```

## CI usage

```bash
# Build + attest only (no GitHub Release) — safe without signing certs
gh workflow run luxirisk-release.yml -R RegularJoe-CEO/LuxiDemo \
  -f engine_ref=v0.2.0 -f tag=luxirisk-v0.2 -f publish_release=false

# After ALL signing secrets are set — one-shot publish (checks secret names, runs CI, waits):
./luxirisk/scripts/publish_signed_release.sh
# equivalent:
# gh workflow run luxirisk-release.yml -R RegularJoe-CEO/LuxiDemo \
#   -f engine_ref=v0.2.0 -f tag=luxirisk-v0.2 -f publish_release=true
```

Provenance: each platform job runs `actions/attest-build-provenance` on the binary.
Inspect under the Actions run → **Attestations**, or:

```bash
gh attestation verify luxirisk-macos-arm64 -R RegularJoe-CEO/LuxiDemo
```

## Release policy (hard)

Do **not** publish tag/release `luxirisk-v0.2` until:

1. macOS binary is Developer ID signed, notarized, and stapled (`spctl --assess` clean)
2. Windows binary is Authenticode-signed (SmartScreen-trusted path for OV/EV)
3. SHA256SUMS + provenance attestations exist

Until Apple + Windows cert secrets are present, CI still builds **unsigned** hash-verified binaries with provenance. Those are **not** a public release.

## Current environment (as of v0.2 engineering)

- Private engine repo + deploy key + `LUXIRISK_ENGINE_REPO` secret: **wired**
- Developer ID / Authenticode material on the build Mac: **not present** (`security find-identity -p codesigning` → 0 identities)
- Therefore: public tree + CI may land; **signed release waits on cert handoff**
