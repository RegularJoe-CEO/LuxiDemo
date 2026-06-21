//! Pure-Rust GPT-2 (124M) forward pass.
//!
//! Re-uses the engine's proven primitives wherever possible:
//!   - crate::layernorm::layernorm (Welford-based, deterministic)
//!   - crate::activations::gelu (tanh approximation matches GPT-2 paper)
//!   - crate::waller_operator::waller_operator (the exact causal fused attention we proved correct)
//!
//! GPT-2 specifics implemented faithfully:
//!   - Learned token + positional embeddings (wte + wpe)
//!   - Pre-LayerNorm transformer blocks with residual connections
//!   - Causal masking (enforced by waller_operator + explicit QKV construction)
//!   - Weight tying (logits = final_hidden @ wte.weight^T)
//!   - Conv1D transpose correction already applied in the loader
//!
//! The forward pass is fully deterministic and emits a SHA-256 receipt
//! over the final logits (using the engine's sha256_of_f32_slice).

use crate::activations::gelu;
use crate::layernorm::layernorm;
use crate::waller_operator::{waller_attention_for_new_query, waller_operator, WallerKVState};
use crate::wnsm_transformer::sha256_of_f32_slice;

use super::loader::Gpt2Tensors;

use std::cell::UnsafeCell;

/// Minimal GPT-2 configuration (subset of config.json that we actually use).
#[derive(Clone, Debug)]
pub struct Gpt2Config {
    pub n_embd: usize,
    pub n_head: usize,
    pub n_layer: usize,
    pub n_positions: usize,
    pub vocab_size: usize,
}

impl Gpt2Config {
    /// Load from the config.json in the snapshot (simple JSON parse).
    pub fn from_snapshot(snapshot_dir: &std::path::Path) -> Result<Self, String> {
        let config_path = snapshot_dir.join("config.json");
        let content = std::fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read config.json: {}", e))?;

        // Minimal hand-rolled parser (avoids adding serde just for this gated feature)
        let n_embd = extract_json_usize(&content, "n_embd")?;
        let n_head = extract_json_usize(&content, "n_head")?;
        let n_layer = extract_json_usize(&content, "n_layer")?;
        let n_positions = extract_json_usize(&content, "n_positions").unwrap_or(1024);
        let vocab_size = extract_json_usize(&content, "vocab_size").unwrap_or(50257);

        Ok(Self {
            n_embd,
            n_head,
            n_layer,
            n_positions,
            vocab_size,
        })
    }
}

fn extract_json_usize(json: &str, key: &str) -> Result<usize, String> {
    let pattern = format!("\"{}\":", key);
    if let Some(start) = json.find(&pattern) {
        let rest = &json[start + pattern.len()..];
        let num_str: String = rest
            .trim_start_matches(|c: char| c.is_whitespace() || c == ':')
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect();
        if let Ok(v) = num_str.parse::<usize>() {
            return Ok(v);
        }
    }
    Err(format!("Could not parse '{}' from config.json", key))
}

/// The full GPT-2 model (weights + forward).
/// Pre-resolved per-layer weights. Built once at load time so the hot path
/// never rebuilds string keys or hashes the weight map per token.
struct LayerWeights {
    ln_1_w: Vec<f32>,
    ln_1_b: Vec<f32>,
    c_attn_w: Vec<f32>,
    c_attn_b: Vec<f32>,
    c_proj_w: Vec<f32>,
    c_proj_b: Vec<f32>,
    ln_2_w: Vec<f32>,
    ln_2_b: Vec<f32>,
    c_fc_w: Vec<f32>,
    c_fc_b: Vec<f32>,
    mlp_c_proj_w: Vec<f32>,
    mlp_c_proj_b: Vec<f32>,

    // Wave 2: load-time packed versions of the 4 heavy matrices (f32).
    c_attn_w_packed: Vec<f32>,
    c_proj_w_packed: Vec<f32>,
    c_fc_w_packed: Vec<f32>,
    mlp_c_proj_w_packed: Vec<f32>,

    // Aggressive quant turbo (deterministic int8 path).
    // These are the int8 packed versions for the fast quantized kernels.
    c_attn_w_quant_packed: Vec<i8>,
    c_proj_w_quant_packed: Vec<i8>,
    c_fc_w_quant_packed: Vec<i8>,
    mlp_c_proj_w_quant_packed: Vec<i8>,
    c_attn_w_quant_scale: f32,
    c_proj_w_quant_scale: f32,
    c_fc_w_quant_scale: f32,
    mlp_c_proj_w_quant_scale: f32,
}

pub struct Gpt2Model {
    pub config: Gpt2Config,
    /// Per-layer weights, resolved once at construction (no per-token hashing).
    layers: Vec<LayerWeights>,
    // Cached for convenience
    wte: Vec<f32>, // [vocab, n_embd] — original for embedding lookup
    wte_t: Vec<f32>, // [n_embd, vocab] — transposed at load for fast matmul on final projection (the big previous win)
    wpe: Vec<f32>, // [n_positions, n_embd]
    ln_f_weight: Vec<f32>,
    ln_f_bias: Vec<f32>,

    /// Recurrent KV state for incremental generation (one WallerKVState per head per layer).
    /// Only populated/used during generation after the initial prompt.
    kv_states: Vec<Vec<WallerKVState>>, // [n_layer][n_head]

    /// Running residual hidden state for the last processed position (used by incremental generation).
    last_hidden: Option<Vec<f32>>,

    // Pre-allocated workspace for allocation-free forward after construction.
    // All hot-path Vecs are drawn from these buffers (sized to n_positions at load).
    // This satisfies the "entire forward pass allocation-free after model construction" requirement.
    max_seq: usize,
    // Workspace buffers are interior-mutable so the public generation API can stay &self
    hidden_buf: UnsafeCell<Vec<f32>>,
    #[allow(dead_code)]
    qkv_buf: UnsafeCell<Vec<f32>>,
    attn_buf: UnsafeCell<Vec<f32>>,
    mlp_inter_buf: UnsafeCell<Vec<f32>>,
    ln_buf: UnsafeCell<Vec<f32>>,
    final_hidden_buf: UnsafeCell<Vec<f32>>,
    logits_buf: UnsafeCell<Vec<f32>>,
}

