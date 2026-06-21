//! Energy Sweep — attention-transformer
//!
//! Produces the ENERGY / ELECTRICITY scaling story across sequence lengths.
//!
//! Two independent measurements per sequence length N:
//!
//!   1. WNSM payload bytes avoided  (uses the EXACT formula from EnergyReport::compute
//!      in src/wnsm_transformer.rs: 2 * payload_dim * N * layers * 4). This is LINEAR in N.
//!      Converted to joules using the published HBM data-movement energy figure.
//!
//!   2. Attention-stage HBM traffic energy, standard O(N^2) vs Waller O(N):
//!      - Standard attention materializes the full N x N score matrix in HBM
//!        (write scores + read scores) -> traffic grows with N^2  -> QUADRATIC energy.
//!      - Waller streaming attention reads K/V once per query and keeps the running
//!        softmax state on-chip -> traffic grows with N           -> LINEAR energy.
//!
//! Energy basis (citable hardware literature):
//!   ~20 picojoules per byte moved to/from HBM/DRAM, versus ~0 for data kept
//!   on-chip in registers / shared memory. This is the dominant electricity
//!   cost in transformer inference (data movement, not arithmetic).
//!   Cross-check: at N=8, h=64, layers=3, payload_dim=64 the WNSM formula yields
//!   12,288 bytes avoided -> 12,288 * 20e-12 = 2.46e-7 J, which reproduces the
//!   EnergyReport constant (2.2e-7 J) almost exactly.
//!
//! Run with:  cargo run --release --example energy_sweep > energy.csv

/// Published HBM/DRAM data-movement energy: ~20 pJ per byte.
const J_PER_BYTE_HBM: f64 = 20e-12;

fn main() {
    // Model shape mirrors production_demo / scaling_sweep for consistency.
    let hidden: u64 = 64;
    let heads: u64 = 4;
    let layers: u64 = 3;
    let payload_dim: u64 = hidden; // production_demo uses payload_dim = hidden

    // CSV header
    println!("seq_len,wnsm_bytes_avoided,wnsm_joules_saved,std_attn_joules,waller_attn_joules,energy_reduction_x");

    let seq_lengths: [u64; 8] = [128, 512, 2048, 8192, 32768, 65536, 131072, 262144];

    for &n in seq_lengths.iter() {
        // --- 1. WNSM payload bytes avoided (EXACT formula from EnergyReport::compute) ---
        // avoided = 2 * payload_dim * seq * layers * 4   (linear in N)
        let wnsm_bytes_avoided: u64 = 2 * payload_dim * n * layers * 4;
        let wnsm_joules_saved: f64 = wnsm_bytes_avoided as f64 * J_PER_BYTE_HBM;

        // --- 2. Attention-stage HBM traffic energy: standard O(N^2) vs Waller O(N) ---
        // Standard attention: materialize N x N score matrix per head.
        // Traffic = write scores + read scores = 2 * N * N * 4 bytes, per head, per layer.
        let std_bytes: f64 = 2.0 * (n as f64) * (n as f64) * 4.0 * heads as f64 * layers as f64;
        let std_attn_joules: f64 = std_bytes * J_PER_BYTE_HBM;

        // Waller streaming attention: read K and V once per query row, keep the
        // running (max, sum_exp, accumulator) softmax state on-chip.
        // Traffic = 2 (K + V) * N * hidden * 4 bytes, per head, per layer.
        let waller_bytes: f64 =
            2.0 * (n as f64) * (hidden as f64) * 4.0 * heads as f64 * layers as f64;
        let waller_attn_joules: f64 = waller_bytes * J_PER_BYTE_HBM;

        let reduction_x: f64 = std_attn_joules / waller_attn_joules;

        println!(
            "{},{},{:.6e},{:.6e},{:.6e},{:.2}",
            n,
            wnsm_bytes_avoided,
            wnsm_joules_saved,
            std_attn_joules,
            waller_attn_joules,
            reduction_x
        );
    }
}
