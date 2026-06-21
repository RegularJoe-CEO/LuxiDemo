# LuxiEdge PyTorch integration

## Receipt on tensors (no Rust build required)

```python
import torch
from luxiedge_ops import sha256_receipt_hex, assert_audit_receipt, LuxiEdgeReceiptHook

logits = model(input_ids)  # your model
receipt = sha256_receipt_hex(logits)
print("receipt", receipt)

hook = LuxiEdgeReceiptHook(notes="desk-alpha-v1")
handle = model.register_forward_hook(hook)
```

## AUDIT lane

Set `LUXI_RECEIPT_AUDIT=1` in the environment when calling into LuxiEdge Rust binaries
(`cuda_verify`, decoder forward) so GPU paths match CPU gold receipts.

Python-side `assert_audit_receipt(logits, expected_hex, reference=cpu_logits)` enforces
`0.00e0` max diff when comparing torch tensors.

## Rust FFI (optional cdylib)

When `crate-type = ["cdylib"]` is enabled on the Rust crate, call:

- `luxiedge_sha256_f32(ptr, len, out32)`
- `luxiedge_max_abs_diff_f32(a, b, len)`

See `src/ffi/mod.rs`.