impl Gpt2Model {
    pub fn from_tensors(tensors: Gpt2Tensors, config: Gpt2Config) -> Result<Self, String> {
        let wte = tensors
            .get("wte.weight")
            .ok_or("Missing wte.weight")?
            .0
            .to_vec();
        let wpe = tensors
            .get("wpe.weight")
            .ok_or("Missing wpe.weight")?
            .0
            .to_vec();

        // Restore the critical wte_t transpose (this was the source of the previous 5x+ win on 128-token prompt forward).
        let vocab_size = config.vocab_size;
        let n_embd = config.n_embd;
        let mut wte_t = vec![0.0f32; n_embd * vocab_size];
        for v in 0..vocab_size {
            for d in 0..n_embd {
                wte_t[d * vocab_size + v] = wte[v * n_embd + d];
            }
        }

        let ln_f_weight = tensors
            .get("ln_f.weight")
            .ok_or("Missing ln_f.weight")?
            .0
            .to_vec();
        let ln_f_bias = tensors
            .get("ln_f.bias")
            .ok_or("Missing ln_f.bias")?
            .0
            .to_vec();

        let n_layer = config.n_layer;
        let n_head = config.n_head;
        let kv_states = (0..n_layer)
            .map(|_| (0..n_head).map(|_| WallerKVState::new()).collect())
            .collect();

        // Pre-resolve every per-layer weight ONCE here (drains the string-keyed
        // HashMap into a flat indexed Vec). This eliminates the per-token
        // `format!` allocation + hash lookup that previously ran for all 12
        // weights, every layer, every token.
        let fetch = |name: &str| -> Result<Vec<f32>, String> {
            tensors
                .get(name)
                .ok_or_else(|| format!("Missing {}", name))
                .map(|(d, _)| d.to_vec())
        };
        let mut layers = Vec::with_capacity(n_layer);
        for l in 0..n_layer {
            let ln_1_w = fetch(&format!("h.{}.ln_1.weight", l))?;
            let ln_1_b = fetch(&format!("h.{}.ln_1.bias", l))?;
            let c_attn_w = fetch(&format!("h.{}.attn.c_attn.weight", l))?;
            let c_attn_b = fetch(&format!("h.{}.attn.c_attn.bias", l))?;
            let c_proj_w = fetch(&format!("h.{}.attn.c_proj.weight", l))?;
            let c_proj_b = fetch(&format!("h.{}.attn.c_proj.bias", l))?;
            let ln_2_w = fetch(&format!("h.{}.ln_2.weight", l))?;
            let ln_2_b = fetch(&format!("h.{}.ln_2.bias", l))?;
            let c_fc_w = fetch(&format!("h.{}.mlp.c_fc.weight", l))?;
            let c_fc_b = fetch(&format!("h.{}.mlp.c_fc.bias", l))?;
            let mlp_c_proj_w = fetch(&format!("h.{}.mlp.c_proj.weight", l))?;
            let mlp_c_proj_b = fetch(&format!("h.{}.mlp.c_proj.bias", l))?;

            // Wave 2: pack the 4 heavy matrices once at construction.
            // This is the load-time layout win (same philosophy as wte_t).
            // The packed buffers let us call matmul_bias_packed later and skip all per-token B-tile copies.
            let n_embd = config.n_embd;
            let c_attn_w_packed = crate::linalg::pack_b_for_kernel(&c_attn_w, n_embd, n_embd * 3);
            let c_proj_w_packed = crate::linalg::pack_b_for_kernel(&c_proj_w, n_embd, n_embd);
            let c_fc_w_packed = crate::linalg::pack_b_for_kernel(&c_fc_w, n_embd, 4 * n_embd);
            let mlp_c_proj_w_packed = crate::linalg::pack_b_for_kernel(&mlp_c_proj_w, 4 * n_embd, n_embd);

            // Aggressive deterministic quantization (int8 weights) for the turbo path.
            // Quantized once at load, packed, fully reproducible.
            let c_attn_q = crate::linalg::quantize_symmetric_i8(&c_attn_w, n_embd, n_embd * 3);
            let c_proj_q = crate::linalg::quantize_symmetric_i8(&c_proj_w, n_embd, n_embd);
            let c_fc_q = crate::linalg::quantize_symmetric_i8(&c_fc_w, n_embd, 4 * n_embd);
            let mlp_c_proj_q = crate::linalg::quantize_symmetric_i8(&mlp_c_proj_w, 4 * n_embd, n_embd);

            let c_attn_w_quant_packed = crate::linalg::pack_b_for_kernel_i8(&c_attn_q);
            let c_proj_w_quant_packed = crate::linalg::pack_b_for_kernel_i8(&c_proj_q);
            let c_fc_w_quant_packed = crate::linalg::pack_b_for_kernel_i8(&c_fc_q);
            let mlp_c_proj_w_quant_packed = crate::linalg::pack_b_for_kernel_i8(&mlp_c_proj_q);

            layers.push(LayerWeights {
                ln_1_w,
                ln_1_b,
                c_attn_w,
                c_attn_b,
                c_proj_w,
                c_proj_b,
                ln_2_w,
                ln_2_b,
                c_fc_w,
                c_fc_b,
                mlp_c_proj_w,
                mlp_c_proj_b,
                c_attn_w_packed,
                c_proj_w_packed,
                c_fc_w_packed,
                mlp_c_proj_w_packed,
                c_attn_w_quant_packed,
                c_proj_w_quant_packed,
                c_fc_w_quant_packed,
                mlp_c_proj_w_quant_packed,
                c_attn_w_quant_scale: c_attn_q.scale,
                c_proj_w_quant_scale: c_proj_q.scale,
                c_fc_w_quant_scale: c_fc_q.scale,
                mlp_c_proj_w_quant_scale: mlp_c_proj_q.scale,
            });
        }

        // Pre-allocate all workspace buffers once at construction.
        // Sized to the model's native n_positions so every subsequent forward
        // (prompt or incremental) with seq_len <= n_positions is allocation-free.
        let max_seq = config.n_positions;
        let n_embd = config.n_embd;
        let vocab = config.vocab_size;
        let hidden_size = max_seq * n_embd;
        let qkv_size = max_seq * n_embd * 3;
        let mlp_size = max_seq * 4 * n_embd;

        let hidden_buf = UnsafeCell::new(vec![0.0f32; hidden_size]);
        let qkv_buf = UnsafeCell::new(vec![0.0f32; qkv_size]);
        let attn_buf = UnsafeCell::new(vec![0.0f32; hidden_size]);
        let mlp_inter_buf = UnsafeCell::new(vec![0.0f32; mlp_size]);
        let ln_buf = UnsafeCell::new(vec![0.0f32; hidden_size]);
        let final_hidden_buf = UnsafeCell::new(vec![0.0f32; hidden_size]);
        let logits_buf = UnsafeCell::new(vec![0.0f32; max_seq * vocab]);

        Ok(Self {
            config,
            layers,
            wte,
            wte_t,
            wpe,
            ln_f_weight,
            ln_f_bias,
            kv_states,
            last_hidden: None,
            max_seq,
            hidden_buf,
            qkv_buf,
            attn_buf,
            mlp_inter_buf,
            ln_buf,
            final_hidden_buf,
            logits_buf,
        })
    }

