"""
LuxiEdge PyTorch integration — receipt contract and AUDIT hooks.

Requires a built `libattention_transformer` (cdylib) or subprocess to `receipt_backtest` example.
This module works standalone for receipt verification on torch tensors (pure Python SHA-256).
"""

from __future__ import annotations

import hashlib
import struct
from typing import Optional, Union

import torch

LUXI_RECEIPT_AUDIT_ENV = "LUXI_RECEIPT_AUDIT"


def sha256_receipt_f32(tensor: torch.Tensor) -> bytes:
    """
    Cryptographic receipt over contiguous f32 tensor (fixed row-major flatten order).
    """
    if tensor.dtype != torch.float32:
        tensor = tensor.detach().cpu().float()
    flat = tensor.detach().cpu().contiguous().view(-1).tolist()
    payload = bytearray()
    for v in flat:
        bits = struct.unpack("<I", struct.pack("<f", float(v)))[0]
        payload.extend(struct.pack("<I", bits))
    return hashlib.sha256(payload).digest()


def sha256_receipt_hex(tensor: torch.Tensor) -> str:
    return sha256_receipt_f32(tensor).hex()


def assert_audit_receipt(
    tensor: torch.Tensor,
    expected_hex: str,
    *,
    max_abs_diff: float = 0.0,
    reference: Optional[torch.Tensor] = None,
) -> None:
    """Gate: receipt match and optional 0.00e0 diff vs reference (AUDIT lane)."""
    got = sha256_receipt_hex(tensor)
    if got != expected_hex.lower().replace(" ", ""):
        raise AssertionError(f"receipt mismatch: got {got} expected {expected_hex}")
    if reference is not None:
        diff = (tensor.float() - reference.float()).abs().max().item()
        if diff > max_abs_diff:
            raise AssertionError(f"max_abs_diff {diff} > {max_abs_diff}")


def audit_mode_enabled() -> bool:
    import os

    return os.environ.get(LUXI_RECEIPT_AUDIT_ENV, "") == "1"


class LuxiEdgeReceiptHook:
    """Register forward hook to log receipts on module outputs."""

    def __init__(self, notes: str = ""):
        self.notes = notes
        self.last_receipt: Optional[str] = None

    def __call__(self, module, inputs, output):
        if isinstance(output, torch.Tensor):
            self.last_receipt = sha256_receipt_hex(output)
        return output