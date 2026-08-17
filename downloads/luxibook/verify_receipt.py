#!/usr/bin/env python3
"""
verify_receipt.py — verify a LuxiBook Ed25519 receipt signature
without needing the luxi-book binary.

Usage:
    python3 verify_receipt.py path/to/receipt.json [path/to/receipt2.json ...]

Exits 0 if every listed receipt passes, 1 if any fail.

Requirements (install one):
    pip install cryptography      # preferred
    pip install PyNaCl            # alternative

Receipt format (luxiquant-receipt-v2)
  The "receipt" JSON field contains: "lxq2_<base64url>"
  Decoded bytes: 3-byte magic ("LXQ") + 1-byte version + 1-byte flags
                 + 32-byte Ed25519 public key + 64-byte Ed25519 signature
  The signature is over the "payload" field (UTF-8 encoded key=value lines).
"""

import base64
import json
import sys

# Byte offsets inside the decoded receipt blob
_HEADER_LEN = 5   # magic(3) + version(1) + flags(1)
_PUBKEY_LEN = 32
_SIG_LEN    = 64
_TOTAL_LEN  = _HEADER_LEN + _PUBKEY_LEN + _SIG_LEN  # 101


def _b64url_decode(s: str) -> bytes:
    """Decode base64url without padding."""
    pad = (-len(s)) % 4
    return base64.urlsafe_b64decode(s + "=" * pad)


def _verify_with_cryptography(pubkey_bytes: bytes, sig_bytes: bytes, msg_bytes: bytes) -> bool:
    from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PublicKey
    from cryptography.exceptions import InvalidSignature
    key = Ed25519PublicKey.from_public_bytes(pubkey_bytes)
    try:
        key.verify(sig_bytes, msg_bytes)
        return True
    except InvalidSignature:
        return False


def _verify_with_nacl(pubkey_bytes: bytes, sig_bytes: bytes, msg_bytes: bytes) -> bool:
    import nacl.signing
    import nacl.exceptions
    vk = nacl.signing.VerifyKey(pubkey_bytes)
    try:
        vk.verify(msg_bytes, sig_bytes)
        return True
    except nacl.exceptions.BadSignatureError:
        return False


def _verify(pubkey_bytes: bytes, sig_bytes: bytes, msg_bytes: bytes) -> bool:
    """Try cryptography first, then PyNaCl."""
    try:
        return _verify_with_cryptography(pubkey_bytes, sig_bytes, msg_bytes)
    except ImportError:
        pass
    try:
        return _verify_with_nacl(pubkey_bytes, sig_bytes, msg_bytes)
    except ImportError:
        print("ERROR: install 'cryptography' or 'PyNaCl':", file=sys.stderr)
        print("       pip install cryptography", file=sys.stderr)
        sys.exit(2)


def verify_receipt(path: str) -> bool:
    try:
        with open(path) as f:
            data = json.load(f)
    except (OSError, json.JSONDecodeError) as e:
        print(f"FAIL  {path}: cannot load JSON — {e}")
        return False

    scheme = data.get("receipt_scheme", "")
    if not scheme.startswith("luxiquant-receipt-v2"):
        print(f"FAIL  {path}: unrecognised scheme '{scheme}'")
        return False

    receipt_str: str = data.get("receipt", "")
    payload: str     = data.get("payload", "")
    fp: str          = data.get("signer_fp", "")

    if not receipt_str.startswith("lxq2_"):
        print(f"FAIL  {path}: receipt field does not start with 'lxq2_'")
        return False

    try:
        raw = _b64url_decode(receipt_str[len("lxq2_"):])
    except Exception as e:
        print(f"FAIL  {path}: cannot base64url-decode receipt — {e}")
        return False

    if len(raw) != _TOTAL_LEN:
        print(f"FAIL  {path}: receipt is {len(raw)} bytes, expected {_TOTAL_LEN}")
        return False

    pubkey_bytes = raw[_HEADER_LEN : _HEADER_LEN + _PUBKEY_LEN]
    sig_bytes    = raw[_HEADER_LEN + _PUBKEY_LEN : _TOTAL_LEN]
    msg_bytes    = payload.encode("utf-8")

    # Cross-check embedded pubkey against top-level field
    expected_pubkey = data.get("signer_pubkey", "")
    if expected_pubkey and pubkey_bytes.hex() != expected_pubkey:
        print(f"FAIL  {path}: pubkey in receipt does not match signer_pubkey field")
        return False

    ok = _verify(pubkey_bytes, sig_bytes, msg_bytes)

    if ok:
        print(f"PASS  {path}  (fp {fp}  pubkey {pubkey_bytes.hex()[:16]}…)")
    else:
        print(f"FAIL  {path}: Ed25519 signature mismatch  (fp {fp})")
    return ok


def main() -> None:
    if len(sys.argv) < 2 or sys.argv[1] in ("-h", "--help"):
        print(__doc__)
        sys.exit(0)

    paths = sys.argv[1:]
    results = [verify_receipt(p) for p in paths]
    passed = sum(results)
    failed = len(results) - passed

    if len(paths) > 1:
        print(f"\n{passed}/{len(paths)} receipts passed" + (f", {failed} failed" if failed else ""))

    sys.exit(0 if all(results) else 1)


if __name__ == "__main__":
    main()