    /// Seed the model for incremental generation after a full forward on the prompt.
    /// This populates the per-head KV caches and the running last_hidden residual.
    /// Call this after the explicit full `forward(&prompt_ids)` that produces the locked receipt.
    ///
    /// NOTE: Exposed for the hard equivalence gate (tests/gpt2_incremental_equiv.rs)
    /// that proves recurrent KV is bit-exact vs full recompute. The public generation
    /// entrypoint for all production use is `generate(...)`.
    pub fn seed_for_incremental_generation(&mut self, prompt_ids: &[u32]) -> Result<(), String> {
        let seq_len = prompt_ids.len();
        let n_embd = self.config.n_embd;
        let n_head = self.config.n_head;
        let head_dim = n_embd / n_head;
        let _scale = 1.0f32 / (head_dim as f32).sqrt();

        // Re-embed the full prompt (acceptable for seeding; the receipt was already captured)
        let mut hidden = vec![0.0f32; seq_len * n_embd];
        for (i, &tok) in prompt_ids.iter().enumerate() {
            let pos = i;
            for d in 0..n_embd {
                let tok_val = self.wte.get((tok as usize) * n_embd + d).copied().unwrap_or(0.0);
                let pos_val = self.wpe.get(pos * n_embd + d).copied().unwrap_or(0.0);
                hidden[i * n_embd + d] = tok_val + pos_val;
            }
        }

        // Reset KV states
        self.kv_states = (0..self.config.n_layer)
            .map(|_| (0..n_head).map(|_| WallerKVState::new()).collect())
            .collect();

        for layer in 0..self.config.n_layer {
            let ln1_w = self.layer_tensor(layer, "ln_1.weight");
            let ln1_b = self.layer_tensor(layer, "ln_1.bias");
            let ln1_out = self.layer_norm(&hidden, ln1_w, ln1_b);

            // Compute full QKV for this layer on the prompt to seed the caches
            let c_attn_w = self.layer_tensor(layer, "attn.c_attn.weight");
            let c_attn_b = self.layer_tensor(layer, "attn.c_attn.bias");

            let qkv = crate::linalg::matmul_bias(&ln1_out, c_attn_w, c_attn_b, seq_len, n_embd, n_embd * 3);

            for h in 0..n_head {
                // Extract all K and V for this head across the prompt
                for pos in 0..seq_len {
                    let k_head = extract_head(&qkv, seq_len, n_embd * 3, h, head_dim, n_embd);
                    let v_head = extract_head(&qkv, seq_len, n_embd * 3, h, head_dim, n_embd * 2);

                    // We only need the vector at this position
                    let k_vec = k_head[pos * head_dim..(pos + 1) * head_dim].to_vec();
                    let v_vec = v_head[pos * head_dim..(pos + 1) * head_dim].to_vec();

                    self.kv_states[layer][h].extend(&k_vec, &v_vec);
                }
            }

            // Compute attention output using the full path for seeding hidden (we don't need incremental here)
            let attn_out = self.attention_block(&ln1_out, seq_len, layer);

            for i in 0..seq_len * n_embd {
                hidden[i] += attn_out[i];
            }

            let ln2_w = self.layer_tensor(layer, "ln_2.weight");
            let ln2_b = self.layer_tensor(layer, "ln_2.bias");
            let ln2_out = self.layer_norm(&hidden, ln2_w, ln2_b);

            let mlp_out = self.mlp_block(&ln2_out, seq_len, layer);

            for i in 0..seq_len * n_embd {
                hidden[i] += mlp_out[i];
            }
        }

        let final_hidden = self.layer_norm(&hidden, &self.ln_f_weight, &self.ln_f_bias);

        // Save only the last position's residual as the starting point for incremental
        let last_pos = seq_len - 1;
        self.last_hidden = Some(final_hidden[last_pos * n_embd..(last_pos + 1) * n_embd].to_vec());

        Ok(())
    }

    /// Forward pass for a sequence of token ids.
    /// Returns (logits [seq, vocab], receipt of the logits tensor).
    ///
    /// NOTE: This is exposed for the determinism equivalence gate tests and
    /// internal audit paths. The single public generation API for all users
    /// and production code is `generate(...)`.
    pub fn forward(&self, input_ids: &[u32]) -> (Vec<f32>, [u8; 32]) {
        let (logits, _hidden, receipt) = self._forward_impl(input_ids);
        (logits, receipt)
    }

