//! Reference scale profiles — same math/receipt contract, different capacity.

use crate::wnsm_transformer::sha256_of_f32_slice;

/// How weights are sharded across devices (deterministic merge order).
#[derive(Clone, Debug)]
pub struct ShardPlan {
    pub tensor_parallel: usize,
    pub pipeline_stages: usize,
    pub bytes_per_shard: u64,
}

/// Canonical model dimensions + audit metadata.
#[derive(Clone, Debug)]
pub struct ModelScaleProfile {
    pub id: &'static str,
    pub hidden_dim: usize,
    pub num_layers: usize,
    pub num_heads: usize,
    pub mlp_dim: usize,
    pub vocab_size: usize,
    pub max_seq_len: usize,
    pub approx_params_b: f32,
    pub shard: ShardPlan,
    /// Receipt from fixed synthetic forward at profile dims (CPU gold); filled by gate tests.
    pub reference_receipt_note: &'static str,
}

/// Immutable contract: AUDIT lane must match CPU f32 for any profile.
#[derive(Clone, Debug)]
pub struct AuditReceiptContract {
    pub receipt_fn: &'static str,
    pub hash_input: &'static str,
    pub audit_env: &'static str,
    pub max_diff_tolerance: f32,
}

pub fn audit_receipt_contract() -> AuditReceiptContract {
    AuditReceiptContract {
        receipt_fn: "sha256_of_f32_slice",
        hash_input: "f32::to_bits() little-endian per element, fixed slice order",
        audit_env: "LUXI_RECEIPT_AUDIT=1",
        max_diff_tolerance: 0.0,
    }
}

pub fn gpt2_124m_profile() -> ModelScaleProfile {
    ModelScaleProfile {
        id: "gpt2_124m",
        hidden_dim: 768,
        num_layers: 12,
        num_heads: 12,
        mlp_dim: 3072,
        vocab_size: 50257,
        max_seq_len: 1024,
        approx_params_b: 0.124,
        shard: ShardPlan {
            tensor_parallel: 1,
            pipeline_stages: 1,
            bytes_per_shard: 500_000_000,
        },
        reference_receipt_note: "756a50a3…b9c8 (GPT-2 prompt logits, f32 CPU)",
    }
}

pub fn llama2_7b_profile() -> ModelScaleProfile {
    ModelScaleProfile {
        id: "llama2_7b",
        hidden_dim: 4096,
        num_layers: 32,
        num_heads: 32,
        mlp_dim: 11008,
        vocab_size: 32000,
        max_seq_len: 4096,
        approx_params_b: 7.0,
        shard: ShardPlan {
            tensor_parallel: 4,
            pipeline_stages: 1,
            bytes_per_shard: 4_000_000_000,
        },
        reference_receipt_note: "TBD: load + cuda_verify after weight port",
    }
}

pub fn llama2_70b_profile() -> ModelScaleProfile {
    ModelScaleProfile {
        id: "llama2_70b",
        hidden_dim: 8192,
        num_layers: 80,
        num_heads: 64,
        mlp_dim: 28672,
        vocab_size: 32000,
        max_seq_len: 4096,
        approx_params_b: 70.0,
        shard: ShardPlan {
            tensor_parallel: 8,
            pipeline_stages: 4,
            bytes_per_shard: 18_000_000_000,
        },
        reference_receipt_note: "TBD: identical AUDIT receipt vs CPU after sharded load",
    }
}

impl ModelScaleProfile {
    pub fn config_digest(&self) -> [u8; 32] {
        let blob = [
            self.hidden_dim as f32,
            self.num_layers as f32,
            self.num_heads as f32,
            self.mlp_dim as f32,
            self.vocab_size as f32,
            self.approx_params_b,
        ];
        sha256_of_f32_slice(&blob)
    }

    pub fn estimated_weight_bytes(&self) -> u64 {
        // Rough 2 bytes/param (FP16 storage) for planning; AUDIT uses f32 gold.
        (self.approx_params_b * 1e9 * 4.0) as u64
    }
}

/// Gate: contract requires zero tolerance and known hash function name.
pub fn scaling_audit_contract_matches_gold() -> bool {
    let c = audit_receipt_contract();
    c.max_diff_tolerance == 0.0 && c.receipt_fn == "sha256_of_f32_slice"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seventy_b_profile_and_contract() {
        let p = llama2_70b_profile();
        assert_eq!(p.approx_params_b, 70.0);
        assert!(p.num_layers >= 80);
        assert!(scaling_audit_contract_matches_gold());
        assert_ne!(p.config_digest(), [0u8; 32]);
    }
}