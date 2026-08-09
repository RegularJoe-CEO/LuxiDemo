#!/usr/bin/env python3
"""Offline verification of LuxiRisk v0.2 Ed25519-signed receipts.

No network. Requires either:
  - cryptography  (pip install cryptography), or
  - PyNaCl        (pip install pynacl)

Verifies:
  1. Receipt blob structure (lxr1_ prefix)
  2. Ed25519 signature over the canonical payload
  3. Payload pubkey/fp match the blob
  4. Published test-vector fingerprints when present

Usage:
  python3 verify_receipts.py
  python3 verify_receipts.py --receipt lxr1_… --payload-file claim.txt
"""
from __future__ import annotations

import argparse
import base64
import hashlib
import json
import sys
from pathlib import Path


PREFIX = "lxr1_"
MAGIC = b"LXR"
BLOB_VERSION = 2
SCHEME = "luxirisk-receipt-v2"


def b64url_decode(s: str) -> bytes:
    pad = "=" * (-len(s) % 4)
    return base64.urlsafe_b64decode(s + pad)


def fingerprint_hex(pubkey: bytes) -> str:
    return hashlib.sha256(pubkey).digest()[:8].hex()


def parse_receipt(receipt: str) -> dict:
    receipt = receipt.strip()
    if not receipt.startswith(PREFIX):
        raise ValueError(f"receipt must start with {PREFIX}")
    blob = b64url_decode(receipt[len(PREFIX) :])
    if len(blob) != 101:
        raise ValueError(f"invalid blob length {len(blob)} (expected 101)")
    if blob[0:3] != MAGIC:
        raise ValueError("invalid magic")
    if blob[3] != BLOB_VERSION:
        raise ValueError(f"unsupported version {blob[3]}")
    flags = blob[4]
    pubkey = blob[5:37]
    sig = blob[37:101]
    return {
        "flags": flags,
        "pubkey": pubkey,
        "signature": sig,
        "fingerprint": fingerprint_hex(pubkey),
        "pubkey_hex": pubkey.hex(),
        "receipt": receipt,
    }


def verify_ed25519(pubkey: bytes, message: bytes, signature: bytes) -> None:
    try:
        from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PublicKey
        from cryptography.exceptions import InvalidSignature

        key = Ed25519PublicKey.from_public_bytes(pubkey)
        try:
            key.verify(signature, message)
        except InvalidSignature as e:
            raise ValueError("signature verification FAILED") from e
        return
    except ImportError:
        pass

    try:
        from nacl.signing import VerifyKey
        from nacl.exceptions import BadSignatureError

        vk = VerifyKey(pubkey)
        try:
            vk.verify(message, signature)
        except BadSignatureError as e:
            raise ValueError("signature verification FAILED") from e
        return
    except ImportError:
        pass

    raise SystemExit(
        "Need Ed25519 verify support. Install one of:\n"
        "  pip install cryptography\n"
        "  pip install pynacl"
    )


def verify_pair(receipt: str, payload: str) -> dict:
    if not payload.endswith("\n"):
        payload = payload + "\n"
    parsed = parse_receipt(receipt)
    lines = payload.splitlines()
    if not lines or lines[0] != SCHEME:
        raise ValueError(f"payload must start with {SCHEME}")
    verify_ed25519(parsed["pubkey"], payload.encode("utf-8"), parsed["signature"])

    fields = {}
    for line in lines[1:]:
        if not line or "=" not in line:
            continue
        k, v = line.split("=", 1)
        fields[k] = v
    if fields.get("pubkey") and fields["pubkey"] != parsed["pubkey_hex"]:
        raise ValueError("payload pubkey mismatch")
    if fields.get("fp") and fields["fp"] != parsed["fingerprint"]:
        raise ValueError("payload fp mismatch")
    if parsed["flags"] & 0x01:
        for k in ("beacon_source", "beacon_round", "beacon_value"):
            if k not in fields:
                raise ValueError(f"beacon flag set but missing {k}")
    return {
        "ok": True,
        "fingerprint": parsed["fingerprint"],
        "pubkey": parsed["pubkey_hex"],
        "payload_sha256": hashlib.sha256(payload.encode("utf-8")).hexdigest(),
        "op": fields.get("op"),
        "beacon": fields.get("beacon_value"),
    }


def main() -> int:
    ap = argparse.ArgumentParser(description="Verify LuxiRisk v0.2 lxr1_ receipts offline")
    ap.add_argument("--receipt", help="Single lxr1_ receipt string")
    ap.add_argument("--payload-file", help="Canonical payload file for --receipt")
    ap.add_argument(
        "--vectors",
        default=str(Path(__file__).resolve().parent / "vectors.json"),
        help="Path to vectors.json (default: alongside this script)",
    )
    args = ap.parse_args()

    if args.receipt:
        if not args.payload_file:
            print("error: --payload-file required with --receipt", file=sys.stderr)
            return 2
        payload = Path(args.payload_file).read_text(encoding="utf-8")
        try:
            info = verify_pair(args.receipt, payload)
        except Exception as e:
            print(f"FAIL: {e}")
            return 1
        print("VERIFIED ✓")
        for k, v in info.items():
            if k != "ok":
                print(f"  {k}: {v}")
        return 0

    data = json.loads(Path(args.vectors).read_text(encoding="utf-8"))
    failed = 0
    for v in data["vectors"]:
        try:
            info = verify_pair(v["receipt"], v["canonical_payload"])
            if v.get("fingerprint") and info["fingerprint"] != v["fingerprint"]:
                raise ValueError("vector fingerprint mismatch")
            if v.get("pubkey") and info["pubkey"] != v["pubkey"]:
                raise ValueError("vector pubkey mismatch")
            status = "PASS"
        except Exception as e:
            status = "FAIL"
            failed += 1
            info = {"error": str(e)}
        print(f"  [{status}] {v['id']}")
        if status == "FAIL":
            print(f"         {info.get('error')}")
        else:
            print(f"         fp {info['fingerprint']}  receipt {v['receipt'][:28]}…")
    print()
    if failed:
        print(f"{failed} vector(s) failed")
        return 1
    print(f"All {len(data['vectors'])} signed receipt vectors verified.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