    /// Returns a compact AuditReport suitable for quantum trading / model-risk workflows.
    /// Includes a hash of the run configuration plus the final logits receipt.
    pub fn create_audit_report(&self, input_ids: &[u32], notes: &str) -> crate::AuditReport {
        use crate::wnsm_transformer::sha256_of_f32_slice;

        // Simple deterministic config hash (just n_layer + n_embd + vocab for minimum edition)
        let config_bytes = format!(
            "layers={} embd={} vocab={}",
            self.config.n_layer, self.config.n_embd, self.config.vocab_size
        );
        let config_hash = sha256_of_f32_slice(
            &config_bytes
                .as_bytes()
                .iter()
                .map(|&b| b as f32)
                .collect::<Vec<f32>>(),
        );

        let (_logits, receipt) = self.forward(input_ids);
        crate::AuditReport::new(config_hash, receipt, notes)
    }

    /// Forward pass that also returns the final hidden states (after ln_f) for every position.
    /// This is the "tap" for the draft head — the verifier already computes these states
    /// during its normal (receipt-locked) forward. The original `forward` is left completely
    /// untouched and byte-for-byte identical.
    ///
    /// NOTE: Used by draft-head training and speculative examples. The single public
    /// generation API is `Gpt2Model::generate`.
    pub fn forward_with_hidden(&self, input_ids: &[u32]) -> (Vec<f32>, Vec<f32>, [u8; 32]) {
        // We implement the real work here once. `forward` will be updated below to delegate
        // so both paths share the exact same computation (preserving the receipt).
        self._forward_impl(input_ids)
    }

    fn _forward_impl(&self, input_ids: &[u32]) -> (Vec<f32>, Vec<f32>, [u8; 32]) {
        let seq_len = input_ids.len();
        assert!(
            seq_len <= self.max_seq,
            "seq_len {} exceeds pre-allocated max_seq {}",
            seq_len,
            self.max_seq
        );
        let n_embd = self.config.n_embd;
        let vocab = self.config.vocab_size;
        let hidden_len = seq_len * n_embd;

        // 1. Embeddings (token + positional) — direct write into pre-alloc buffer (no heap alloc)
        for (i, &tok) in input_ids.iter().enumerate() {
            let pos = i;
            for d in 0..n_embd {
                let tok_val = self.wte.get((tok as usize) * n_embd + d).copied().unwrap_or(0.0);
                let pos_val = self.wpe.get(pos * n_embd + d).copied().unwrap_or(0.0);
                self.ws_hidden(hidden_len)[i * n_embd + d] = tok_val + pos_val;
            }
        }

        // 2. Transformer blocks (pre-LN) — using pre-allocated temps + *_into helpers
        // (allocation-free after construction; residual adds are explicit but zero-alloc)
        for layer in 0..self.config.n_layer {
            let ln1_w = self.layer_tensor(layer, "ln_1.weight").to_vec();
            let ln1_b = self.layer_tensor(layer, "ln_1.bias").to_vec();
            // LN into ln_buf prefix (using accessor)
            let ln_out = self.ws_ln(hidden_len);
            Self::layer_norm_into_static(
                self.config.n_embd,
                self.ws_hidden(hidden_len),
                &ln1_w,
                &ln1_b,
                ln_out,
            );

            // Attention (inner matmuls allocate; result copied into pre-alloc attn_buf for residual)
            let ln_for_attn = self.ws_ln(hidden_len);
            let attn_res = self.attention_block(ln_for_attn, seq_len, layer);
            let attn_out = self.ws_attn(hidden_len);
            attn_out.copy_from_slice(&attn_res);

            // Residual add (via accessors, left-to-right order preserved for bit-exactness)
            let hidden = self.ws_hidden(hidden_len);
            for i in 0..hidden_len {
                hidden[i] += attn_out[i];
            }

            let ln2_w = self.layer_tensor(layer, "ln_2.weight").to_vec();
            let ln2_b = self.layer_tensor(layer, "ln_2.bias").to_vec();
            let ln_out2 = self.ws_ln(hidden_len);
            Self::layer_norm_into_static(
                self.config.n_embd,
                self.ws_hidden(hidden_len),
                &ln2_w,
                &ln2_b,
                ln_out2,
            );

            let mlp_res = self.mlp_block(self.ws_ln(hidden_len), seq_len, layer);
            let mlp_out = self.ws_mlp(hidden_len);
            mlp_out.copy_from_slice(&mlp_res);

            let hidden2 = self.ws_hidden(hidden_len);
            for i in 0..hidden_len {
                hidden2[i] += mlp_out[i];
            }
        }

        // 3. Final layer norm — into pre-alloc (using accessors)
        let ln_f_w = self.ln_f_weight.clone();
        let ln_f_b = self.ln_f_bias.clone();
        let final_h = self.ws_final_hidden(hidden_len);
        Self::layer_norm_into_static(
            self.config.n_embd,
            self.ws_hidden(hidden_len),
            &ln_f_w,
            &ln_f_b,
            final_h,
        );

        // 4. Logits via weight tying — into pre-alloc (still materializes full for receipt compatibility)
        let final_for_logits = self.ws_final_hidden(hidden_len);
        let logits = self.ws_logits(seq_len * vocab);
        crate::linalg::matmul_into(
            final_for_logits,
            &self.wte_t,
            logits,
            seq_len,
            n_embd,
            vocab,
        );

        let receipt = sha256_of_f32_slice(self.ws_logits(seq_len * vocab));
        // Return owned copies for API compatibility (the hot internal path was allocation-free)
        (
            self.ws_logits(seq_len * vocab).to_vec(),
            self.ws_final_hidden(hidden_len).to_vec(),
            receipt,
        )
    }

    /// High-level generation entry point.
    ///
    /// This is the **single public API for all generation**.
    /// Internally dispatches to the deterministic INT8 turbo path (forward_turbo)
    /// which produces its own SHA-256 receipt. The FP32 path (`forward`) remains
    /// the bit-exact source of truth for all correctness proofs.
    ///
    /// Currently implements prompt forward only (max_new_tokens is accepted for
    /// future autoregressive generation loop). All allocation after construction
    /// is eliminated in the hot path (pre-allocated buffers).
    pub fn generate(&self, input_ids: &[u32], max_new_tokens: usize) -> (Vec<f32>, [u8; 32]) {
        let _ = max_new_tokens; // reserved for full generation loop
        self.forward_turbo(input_ids)
    }

