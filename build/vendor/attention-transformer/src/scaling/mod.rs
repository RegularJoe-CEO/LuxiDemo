//! Model scale profiles and AUDIT receipt contract for 70B+ deployment.

pub mod profile;

pub use profile::{
    audit_receipt_contract, gpt2_124m_profile, llama2_70b_profile, llama2_7b_profile,
    scaling_audit_contract_matches_gold, AuditReceiptContract, ModelScaleProfile, ShardPlan,
};