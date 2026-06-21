//! Live P&L proof scaffold: receipt-verified backtest vs momentum baseline.
//!
//!   cargo run --release --example receipt_backtest
//!   cargo run --release --example receipt_backtest -- path/to/desk_signals.csv

use attention_transformer::quant::{load_signals_csv, BacktestComparison};

fn main() {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| concat!(env!("CARGO_MANIFEST_DIR"), "/testdata/quant_signals.csv").to_string());

    println!("Loading signals: {path}");
    let bars = load_signals_csv(&path).expect("signals csv");
    let cmp = BacktestComparison::run(&bars);
    cmp.print_report();

    println!("\nEvery LuxiEdge bar stores:");
    println!("  - score_receipt (SHA-256 of alpha score f32)");
    println!("  - AuditReport (config_hash + receipt + notes)");
    println!("\nNext: wire Gpt2Model / desk alpha head; ingest live CSV from production.");
}