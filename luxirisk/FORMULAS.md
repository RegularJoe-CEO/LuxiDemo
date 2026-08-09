# LuxiRisk v0.2 — exact formulas & receipt scheme

These are the **only** calculations performed by LuxiRisk v0.2. Anyone can
re-implement them and compare outputs against the published
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

> Simplified isolated-margin approximation. Live exchange engines may include
> fees, funding, and tiered MMR. LuxiRisk does **not** call exchanges.

---

## 2. Position size from risk %

\[
\begin{aligned}
\text{Risk amount} &= B \times \frac{r}{100} \\
\text{Stop distance} &=
\begin{cases}
|E - S| & \text{absolute stop } S \\
E \times \dfrac{d}{100} & \text{stop distance \% } d
\end{cases} \\
\text{Position size (base)} &= \frac{\text{Risk amount}}{\text{Stop distance}} \\
\text{Notional} &= \text{Position size} \times E
\end{aligned}
\]

**Example** — \(B = 10000\), \(r = 1\), \(E = 65000\), \(S = 63000\):

risk amount \(= 100\), size \(= 0.05\), notional \(= 3250\).

---

## 3. Max dollar loss / risk at stop

\[
\text{Max \$ loss} = Q \times |E - S|
\]

Consistent with formula 2: size \(0.05\) → loss \(100\).

---

## Receipt scheme v2 (Ed25519, non-forgeable without the private key)

v0.1 used a truncated public SHA-256. **Anyone could mint a valid-looking
receipt without the binary.** v0.2 replaces that with an **Ed25519 signature**
over a canonical payload, produced by a **per-install private key**.

### Share form (branded, greppable)

```text
lxr1_<base64url(blob)>
```

| Field | Size | Content |
|-------|-----:|---------|
| magic | 3 | `LXR` |
| version | 1 | `2` |
| flags | 1 | bit0 = beacon present in payload |
| pubkey | 32 | Ed25519 public key (this install) |
| signature | 64 | Ed25519 signature over canonical payload |

Total blob = **101 bytes**. Prefix `lxr1_` makes receipts greppable in Discord / X.

### Canonical signed payload

UTF-8, line-oriented:

```text
luxirisk-receipt-v2
<entry_key>=<canonical_value>
...
```

Rules:

1. First line is exactly `luxirisk-receipt-v2`.
2. Remaining lines are `key=value`, sorted by key (byte order).
3. Decimal values use canonical fixed-point form (same as v0.1).
4. Always includes: `tool=luxirisk`, `version=0.2.0`, `pubkey=<64 hex>`, `fp=<16 hex>`.
5. Payload ends with a trailing newline.
6. Signature = Ed25519.Sign(private_key, payload_utf8_bytes).

### Per-install identity

On first successful calculation (or `luxirisk id`):

1. Generate a random Ed25519 keypair.
2. Store the secret key under the platform config dir (mode `0600` on Unix):
   - macOS: `~/Library/Application Support/LuxiRisk/`
   - Linux: `~/.config/luxirisk/` (or `$XDG_CONFIG_HOME/luxirisk/`)
   - Windows: `%APPDATA%\LuxiRisk\`
   - Override: `LUXIRISK_HOME`
3. Fingerprint = first 8 bytes of SHA-256(pubkey), hex (16 chars).

The fingerprint is a **stable pseudonymous identity** across receipts from that
install. It is **not** a global vendor key — each user (machine) has their own.

### Optional time binding

| Flag | Behavior | Network |
|------|----------|---------|
| *(none)* | No beacon fields | Offline |
| `--beacon VALUE` | Include user-supplied beacon | Offline |
| `--stamp` | Fetch latest [drand](https://drand.love) randomness | **One HTTPS GET** |

Endpoint (only with `--stamp`):

```text
https://api.drand.sh/public/latest
```

Source label in payload: `drand/default`. Fields: `beacon_source`,
`beacon_round`, `beacon_value`.

Offline third parties can re-check the round against a public drand archive.

### Independent verification

**With the binary (offline):**

```bash
luxirisk verify lxr1_… --payload-file claim.txt
luxirisk verify lxr1_… liq --side long --entry 65000 --leverage 10 --expect-liq 58825
```

**Without the binary (Python, offline):**

```bash
pip install cryptography   # or pynacl
python3 test-vectors/verify_receipts.py
python3 test-vectors/verify_receipts.py --receipt 'lxr1_…' --payload-file claim.txt
```

### Test-vector identity (documentation only)

Public vectors are signed with a **fixed deterministic seed**:

```text
seed = SHA-256(utf8("luxirisk-v0.2-test-vector-identity"))
```

Fingerprint and public key are published in `test-vectors/vectors.json`.
This is **not** used for real user receipts.

---

## Non-goals (v0.2)

Not computed: funding rates, Kelly, Monte Carlo, prop-firm modes, exchange APIs.
Local UI removed (CLI only).
