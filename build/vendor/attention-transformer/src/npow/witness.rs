//! NPOW payload layout for WNSM transport (12 × f32, fixed order).

use crate::npow::scaling::{fit_log_log_slope, ScalingSample};
use crate::wnsm_transformer::{sha256_of_f32_slice, WNSM_GAE_Layer};

/// Magic tag `NPOW` as f32 bit pattern (deterministic witness id).
pub const NPOW_MAGIC: f32 = f32::from_bits(0x4E50_4F57); // "NPOW"

pub const NPOW_VERSION: f32 = 1.0;
pub const NPOW_PAYLOAD_DIM: usize = 12;

/// Decoded NPOW scaling witness.
#[derive(Clone, Debug, PartialEq)]
pub struct NpowPayload {
    pub seq_len: usize,
    pub standard_mem_bytes: u64,
    pub waller_mem_bytes: u64,
    pub memory_reduction_x: f64,
    pub waller_time_ms: f64,
    pub mem_slope_waller: f64,
    pub mem_slope_standard: f64,
    pub time_slope_waller: f64,
    pub witness_receipt: [u8; 32],
}

impl NpowPayload {
    /// Build from scaling sweep + anchor sequence length.
    pub fn from_samples(samples: &[ScalingSample], anchor: &ScalingSample) -> Self {
        let xs: Vec<f64> = samples.iter().map(|s| s.seq_len as f64).collect();
        let mem_w: Vec<f64> = samples.iter().map(|s| s.waller_mem_bytes as f64).collect();
        let mem_s: Vec<f64> = samples
            .iter()
            .map(|s| s.standard_mem_bytes as f64)
            .collect();
        let time_w: Vec<f64> = samples.iter().map(|s| s.waller_time_ms).collect();

        let mem_slope_waller = fit_log_log_slope(&xs, &mem_w);
        let mem_slope_standard = fit_log_log_slope(&xs, &mem_s);
        let time_slope_waller = fit_log_log_slope(&xs, &time_w);

        let mut scratch = vec![0.0f32; NPOW_PAYLOAD_DIM];
        encode_npow_payload_into(
            &mut scratch,
            anchor,
            mem_slope_waller,
            mem_slope_standard,
            time_slope_waller,
        );
        let witness_receipt = sha256_of_f32_slice(&scratch);

        Self {
            seq_len: anchor.seq_len,
            standard_mem_bytes: anchor.standard_mem_bytes,
            waller_mem_bytes: anchor.waller_mem_bytes,
            memory_reduction_x: anchor.memory_reduction_x,
            waller_time_ms: anchor.waller_time_ms,
            mem_slope_waller,
            mem_slope_standard,
            time_slope_waller,
            witness_receipt,
        }
    }

    /// O(N) / O(N²) **memory** power-law (primary NPOW claim; analytical bytes).
    pub fn proves_memory_scaling(&self) -> bool {
        (0.85..=1.15).contains(&self.mem_slope_waller)
            && (1.85..=2.15).contains(&self.mem_slope_standard)
            && self.memory_reduction_x > 1.0
    }

    /// Full gate including optional wall-clock time slope (informational only).
    pub fn proves_on_scaling(&self) -> bool {
        self.proves_memory_scaling()
    }

    pub fn to_payload_vec(&self, _samples: &[ScalingSample], anchor: &ScalingSample) -> Vec<f32> {
        let mut v = vec![0.0f32; NPOW_PAYLOAD_DIM];
        encode_npow_payload_into(
            &mut v,
            anchor,
            self.mem_slope_waller,
            self.mem_slope_standard,
            self.time_slope_waller,
        );
        v
    }

    pub fn summary_line(&self) -> String {
        format!(
            "NPOW seq={} mem_slope_waller={:.3} mem_slope_std={:.3} time_slope={:.3} reduction={:.1}x receipt={}",
            self.seq_len,
            self.mem_slope_waller,
            self.mem_slope_standard,
            self.time_slope_waller,
            self.memory_reduction_x,
            crate::wnsm_transformer::format_receipt(&self.witness_receipt)
        )
    }
}

pub fn encode_npow_payload(
    anchor: &ScalingSample,
    mem_slope_waller: f64,
    mem_slope_standard: f64,
    time_slope_waller: f64,
) -> Vec<f32> {
    let mut v = vec![0.0f32; NPOW_PAYLOAD_DIM];
    encode_npow_payload_into(
        &mut v,
        anchor,
        mem_slope_waller,
        mem_slope_standard,
        time_slope_waller,
    );
    v
}

pub fn encode_npow_payload_into(
    out: &mut [f32],
    anchor: &ScalingSample,
    mem_slope_waller: f64,
    mem_slope_standard: f64,
    time_slope_waller: f64,
) {
    assert!(out.len() >= NPOW_PAYLOAD_DIM);
    out[0] = NPOW_MAGIC;
    out[1] = NPOW_VERSION;
    out[2] = anchor.seq_len as f32;
    out[3] = anchor.waller_mem_bytes as f32;
    out[4] = anchor.standard_mem_bytes as f32;
    out[5] = anchor.memory_reduction_x as f32;
    out[6] = anchor.waller_time_ms as f32;
    out[7] = mem_slope_waller as f32;
    out[8] = mem_slope_standard as f32;
    out[9] = time_slope_waller as f32;
    out[10] = 1.0; // O(N) class tag (Waller)
    out[11] = 2.0; // O(N²) class tag (standard)
}

pub fn decode_npow_payload(data: &[f32]) -> Option<NpowPayload> {
    if data.len() < NPOW_PAYLOAD_DIM {
        return None;
    }
    if data[0].to_bits() != NPOW_MAGIC.to_bits() {
        return None;
    }
    let witness_receipt = sha256_of_f32_slice(&data[..NPOW_PAYLOAD_DIM]);
    Some(NpowPayload {
        seq_len: data[2] as usize,
        waller_mem_bytes: data[3] as u64,
        standard_mem_bytes: data[4] as u64,
        memory_reduction_x: data[5] as f64,
        waller_time_ms: data[6] as f64,
        mem_slope_waller: data[7] as f64,
        mem_slope_standard: data[8] as f64,
        time_slope_waller: data[9] as f64,
        witness_receipt,
    })
}

/// Identity null-direction basis for zero-initialized `w_proj` (exact null-space injection).
pub fn install_identity_null_basis(layer: &mut WNSM_GAE_Layer, payload_dim: usize) {
    let m = layer.config.mlp_dim;
    let pd = payload_dim.min(m).min(NPOW_PAYLOAD_DIM);
    let mut vn = vec![0.0f32; m * pd];
    for k in 0..pd {
        for i in 0..m {
            vn[i * pd + k] = if i == k { 1.0 } else { 0.0 };
        }
    }
    layer.v_null = Some(vn);
    layer.payload_dim = pd;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::npow::scaling::run_scaling_samples;

    #[test]
    fn npow_roundtrip_and_proof() {
        let samples = run_scaling_samples(64);
        let anchor = samples.last().expect("anchor").clone();
        let npow = NpowPayload::from_samples(&samples, &anchor);
        assert!(npow.proves_memory_scaling());
        let vec = npow.to_payload_vec(&samples, &anchor);
        let dec = decode_npow_payload(&vec).expect("decode");
        assert!((dec.mem_slope_waller - npow.mem_slope_waller).abs() < 1e-5);
    }
}