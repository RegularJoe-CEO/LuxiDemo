//! Waller Operator: The production fused online-softmax causal attention kernel.
//! O(N) memory, single-pass, deterministic. Core of the GAE contribution.

use crate::online_softmax::OnlineSoftmax;

/// Per-head (per query-row in the streaming sense) persistent state for the
/// online-softmax causal attention. This is what allows incremental extension
/// without re-reading the entire prefix.
#[derive(Clone, Debug)]
pub struct WallerKVState {
    /// Cached Keys for the prefix (one Vec<f32> of head_dim per previous token).
    /// This is the "recurrent" part that lets us avoid re-materializing the prefix.
    pub k_cache: Vec<Vec<f32>>,
    /// Cached Values for the prefix.
    pub v_cache: Vec<Vec<f32>>,
}

impl Default for WallerKVState {
    fn default() -> Self {
        Self::new()
    }
}

impl WallerKVState {
    pub fn new() -> Self {
        Self {
            k_cache: Vec::new(),
            v_cache: Vec::new(),
        }
    }

    /// Append a new token's K and V (for this head) to the persistent cache.
    /// This is the "extend" operation for the recurrent state.
    pub fn extend(&mut self, new_k: &[f32], new_v: &[f32]) {
        self.k_cache.push(new_k.to_vec());
        self.v_cache.push(new_v.to_vec());
    }

    pub fn len(&self) -> usize {
        self.k_cache.len()
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn is_empty(&self) -> bool {
        self.k_cache.is_empty()
    }
}

/// Compute the attention output for a *single new query row* against a persisted
/// WallerKVState (the cached K/V for the prefix including any just-extended tokens).
/// This is the core of incremental/recurrent generation.
///
/// The computation uses the exact same online-softmax math and left-to-right
/// processing order as the full waller_operator, guaranteeing bit-identical
/// results (to_bits() equality) when compared to a full recompute over the
/// extended sequence.
pub fn waller_attention_for_new_query(
    q_row: &[f32],
    state: &WallerKVState,
    head_dim: usize,
    scale: f32,
) -> Vec<f32> {
    if state.k_cache.is_empty() {
        // No previous context; just return zeros (or handle edge case).
        return vec![0.0; head_dim];
    }

    let mut softmax = OnlineSoftmax::new();
    let mut acc = vec![0.0f32; head_dim];

    // Stream through the persisted cache in the original left-to-right order.
    for (k_row, v_row) in state.k_cache.iter().zip(state.v_cache.iter()) {
        let score: f32 = q_row
            .iter()
            .zip(k_row.iter())
            .map(|(&qi, &ki)| qi * ki)
            .sum::<f32>()
            * scale;

        let old_max = softmax.max;
        softmax.update(score);

        if old_max != f32::NEG_INFINITY {
            let correction = (old_max - softmax.max).exp();
            for a in &mut acc {
                *a *= correction;
            }
        }

        let weight = (score - softmax.max).exp();
        for (a, &vi) in acc.iter_mut().zip(v_row.iter()) {
            *a += weight * vi;
        }
    }

    // Final normalization for this query row.
    let mut out = vec![0.0f32; head_dim];
    for (o, a) in out.iter_mut().zip(acc.iter()) {
        *o = *a / softmax.sum;
    }
    out
}

/// Waller Operator — single-pass fused causal attention (Q, K, V are [seq, head_dim]).
/// Returns output of same shape. Scale is usually 1/sqrt(head_dim).
pub fn waller_operator(
    q: &[f32],
    k: &[f32],
    v: &[f32],
    seq_len: usize,
    head_dim: usize,
    scale: f32,
) -> Vec<f32> {
    let mut output = vec![0.0; seq_len * head_dim];

    for i in 0..seq_len {
        let q_row = &q[i * head_dim..(i + 1) * head_dim];
        let mut softmax = OnlineSoftmax::new();
        let mut acc = vec![0.0; head_dim];

        for j in 0..=i {
            let k_row = &k[j * head_dim..(j + 1) * head_dim];
            let v_row = &v[j * head_dim..(j + 1) * head_dim];

            let score: f32 = q_row
                .iter()
                .zip(k_row.iter())
                .map(|(&qi, &ki)| qi * ki)
                .sum::<f32>()
                * scale;

            let old_max = softmax.max;
            softmax.update(score);

            if old_max != f32::NEG_INFINITY {
                let correction = (old_max - softmax.max).exp();
                for a in &mut acc {
                    *a *= correction;
                }
            }

            let weight = (score - softmax.max).exp();
            for (a, &vi) in acc.iter_mut().zip(v_row.iter()) {
                *a += weight * vi;
            }
        }

        let out_row = &mut output[i * head_dim..(i + 1) * head_dim];
        for (o, a) in out_row.iter_mut().zip(acc.iter()) {
            *o = a / softmax.sum;
        }
    }
    output
}

/// Parallel version (rayon feature). Falls back to serial when rayon disabled.
#[cfg(feature = "rayon")]
pub fn waller_operator_parallel(
    q: &[f32],
    k: &[f32],
    v: &[f32],
    seq_len: usize,
    head_dim: usize,
    scale: f32,
) -> Vec<f32> {
    // For production simplicity the serial version is the reference.
    // A full parallel blocked version can be added later without changing the contract.
    waller_operator(q, k, v, seq_len, head_dim, scale)
}

#[cfg(not(feature = "rayon"))]
pub fn waller_operator_parallel(
    q: &[f32],
    k: &[f32],
    v: &[f32],
    seq_len: usize,
    head_dim: usize,
    scale: f32,
) -> Vec<f32> {
    waller_operator(q, k, v, seq_len, head_dim, scale)
}

#[cfg(test)]
mod tests {
    use super::{waller_attention_for_new_query, waller_operator, WallerKVState};

