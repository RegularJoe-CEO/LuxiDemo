#!/usr/bin/env python3
"""
verify_receipt.py — verify a LuxiBook Ed25519 receipt signature
without needing the luxi-book binary.

Usage:
    python3 verify_receipt.py [--expect-pubkey <hex>] [--expect-fp <hex>] path/to/receipt.json [path/to/receipt2.json ...]

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

# Set via --expect-pubkey / --expect-fp. Empty means "no identity check".
PIN_PUBKEY = ""
PIN_FP     = ""


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

    if not isinstance(data, dict):
        print(f"SKIP  {path}: not a receipt "
              f"(top-level JSON is {type(data).__name__}, not an object)")
        return None

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

    if PIN_PUBKEY:
        if pubkey_bytes.hex() != PIN_PUBKEY.lower():
            print(f"FAIL  {path}: signer key {pubkey_bytes.hex()[:16]}... "
                  f"is not the pinned key {PIN_PUBKEY.lower()[:16]}...")
            return False
    elif PIN_FP:
        if fp.lower() != PIN_FP.lower():
            print(f"FAIL  {path}: signer_fp {fp} is not the pinned fp {PIN_FP}")
            return False

    ok = _verify(pubkey_bytes, sig_bytes, msg_bytes)

    if ok:
        print(f"PASS  {path}  (fp {fp}  pubkey {pubkey_bytes.hex()[:16]}…)")
    else:
        print(f"FAIL  {path}: Ed25519 signature mismatch  (fp {fp})")
    return ok


def main() -> None:
    global PIN_PUBKEY, PIN_FP

    args = sys.argv[1:]
    if not args or args[0] in ("-h", "--help"):
        print(__doc__)
        sys.exit(0)

    paths = []
    i = 0
    while i < len(args):
        if args[i] == "--expect-pubkey":
            if i + 1 >= len(args):
                print("ERROR: --expect-pubkey requires a hex argument", file=sys.stderr)
                sys.exit(2)
            PIN_PUBKEY = args[i + 1]
            i += 2
        elif args[i] == "--expect-fp":
            if i + 1 >= len(args):
                print("ERROR: --expect-fp requires a hex argument", file=sys.stderr)
                sys.exit(2)
            PIN_FP = args[i + 1]
            i += 2
        else:
            paths.append(args[i])
            i += 1

    if not paths:
        print(__doc__)
        sys.exit(0)

    if not PIN_PUBKEY and not PIN_FP:
        print("WARNING: no --expect-pubkey/--expect-fp given. This checks that each\n"
              "         receipt is internally consistent, NOT that it came from a\n"
              "         LuxiEdge install. Pin the key to make it an identity check.")

    results = [verify_receipt(p) for p in paths]
    checked = [r for r in results if r is not None]
    skipped = len(results) - len(checked)
    passed  = sum(checked)
    failed  = len(checked) - passed

    if len(paths) > 1:
        summary = f"\n{passed}/{len(checked)} passed"
        if skipped:
            summary += f", {skipped} skipped"
        if failed:
            summary += f", {failed} failed"
        print(summary)

    sys.exit(0 if (failed == 0 and passed > 0) else 1)


if __name__ == "__main__":
    main()