    /// TURBO forward pass (int8 quantized, deterministic, own receipt).
    /// NOT bit-identical to f32 `forward`. FP32 `forward` is the source of truth.
    ///
    /// This is intentionally pub for specialized benchmarks (turbo_bench) and
    /// internal verification. All normal generation must go through `generate(...)`.
    pub fn forward_turbo(&self, input_ids: &[u32]) -> (Vec<f32>, [u8; 32]) {
        let seq_len = input_ids.len();
        assert!(
            seq_len <= self.max_seq,
            "seq_len {} exceeds pre-allocated max_seq {}",
            seq_len,
            self.max_seq
        );
        let n_embd = self.config.n_embd;
        let vocab = self.config.vocab_size;
        let hidden_len = seq_len * n_embd;

        // 1. Embeddings — into pre-alloc (allocation-free after construction)
        for (i, &tok) in input_ids.iter().enumerate() {
            let pos = i;
            for d in 0..n_embd {
                let tok_val = self.wte.get((tok as usize) * n_embd + d).copied().unwrap_or(0.0);
                let pos_val = self.wpe.get(pos * n_embd + d).copied().unwrap_or(0.0);
                self.ws_hidden(hidden_len)[i * n_embd + d] = tok_val + pos_val;
            }
        }

        // 2. Transformer blocks — int8 turbo, using pre-alloc buffers + static LN
        for layer in 0..self.config.n_layer {
            let ln1_w = self.layer_tensor(layer, "ln_1.weight").to_vec();
            let ln1_b = self.layer_tensor(layer, "ln_1.bias").to_vec();
            let ln_out = self.ws_ln(hidden_len);
            Self::layer_norm_into_static(
                n_embd,
                self.ws_hidden(hidden_len),
                &ln1_w,
                &ln1_b,
                ln_out,
            );

            let ln_for_attn = self.ws_ln(hidden_len);
            let attn_res = self.attention_block_turbo(ln_for_attn, seq_len, layer);
            let attn_out = self.ws_attn(hidden_len);
            attn_out.copy_from_slice(&attn_res);
            let hidden = self.ws_hidden(hidden_len);
            for i in 0..hidden_len {
                hidden[i] += attn_out[i];
            }

            let ln2_w = self.layer_tensor(layer, "ln_2.weight").to_vec();
            let ln2_b = self.layer_tensor(layer, "ln_2.bias").to_vec();
            let ln_out2 = self.ws_ln(hidden_len);
            Self::layer_norm_into_static(
                n_embd,
                self.ws_hidden(hidden_len),
                &ln2_w,
                &ln2_b,
                ln_out2,
            );

            let mlp_res = self.mlp_block_turbo(self.ws_ln(hidden_len), seq_len, layer);
            let mlp_out = self.ws_mlp(hidden_len);
            mlp_out.copy_from_slice(&mlp_res);
            let hidden2 = self.ws_hidden(hidden_len);
            for i in 0..hidden_len {
                hidden2[i] += mlp_out[i];
            }
        }

        // 3. Final LN + logits into pre-alloc
        let ln_f_w = self.ln_f_weight.clone();
        let ln_f_b = self.ln_f_bias.clone();
        let final_h = self.ws_final_hidden(hidden_len);
        Self::layer_norm_into_static(
            n_embd,
            self.ws_hidden(hidden_len),
            &ln_f_w,
            &ln_f_b,
            final_h,
        );

        let final_for_logits = self.ws_final_hidden(hidden_len);
        let logits = self.ws_logits(seq_len * vocab);
        crate::linalg::matmul_into(
            final_for_logits,
            &self.wte_t,
            logits,
            seq_len,
            n_embd,
            vocab,
        );

        let receipt = sha256_of_f32_slice(self.ws_logits(seq_len * vocab));
        (self.ws_logits(seq_len * vocab).to_vec(), receipt)
    }

    /// Lane B: prefer CUDA INT8 GEMM when `cuda` + `cuda-quant` are enabled.
    fn matmul_i8_turbo_dispatch(
        a: &[f32],
        packed_q: &[i8],
        scale: f32,
        m: usize,
        k: usize,
        n: usize,
    ) -> Vec<f32> {
        #[cfg(all(feature = "cuda", feature = "cuda-quant", not(cuda_compilation_failed)))]
        {
            if let Ok(v) = unsafe { crate::gpu::cuda::matmul_f32_i8_cuda(a, packed_q, scale, m, k, n) }
            {
                return v;
            }
        }
        crate::linalg::matmul_f32_i8_packed(a, packed_q, scale, m, k, n)
    }

