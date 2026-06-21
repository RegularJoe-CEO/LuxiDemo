//! 70B+ scale: AUDIT receipt contract must not change with profile.

use attention_transformer::scaling::{
    audit_receipt_contract, gpt2_124m_profile, llama2_70b_profile, llama2_7b_profile,
    scaling_audit_contract_matches_gold,
};

#[test]
fn audit_contract_zero_tolerance() {
    let c = audit_receipt_contract();
    assert_eq!(c.max_diff_tolerance, 0.0);
    assert!(scaling_audit_contract_matches_gold());
}

#[test]
fn scale_profiles_monotonic_params() {
    let g = gpt2_124m_profile();
    let s7 = llama2_7b_profile();
    let s70 = llama2_70b_profile();
    assert!(s7.approx_params_b > g.approx_params_b);
    assert!(s70.approx_params_b >= 70.0);
    assert!(s70.estimated_weight_bytes() > s7.estimated_weight_bytes());
    assert_ne!(s70.config_digest(), g.config_digest());
}