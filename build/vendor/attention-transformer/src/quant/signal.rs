//! Quant signal bars for receipt-verified backtests.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// One bar of market / feature data for systematic strategies.
#[derive(Clone, Debug, PartialEq)]
pub struct QuantSignalBar {
    pub timestamp: String,
    pub symbol: String,
    /// Feature vector (e.g. returns, vol, order-flow proxies).
    pub features: Vec<f32>,
    /// Realized forward return for P&L simulation (next-bar or labeled horizon).
    pub forward_return: f32,
}

/// Parse CSV: `timestamp,symbol,f0,f1,...,forward_return`
pub fn load_signals_csv(path: impl AsRef<Path>) -> Result<Vec<QuantSignalBar>, String> {
    let file = File::open(path.as_ref()).map_err(|e| format!("open csv: {e}"))?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let header = lines
        .next()
        .transpose()
        .map_err(|e| format!("read header: {e}"))?
        .ok_or("empty csv")?;
    let cols: Vec<&str> = header.split(',').map(str::trim).collect();
    if cols.len() < 4 || cols[0] != "timestamp" || cols[1] != "symbol" {
        return Err(format!(
            "expected header timestamp,symbol,f0,...,forward_return; got {header}"
        ));
    }
    let forward_idx = cols.len() - 1;
    let mut bars = Vec::new();
    for (line_no, line) in lines.enumerate() {
        let line = line.map_err(|e| format!("line {}: {e}", line_no + 2))?;
        if line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(',').map(str::trim).collect();
        if parts.len() != cols.len() {
            return Err(format!(
                "line {}: expected {} fields, got {}",
                line_no + 2,
                cols.len(),
                parts.len()
            ));
        }
        let mut features = Vec::with_capacity(forward_idx - 2);
        for i in 2..forward_idx {
            let v: f32 = parts[i]
                .parse()
                .map_err(|_| format!("line {}: bad f32 '{}'", line_no + 2, parts[i]))?;
            features.push(v);
        }
        let forward_return: f32 = parts[forward_idx]
            .parse()
            .map_err(|_| format!("line {}: bad forward_return", line_no + 2))?;
        bars.push(QuantSignalBar {
            timestamp: parts[0].to_string(),
            symbol: parts[1].to_string(),
            features,
            forward_return,
        });
    }
    Ok(bars)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_example_csv() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/testdata/quant_signals.csv");
        let bars = load_signals_csv(path).expect("csv");
        assert!(bars.len() >= 8);
        assert_eq!(bars[0].features.len(), 4);
    }
}