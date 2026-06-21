use attention_transformer::wnsm_transformer::EnergyReport;

pub fn run_demo() {
    println!();
    println!("  Physics / Energy Demo");
    println!("  ============================================================");

    let configs = [
        ("Small edge (seq=8, hidden=64)", 8usize, 64usize, 256usize, 3usize),
        ("Medium (seq=32, hidden=256)", 32usize, 256usize, 1024usize, 6usize),
        ("Large profile (seq=128, hidden=768)", 128usize, 768usize, 3072usize, 12usize),
    ];

    for (label, seq, hidden, mlp, layers) in configs {
        let report = EnergyReport::compute(seq, hidden, mlp, layers, hidden, true);
        println!("  {}", label);
        println!(
            "    Payload avoided: {} bytes",
            report.wnsm_payload_bytes_avoided
        );
        println!(
            "    Joules saved (est): {:.4e}",
            report.estimated_joules_saved_vs_standard
        );
        println!("    {}", report.notes);
        println!();
    }

    println!("  Energy scales with transformer payload size.");
    println!("  WNSM moves side-channel data into null space.");
    println!("  Same math output, less data moved.");
    println!("  ============================================================");
    println!();
}