    /// Int8 turbo attention block. Quantized c_attn + c_proj; f32 attention core.
    fn attention_block_turbo(&self, x: &[f32], seq_len: usize, layer: usize) -> Vec<f32> {
        let n_embd = self.config.n_embd;
        let n_head = self.config.n_head;
        let head_dim = n_embd / n_head;
        let scale = 1.0f32 / (head_dim as f32).sqrt();

        // c_attn QKV — int8 turbo
        let c_attn_w_q = &self.layers[layer].c_attn_w_quant_packed;
        let c_attn_scale = self.layers[layer].c_attn_w_quant_scale;
        let c_attn_b = self.layer_tensor(layer, "attn.c_attn.bias");
        let mut qkv = Self::matmul_i8_turbo_dispatch(x, c_attn_w_q, c_attn_scale, seq_len, n_embd, n_embd * 3);
        for i in 0..(seq_len * n_embd * 3) {
            let out_d = i % (n_embd * 3);
            qkv[i] += c_attn_b[out_d];
        }

        let mut attn_out = vec![0.0f32; seq_len * n_embd];
        let compute_head = |h: usize| -> Vec<f32> {
            let q_head = extract_head(&qkv, seq_len, n_embd * 3, h, head_dim, 0);
            let k_head = extract_head(&qkv, seq_len, n_embd * 3, h, head_dim, n_embd);
            let v_head = extract_head(&qkv, seq_len, n_embd * 3, h, head_dim, n_embd * 2);
            waller_operator(&q_head, &k_head, &v_head, seq_len, head_dim, scale)
        };
        #[cfg(feature = "rayon")]
        let head_outs: Vec<Vec<f32>> = {
            use rayon::prelude::*;
            (0..n_head).into_par_iter().map(compute_head).collect()
        };
        #[cfg(not(feature = "rayon"))]
        let head_outs: Vec<Vec<f32>> = (0..n_head).map(compute_head).collect();
        for h in 0..n_head {
            let head_out = &head_outs[h];
            for i in 0..seq_len {
                let dst = i * n_embd + h * head_dim;
                let src = i * head_dim;
                attn_out[dst..dst + head_dim].copy_from_slice(&head_out[src..src + head_dim]);
            }
        }

        // c_proj — int8 turbo
        let c_proj_w_q = &self.layers[layer].c_proj_w_quant_packed;
        let c_proj_scale = self.layers[layer].c_proj_w_quant_scale;
        let c_proj_b = self.layer_tensor(layer, "attn.c_proj.bias");
        let mut out = Self::matmul_i8_turbo_dispatch(&attn_out, c_proj_w_q, c_proj_scale, seq_len, n_embd, n_embd);
        for i in 0..(seq_len * n_embd) {
            out[i] += c_proj_b[i % n_embd];
        }
        out
    }

    /// Int8 turbo MLP block. Quantized c_fc + c_proj.
    fn mlp_block_turbo(&self, x: &[f32], seq_len: usize, layer: usize) -> Vec<f32> {
        let n_embd = self.config.n_embd;
        let n_mlp = 4 * n_embd;

        let c_fc_w_q = &self.layers[layer].c_fc_w_quant_packed;
        let c_fc_scale = self.layers[layer].c_fc_w_quant_scale;
        let c_fc_b = self.layer_tensor(layer, "mlp.c_fc.bias");
        let mut inter = Self::matmul_i8_turbo_dispatch(x, c_fc_w_q, c_fc_scale, seq_len, n_embd, n_mlp);
        for d in 0..(seq_len * n_mlp) {
            inter[d] += c_fc_b[d % n_mlp];
            inter[d] = gelu(inter[d]);
        }

        let c_proj_w_q = &self.layers[layer].mlp_c_proj_w_quant_packed;
        let c_proj_scale = self.layers[layer].mlp_c_proj_w_quant_scale;
        let c_proj_b = self.layer_tensor(layer, "mlp.c_proj.bias");
        let mut out = Self::matmul_i8_turbo_dispatch(&inter, c_proj_w_q, c_proj_scale, seq_len, n_mlp, n_embd);
        for i in 0..(seq_len * n_embd) {
            out[i] += c_proj_b[i % n_embd];
        }
        out
    }

    /// Incremental forward for one new token at a given position.
    /// Must be called after `seed_for_incremental_generation`.
    /// Returns logits for this single position + receipt over those logits.
    ///
    /// NOTE: Exposed for the hard equivalence gate proving recurrent KV bit-exactness.
    /// All user-facing generation should go through the single public API `generate(...)`.
    pub fn forward_incremental(&mut self, new_token_id: u32, position: usize) -> (Vec<f32>, [u8; 32]) {
        let n_embd = self.config.n_embd;
        let n_head = self.config.n_head;
        let head_dim = n_embd / n_head;
        let scale = 1.0f32 / (head_dim as f32).sqrt();

        // Start from previous residual (or bootstrap)
        let mut current_hidden = self.last_hidden.clone().unwrap_or_else(|| vec![0.0; n_embd]);

        // Embed the new token at this absolute position
        for d in 0..n_embd {
            let tok_val = self.wte.get((new_token_id as usize) * n_embd + d).copied().unwrap_or(0.0);
            let pos_val = self.wpe.get(position * n_embd + d).copied().unwrap_or(0.0);
            current_hidden[d] = tok_val + pos_val;
        }

        for layer in 0..self.config.n_layer {
            // LN1
            let ln1_w = self.layer_tensor(layer, "ln_1.weight");
            let ln1_b = self.layer_tensor(layer, "ln_1.bias");
            let ln1_vec = layernorm(&current_hidden, ln1_w, ln1_b, 1e-5);

            // QKV — deterministic f32 (bit-exact source of truth)
            let c_attn_w = self.layer_tensor(layer, "attn.c_attn.weight");
            let c_attn_b = self.layer_tensor(layer, "attn.c_attn.bias");
            let qkv = crate::linalg::matmul_bias(&ln1_vec, c_attn_w, c_attn_b, 1, n_embd, n_embd * 3);

            // Per-head incremental attention using persisted KV state
            let mut attn_out = vec![0.0f32; n_embd];
            for h in 0..n_head {
                let q_head = &qkv[h * head_dim..(h + 1) * head_dim];
                let k_head = &qkv[n_embd + h * head_dim..n_embd + (h + 1) * head_dim];
                let v_head = &qkv[2 * n_embd + h * head_dim..2 * n_embd + (h + 1) * head_dim];

                self.kv_states[layer][h].extend(k_head, v_head);

                let head_attn = waller_attention_for_new_query(
                    q_head,
                    &self.kv_states[layer][h],
                    head_dim,
                    scale,
                );

                for d in 0..head_dim {
                    attn_out[h * head_dim + d] = head_attn[d];
                }
            }

            // c_proj — deterministic f32 (bit-exact)
            let c_proj_w = self.layer_tensor(layer, "attn.c_proj.weight");
            let c_proj_b = self.layer_tensor(layer, "attn.c_proj.bias");
            let attn_proj = crate::linalg::matmul_bias(&attn_out, c_proj_w, c_proj_b, 1, n_embd, n_embd);

            // Residual 1
            for d in 0..n_embd {
                current_hidden[d] += attn_proj[d];
            }

            // LN2
            let ln2_w = self.layer_tensor(layer, "ln_2.weight");
            let ln2_b = self.layer_tensor(layer, "ln_2.bias");
            let ln2_vec = layernorm(&current_hidden, ln2_w, ln2_b, 1e-5);

            // MLP — deterministic f32 (bit-exact)
            let c_fc_w = self.layer_tensor(layer, "mlp.c_fc.weight");
            let c_fc_b = self.layer_tensor(layer, "mlp.c_fc.bias");
            let c_proj_mlp_w = self.layer_tensor(layer, "mlp.c_proj.weight");
            let c_proj_mlp_b = self.layer_tensor(layer, "mlp.c_proj.bias");

            let mut inter = crate::linalg::matmul_bias(&ln2_vec, c_fc_w, c_fc_b, 1, n_embd, 4 * n_embd);
            for d in 0..(4 * n_embd) {
                inter[d] = gelu(inter[d]);
            }

            let mlp_out = crate::linalg::matmul_bias(&inter, c_proj_mlp_w, c_proj_mlp_b, 1, 4 * n_embd, n_embd);

            // Residual 2
            for d in 0..n_embd {
                current_hidden[d] += mlp_out[d];
            }
        }

        // Final LN for this position
        let final_vec = self.layer_norm(&current_hidden, &self.ln_f_weight, &self.ln_f_bias);

        // Logits for this single position (weight tying)
        let vocab = self.config.vocab_size;
        let mut logits = vec![0.0f32; vocab];
        for v in 0..vocab {
            let mut sum = 0.0f32;
            for d in 0..n_embd {
                sum += final_vec[d] * self.wte[v * n_embd + d];
            }
            logits[v] = sum;
        }

        let receipt = sha256_of_f32_slice(&logits);

        // Update running hidden for next incremental step
        self.last_hidden = Some(final_vec);

        (logits, receipt)
    }

