# Binary signing & provenance (v0.2)

## v0.2 release policy: **unsigned by design**

**LuxiRisk v0.2 ships without Apple Developer ID signing, notarization, or
Windows Authenticode.**

| Topic | v0.2 status |
|-------|-------------|
| macOS Developer ID + notarize + staple | **Not applied** |
| Windows Authenticode | **Not applied** |
| SHA-256 checksums | **Published** (`.sha256` + `SHA256SUMS`) |
| GitHub Actions build provenance | **Available** on CI runs that built the assets |

Users will hit first-run OS friction (Gatekeeper / SmartScreen). That is
documented in the [README trust model](README.md#trust-model-honest) and in the
GitHub Release notes. It is the residual risk of a free closed binary without
paid certificates.

**Do not claim v0.2 binaries are signed or notarized.**

---

## Private engine & CI (used for builds)

| Item | Value |
|------|--------|
| Repo | `RegularJoe-CEO/luxirisk-engine` (**private**) |
| Engine tag | `v0.2.0` |
| Public demo repo | `RegularJoe-CEO/LuxiDemo` |
| Workflow | [`.github/workflows/luxirisk-release.yml`](../.github/workflows/luxirisk-release.yml) |

### Secrets used for v0.2 CI (engine checkout only)

| Secret | Purpose |
|--------|---------|
| `LUXIRISK_ENGINE_REPO` | `RegularJoe-CEO/luxirisk-engine` |
| `LUXIRISK_ENGINE_DEPLOY_KEY` | Read-only SSH deploy key for the private engine |

Optional alternative: `LUXIRISK_ENGINE_TOKEN` (PAT with `contents:read`).

---

## Optional / future: paid code-signing (not used in v0.2)

Scripts and secret names below are **kept for a future release**. They are
**not** required for v0.2 and were **not** applied to the published assets.

### Apple (Developer ID Application + notarization)

| Secret | Purpose |
|--------|---------|
| `APPLE_CERTIFICATE_P12_BASE64` | Base64 of `.p12` Developer ID Application cert |
| `APPLE_CERTIFICATE_PASSWORD` | P12 password |
| `APPLE_SIGNING_IDENTITY` | e.g. `Developer ID Application: Your Name (TEAMID)` |
| `APPLE_API_KEY_ID` | App Store Connect API key id |
| `APPLE_API_ISSUER` | App Store Connect issuer UUID |
| `APPLE_API_KEY_P8` | Full `.p8` private key PEM text |

Engine scripts (private repo): `scripts/macos_sign_notarize.sh`

Post-sign checks (when used):

```bash
codesign --verify --verbose=2 luxirisk-macos-arm64
xcrun stapler validate luxirisk-macos-arm64
spctl --assess --type execute --verbose=4 luxirisk-macos-arm64
```

### Windows (Authenticode — OV min, EV preferred)

| Secret | Purpose |
|--------|---------|
| `WINDOWS_CERT_P12_BASE64` | Base64 of code-signing `.p12` / `.pfx` |
| `WINDOWS_CERT_PASSWORD` | P12 password |

Engine script: `scripts/windows_sign.sh`

### Future publish helper

[`scripts/publish_signed_release.sh`](scripts/publish_signed_release.sh) checks
for the optional signing secrets and can dispatch CI with `publish_release=true`.
**Not used for the honest unsigned v0.2 ship.**

---

## Provenance without code-signing

For v0.2, trust the download chain as:

1. Download binary + `.sha256` / `SHA256SUMS` from the GitHub Release  
2. Verify the digest matches  
3. Optionally inspect GitHub Actions build provenance for the CI run that produced the artifact  

Receipt trust (`lxr1_…`) is independent of OS code-signing: it is Ed25519 over
the calculation payload with a **per-install** key.