    // Naive reference: materializing scores for verification (only for small seq).
    fn naive_causal_attention(
        q: &[f32],
        k: &[f32],
        v: &[f32],
        seq: usize,
        d: usize,
        scale: f32,
    ) -> Vec<f32> {
        let mut out = vec![0.0f32; seq * d];
        for i in 0..seq {
            let q_row = &q[i * d..(i + 1) * d];
            let mut scores = vec![f32::NEG_INFINITY; seq];
            for j in 0..=i {
                let k_row = &k[j * d..(j + 1) * d];
                let mut dot = 0.0f32;
                for dd in 0..d {
                    dot += q_row[dd] * k_row[dd];
                }
                scores[j] = dot * scale;
            }
            // softmax only over 0..=i
            let max = scores
                .iter()
                .take(i + 1)
                .cloned()
                .fold(f32::NEG_INFINITY, f32::max);
            let mut exps = vec![0.0f32; seq];
            let mut sum = 0.0f32;
            for j in 0..=i {
                exps[j] = (scores[j] - max).exp();
                sum += exps[j];
            }
            for j in 0..=i {
                exps[j] /= sum;
            }
            // weighted sum of V
            for dd in 0..d {
                let mut acc = 0.0f32;
                for j in 0..=i {
                    let v_row = &v[j * d..(j + 1) * d];
                    acc += exps[j] * v_row[dd];
                }
                out[i * d + dd] = acc;
            }
        }
        out
    }

    #[test]
    fn waller_operator_matches_naive_causal_on_small() {
        let seq = 4usize;
        let d = 3usize;
        let scale = 1.0f32 / (d as f32).sqrt();

        // Deterministic synthetic Q/K/V
        let q: Vec<f32> = (0..seq * d).map(|i| (i as f32 * 0.1).sin() * 0.5).collect();
        let k: Vec<f32> = (0..seq * d)
            .map(|i| (i as f32 * 0.13).cos() * 0.5)
            .collect();
        let v: Vec<f32> = (0..seq * d)
            .map(|i| (i as f32 * 0.07).sin() * 0.7)
            .collect();

        let ours = waller_operator(&q, &k, &v, seq, d, scale);
        let naive = naive_causal_attention(&q, &k, &v, seq, d, scale);

        for (o, n) in ours.iter().zip(naive.iter()) {
            assert!((o - n).abs() < 1e-4, "waller mismatch {} vs {}", o, n);
        }
    }

    #[test]
    fn waller_operator_is_deterministic() {
        let seq = 3;
        let d = 2;
        let q = vec![0.1f32; seq * d];
        let k = vec![0.2f32; seq * d];
        let v = vec![0.3f32; seq * d];
        let a = waller_operator(&q, &k, &v, seq, d, 0.5);
        let b = waller_operator(&q, &k, &v, seq, d, 0.5);
        for (x, y) in a.iter().zip(b.iter()) {
            assert_eq!(x.to_bits(), y.to_bits());
        }
    }

    #[test]
    fn waller_kv_state_incremental_matches_full_recompute() {
        // Prove that building the KV cache incrementally + computing attention
        // for a new query row using the persisted state gives bit-identical
        // results to a full waller_operator call over the extended sequence.
        let d = 4usize; // head_dim
        let scale = 1.0f32 / (d as f32).sqrt();

        // Simulate a growing sequence for one head
        let mut full_q = Vec::new();
        let mut full_k = Vec::new();
        let mut full_v = Vec::new();

        let mut state = WallerKVState::new();

        for t in 0..5 {
            // New token's q/k/v for this head (synthetic but deterministic)
            let q_t: Vec<f32> = (0..d).map(|i| ((t * 7 + i) as f32 * 0.1).sin() * 0.3).collect();
            let k_t: Vec<f32> = (0..d).map(|i| ((t * 11 + i) as f32 * 0.1).cos() * 0.3).collect();
            let v_t: Vec<f32> = (0..d).map(|i| ((t * 13 + i) as f32 * 0.1).sin() * 0.4).collect();

            full_q.extend_from_slice(&q_t);
            full_k.extend_from_slice(&k_t);
            full_v.extend_from_slice(&v_t);

            // Incremental path
            state.extend(&k_t, &v_t);

            // Full recompute for the current length (to get the "ground truth" last row)
            let full_out = waller_operator(&full_q, &full_k, &full_v, t + 1, d, scale);
            let full_last_row = &full_out[t * d..(t + 1) * d];

            // Incremental attention for the *new* query row against the persisted KV state
            let inc_last_row = waller_attention_for_new_query(&q_t, &state, d, scale);

            for (a, b) in full_last_row.iter().zip(inc_last_row.iter()) {
                assert_eq!((*a).to_bits(), (*b).to_bits(), "mismatch at step {}", t);
            }
        }
    }
}
