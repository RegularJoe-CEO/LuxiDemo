# LuxiRisk v0.1 — exact formulas

These are the **only** calculations performed by LuxiRisk v0.1. Anyone can
re-implement them and compare outputs (and receipts) against the published
[test vectors](test-vectors/).

All arithmetic uses a **deterministic fixed-point decimal** path (scale
\(10^{12}\), half-away-from-zero on intermediate multiply/divide). Results are
bit-identical across macOS ARM64, Linux x86_64, and Windows x86_64.

---

## 1. Liquidation price (isolated margin approximation)

**Inputs**

| Name | Symbol | Notes |
|------|--------|--------|
| Side | long / short | Required |
| Entry price | \(E\) | \(E > 0\) |
| Leverage | \(L\) | \(L > 0\) |
| Maintenance margin rate | \(m\) | Default \(m = 0.005\) (0.5%). Override with `--mmr` or `--mmr-pct`. |

**Formulas**

\[
\begin{aligned}
\text{Long:}  &\quad P_{\text{liq}} = E \times \bigl(1 - \tfrac{1}{L} + m\bigr) \\
\text{Short:} &\quad P_{\text{liq}} = E \times \bigl(1 + \tfrac{1}{L} - m\bigr)
\end{aligned}
\]

**Example** — long, \(E = 65000\), \(L = 10\), \(m = 0.005\):

\[
P_{\text{liq}} = 65000 \times (1 - 0.1 + 0.005) = 65000 \times 0.905 = 58825
\]

**Example** — short, same inputs:

\[
P_{\text{liq}} = 65000 \times (1 + 0.1 - 0.005) = 65000 \times 1.095 = 71175
\]

> This is a simplified isolated-margin approximation used by many educational
> and retail tools. Live exchange liquidation engines may include fees, funding,
> and tiered maintenance margin schedules. LuxiRisk does **not** call exchanges.

---

## 2. Position size from risk %

**Inputs**

| Name | Symbol | Notes |
|------|--------|--------|
| Account balance | \(B\) | \(B > 0\) |
| Risk percentage | \(r\) | e.g. `1` means 1% of account |
| Entry price | \(E\) | \(E > 0\) |
| Stop price **or** stop distance % | \(S\) or \(d\%\) | Exactly one |

**Formulas**

\[
\begin{aligned}
\text{Risk amount} &= B \times \frac{r}{100} \\
\text{Stop distance} &=
\begin{cases}
|E - S| & \text{if absolute stop price } S \\
E \times \dfrac{d}{100} & \text{if stop distance \% } d
\end{cases} \\
\text{Position size (base)} &= \frac{\text{Risk amount}}{\text{Stop distance}} \\
\text{Notional} &= \text{Position size} \times E
\end{aligned}
\]

**Example** — \(B = 10000\), \(r = 1\), \(E = 65000\), \(S = 63000\):

\[
\begin{aligned}
\text{Risk amount} &= 100 \\
\text{Stop distance} &= 2000 \\
\text{Position size} &= 0.05 \\
\text{Notional} &= 3250
\end{aligned}
\]

Beginner tip: risk only **1% of your account** per trade.

---

## 3. Max dollar loss / risk at stop

**Inputs:** position size \(Q\) (base units), entry \(E\), stop \(S\).

\[
\text{Max \$ loss} = Q \times |E - S|
\]

**Consistency check:** for the position-size example above,
\(Q = 0.05\), \(E = 65000\), \(S = 63000\) ⇒ max loss \(= 100\), which equals
the risk amount from formula 2.

**Example** — \(Q = 0.5\), \(E = 65000\), \(S = 63000\):

\[
\text{Max \$ loss} = 0.5 \times 2000 = 1000
\]

---

## Receipt algorithm

Every successful calculation emits a **short receipt** (first 12 hex characters
of SHA-256) and can emit the **full** SHA-256 with `--full-receipt`.

### Canonical payload

UTF-8 text, one field per line, keys sorted lexicographically **after** a fixed
header line:

```text
luxirisk-receipt-v1
<entry_key>=<canonical_value>
...
```

Rules:

1. First line is exactly `luxirisk-receipt-v1`.
2. Remaining lines are `key=value`, sorted by `key` (byte order).
3. Decimal values use **canonical form**: no scientific notation, no leading
   `+`, no trailing fractional zeros (`1.2500` → `1.25`), no thousands
   separators, `.` as the decimal separator.
4. Enum-like strings are lowercase (`long`, `short`, `liq`, `size`, `risk`).
5. Payload always ends with a trailing newline after the last line.
6. Hash: `SHA-256` over the exact UTF-8 bytes of that payload.
7. Short receipt: first **12** hex characters of the lowercase hex digest.

### Common fields

| Field | Present on |
|-------|------------|
| `tool` | always (`luxirisk`) |
| `version` | always (`0.1.0`) |
| `op` | `liq` \| `size` \| `risk` |

Plus operation-specific inputs and outputs (see [test-vectors/](test-vectors/)).

### Independent verification

1. Build the canonical payload from known inputs/outputs using the rules above.
2. Compute SHA-256 (any standard tool, e.g. `sha256sum`, Python `hashlib`).
3. Compare the full digest (or first 12 chars) to the receipt from the binary.

Example (Python):

```python
import hashlib
payload = """luxirisk-receipt-v1
entry=65000
leverage=10
liq_price=58825
mmr=0.005
op=liq
side=long
tool=luxirisk
version=0.1.0
"""
print(hashlib.sha256(payload.encode()).hexdigest())
# → a896b6f35054bd83e82693308c6ad591699cd11ea8bcc69725d3d5fde80eeddc
# short → a896b6f35054
```

---

## Non-goals (v0.1)

Not computed: funding rates, full Kelly criterion, Monte Carlo scenarios,
prop-firm consistency modes, exchange API lookups.
