use attention_transformer::config::Config;
use attention_transformer::wnsm_transformer::{
    format_receipt, sha256_of_f32_slice, EnergyReport, WNSM_GAE_Decoder,
};

pub fn run_demo() {
    println!();
    println!("  Waller Geodesic Attention / ATE Demo");
    println!("  ============================================================");

    let hidden = 64usize;
    let heads = 4usize;
    let layers = 3usize;
    let seq = 8usize;
    let mlp = hidden * 4;
    let cfg = Config::new(hidden, heads, mlp, seq);
    let mut model = WNSM_GAE_Decoder::new(cfg, layers);

    let input: Vec<f32> = (0..seq * hidden)
        .map(|i| ((i as f32) * 0.017).sin() * 0.2)
        .collect();

    let normal = model.forward(input.clone(), seq);
    let normal_receipt = sha256_of_f32_slice(&normal);

    let (wnsm, _) = model.forward_wnsm_chained(input, seq, None);
    let wnsm_receipt = sha256_of_f32_slice(&wnsm);

    let max_diff: f32 = normal
        .iter()
        .zip(wnsm.iter())
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f32, f32::max);

    println!("  NORMAL receipt: {}", format_receipt(&normal_receipt));
    println!("  WNSM   receipt: {}", format_receipt(&wnsm_receipt));
    println!("  Max output diff: {:.2e}", max_diff);

    let energy = EnergyReport::compute(seq, hidden, mlp, layers, hidden, true);
    println!("  Payload bytes avoided: {}", energy.wnsm_payload_bytes_avoided);
    println!(
        "  Est. joules saved: {:.2e}",
        energy.estimated_joules_saved_vs_standard
    );

    if format_receipt(&normal_receipt)
        == "e1980a6fa77252dcab86e48aa7aa8ab2a6d3c5639789d0917e7efa1a7bb37628"
    {
        println!("  Reference receipt match: YES");
    } else {
        println!("  Reference receipt match: check platform build");
    }

    println!("  ============================================================");
    println!();
}