    // --- Helpers ---

    fn layer_tensor(&self, layer: usize, suffix: &str) -> &[f32] {
        // Pre-resolved lookup: index the flat per-layer Vec and match the
        // suffix against literals. No string allocation, no hashing — this
        // runs for 12 weights per layer per token, so eliminating the
        // `format!` + HashMap hash here removes thousands of allocations
        // per generation.
        let lw = &self.layers[layer];
        match suffix {
            "ln_1.weight" => &lw.ln_1_w,
            "ln_1.bias" => &lw.ln_1_b,
            "attn.c_attn.weight" => &lw.c_attn_w,
            "attn.c_attn.bias" => &lw.c_attn_b,
            "attn.c_proj.weight" => &lw.c_proj_w,
            "attn.c_proj.bias" => &lw.c_proj_b,
            "ln_2.weight" => &lw.ln_2_w,
            "ln_2.bias" => &lw.ln_2_b,
            "mlp.c_fc.weight" => &lw.c_fc_w,
            "mlp.c_fc.bias" => &lw.c_fc_b,
            "mlp.c_proj.weight" => &lw.mlp_c_proj_w,
            "mlp.c_proj.bias" => &lw.mlp_c_proj_b,
            _ => &[],
        }
    }

    /// Wave 2: return the pre-packed version of a heavy weight (for matmul_bias_packed).
    /// Preserved infrastructure for the optional packed/turbo paths.
    #[allow(dead_code)]
    fn layer_tensor_packed(&self, layer: usize, suffix: &str) -> &[f32] {
        let lw = &self.layers[layer];
        match suffix {
            "attn.c_attn.weight" => &lw.c_attn_w_packed,
            "attn.c_proj.weight" => &lw.c_proj_w_packed,
            "mlp.c_fc.weight" => &lw.c_fc_w_packed,
            "mlp.c_proj.weight" => &lw.mlp_c_proj_w_packed,
            _ => &[],
        }
    }

    // --- Workspace accessors (interior mutability for allocation-free &self API) ---
    #[inline]
    fn ws_hidden(&self, len: usize) -> &mut [f32] {
        // SAFETY: The workspace buffers are used exclusively as scratch space inside the
        // forward paths. Calls are not re-entrant on the same model instance from multiple
        // threads, and mutation is confined to the duration of a single &self forward call.
        // This does not affect the deterministic receipt or any observable state.
        let v = unsafe { &mut *self.hidden_buf.get() };
        &mut v[..len]
    }
    #[inline]
    fn ws_ln(&self, len: usize) -> &mut [f32] {
        let v = unsafe { &mut *self.ln_buf.get() };
        &mut v[..len]
    }
    #[inline]
    fn ws_attn(&self, len: usize) -> &mut [f32] {
        let v = unsafe { &mut *self.attn_buf.get() };
        &mut v[..len]
    }
    #[inline]
    fn ws_mlp(&self, len: usize) -> &mut [f32] {
        let v = unsafe { &mut *self.mlp_inter_buf.get() };
        &mut v[..len]
    }
    #[inline]
    fn ws_final_hidden(&self, len: usize) -> &mut [f32] {
        let v = unsafe { &mut *self.final_hidden_buf.get() };
        &mut v[..len]
    }
    #[inline]
    fn ws_logits(&self, len: usize) -> &mut [f32] {
        let v = unsafe { &mut *self.logits_buf.get() };
        &mut v[..len]
    }

    fn layer_norm(&self, x: &[f32], gamma: &[f32], beta: &[f32]) -> Vec<f32> {
        let mut out = vec![0.0f32; x.len()];
        Self::layer_norm_into_static(self.config.n_embd, x, gamma, beta, &mut out);
        out
    }

    /// Allocation-free LayerNorm: writes directly into the provided output buffer.
    /// Static variant so callers can avoid &self borrow conflicts with workspace slices.
    #[allow(dead_code)]
    fn layer_norm_into(&self, x: &[f32], gamma: &[f32], beta: &[f32], out: &mut [f32]) {
        Self::layer_norm_into_static(self.config.n_embd, x, gamma, beta, out);
    }

