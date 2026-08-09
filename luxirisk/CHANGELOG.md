# Changelog

## [0.2.0] — luxirisk-v0.2

### Fixed (blocking)

- **Receipt forgeability (Defect 1):** Replaced truncated public SHA-256 with
  **Ed25519 signatures** from a **per-install keypair**. Share form is branded
  `lxr1_…`. Third parties verify offline with the binary or
  `test-vectors/verify_receipts.py`.
- **Optional time binding:** `--stamp` (drand HTTPS, opt-in) or offline
  `--beacon VALUE`.
- **Trust surface:** CLI-only (removed `ui`). Public CI workflow with build
  provenance. Honest residual-risk table (unsigned OS binary + extractable
  install key).

### Distribution (honest)

- **v0.2 OS binaries are unsigned** (no Apple Developer ID / notarization, no
  Windows Authenticode). Expect Gatekeeper / SmartScreen friction on first run.
- Checksums + CI provenance published; paid code-signing deferred (scripts kept
  for a future optional release).

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
