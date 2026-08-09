# Changelog

## [0.2.0] — luxirisk-v0.2

### Fixed (blocking)

- **Receipt forgeability (Defect 1):** Replaced truncated public SHA-256 with
  **Ed25519 signatures** from a **per-install keypair**. Share form is branded
  `lxr1_…`. Third parties verify offline with the binary or
  `test-vectors/verify_receipts.py`.
- **Optional time binding:** `--stamp` (drand HTTPS, opt-in) or offline
  `--beacon VALUE`.
- **Runnable / trust surface (Defect 2):** CLI-only (removed `ui`). Public CI
  workflow with build provenance attestation. Signing/notarization scripts for
  Developer ID + Authenticode when credentials are available. Honest residual
  risk table in README.

### Changed

- Receipt scheme: `luxirisk-receipt-v2`
- Version string includes LuxiEdge attribution
- Test vectors re-signed with fixed documentation identity

### Removed

- Localhost `ui` subcommand
- v0.1 short-hash receipts (no longer produced)

## [0.1.0] — luxirisk-v0.1

Initial release: liq / size / risk, SHA-256 short receipts, optional UI,
macOS/Linux/Windows binaries.