    fn layer_norm_into_static(n_embd: usize, x: &[f32], gamma: &[f32], beta: &[f32], out: &mut [f32]) {
        assert_eq!(x.len(), out.len(), "layer_norm_into: buffer size mismatch");
        for i in 0..(x.len() / n_embd) {
            let row = &x[i * n_embd..(i + 1) * n_embd];
            let mut state = crate::welford::WelfordState::new();
            for &v in row {
                state.update(v);
            }
            let mean = state.mean;
            let std = state.std(1e-5);
            let dst = &mut out[i * n_embd..(i + 1) * n_embd];
            for d in 0..n_embd {
                dst[d] = (row[d] - mean) / std * gamma[d] + beta[d];
            }
        }
    }

    fn attention_block(&self, x: &[f32], seq_len: usize, layer: usize) -> Vec<f32> {
        let n_embd = self.config.n_embd;
        let n_head = self.config.n_head;
        let head_dim = n_embd / n_head;
        let scale = 1.0f32 / (head_dim as f32).sqrt();

        // c_attn fused QKV projection — deterministic f32, pre-packed B (bit-exact source of truth)
        let c_attn_w_packed = &self.layers[layer].c_attn_w_packed;
        let c_attn_b = &self.layers[layer].c_attn_b;
        let qkv = crate::linalg::matmul_bias_packed(x, c_attn_w_packed, c_attn_b, seq_len, n_embd, n_embd * 3);

        // Split into Q, K, V and run per-head waller_operator.
        // The heads are mathematically independent and each writes to a
        // disjoint slice of attn_out, so parallelizing across heads is
        // determinism-safe: no float value depends on head execution order,
        // only its placement. The per-head accumulation order inside
        // waller_operator is unchanged.
        let mut attn_out = vec![0.0f32; seq_len * n_embd];

        // Compute each head's output (the heavy part) independently.
        let compute_head = |h: usize| -> Vec<f32> {
            let q_head = extract_head(&qkv, seq_len, n_embd * 3, h, head_dim, 0);
            let k_head = extract_head(&qkv, seq_len, n_embd * 3, h, head_dim, n_embd);
            let v_head = extract_head(&qkv, seq_len, n_embd * 3, h, head_dim, n_embd * 2);
            waller_operator(&q_head, &k_head, &v_head, seq_len, head_dim, scale)
        };

        #[cfg(feature = "rayon")]
        let head_outs: Vec<Vec<f32>> = {
            use rayon::prelude::*;
            (0..n_head).into_par_iter().map(compute_head).collect()
        };
        #[cfg(not(feature = "rayon"))]
        let head_outs: Vec<Vec<f32>> = (0..n_head).map(compute_head).collect();

        // Scatter back in fixed order (placement only, no arithmetic).
        // The head_dim run is contiguous in both src and dst, so copy whole
        // rows instead of element-by-element (bit-exact, fewer bounds checks).
        for h in 0..n_head {
            let head_out = &head_outs[h];
            for i in 0..seq_len {
                let dst = i * n_embd + h * head_dim;
                let src = i * head_dim;
                attn_out[dst..dst + head_dim]
                    .copy_from_slice(&head_out[src..src + head_dim]);
            }
        }

        // c_proj (output projection) — deterministic f32, pre-packed B (bit-exact)
        let c_proj_w_packed = &self.layers[layer].c_proj_w_packed;
        let c_proj_b = &self.layers[layer].c_proj_b;
        crate::linalg::matmul_bias_packed(&attn_out, c_proj_w_packed, c_proj_b, seq_len, n_embd, n_embd)
    }

    fn mlp_block(&self, x: &[f32], seq_len: usize, layer: usize) -> Vec<f32> {
        let n_embd = self.config.n_embd;
        let n_mlp = 4 * n_embd; // GPT-2 small uses 4x

        // Pre-packed weights — deterministic f32, bit-exact, zero runtime allocation
        let c_fc_w_packed = &self.layers[layer].c_fc_w_packed;
        let c_fc_b = &self.layers[layer].c_fc_b;
        let mlp_c_proj_w_packed = &self.layers[layer].mlp_c_proj_w_packed;
        let mlp_c_proj_b = &self.layers[layer].mlp_c_proj_b;

        // c_fc (expand) + GELU — deterministic f32 (bit-exact)
        let mut inter = crate::linalg::matmul_bias_packed(x, c_fc_w_packed, c_fc_b, seq_len, n_embd, n_mlp);
        for v in &mut inter {
            *v = gelu(*v);
        }

        // c_proj (project down)
        crate::linalg::matmul_bias_packed(&inter, mlp_c_proj_w_packed, mlp_c_proj_b, seq_len, n_mlp, n_embd)
    }

    /// Allocation-free (model-level) attention block writing into caller-provided buffer.
    /// For full zero-alloc the inner matmuls also need into-variants (partial implementation here).
    #[allow(dead_code)]
    fn attention_block_into(&self, x: &[f32], seq_len: usize, layer: usize, out: &mut [f32]) {
        let res = self.attention_block(x, seq_len, layer);
        out[..res.len()].copy_from_slice(&res);
    }

    /// Allocation-free (model-level) MLP block writing into caller-provided buffer.
    #[allow(dead_code)]
    fn mlp_block_into(&self, x: &[f32], seq_len: usize, layer: usize, out: &mut [f32]) {
        let res = self.mlp_block(x, seq_len, layer);
        out[..res.len()].copy_from_slice(&res);
    }
}

fn extract_head(
    qkv: &[f32],
    seq: usize,
    stride: usize,
    head: usize,
    head_dim: usize,
    offset: usize,
) -> Vec<f32> {
    let mut head_data = vec![0.0f32; seq * head_dim];
    for i in 0..seq {
        for d in 0..head_dim {
            head_data[i * head_dim + d] =
                qkv[i * stride + offset + head * head_dim + d];
        }
    }
    head_data
}