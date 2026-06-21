//! Receipt-verified backtest: LuxiEdge scorer vs baseline, P&L with per-bar receipts.

use crate::npow::{run_scaling_samples, NpowPayload, NPOW_PAYLOAD_DIM};
use crate::quant::signal::QuantSignalBar;
use crate::wnsm_transformer::{format_receipt, sha256_of_f32_slice, WNSM_GAE_Decoder};
use crate::{AuditReport, Config};

/// Strategy lane for comparison.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StrategyKind {
    /// LuxiEdge WNSM/GAE decoder scores → positions.
    LuxiEdge,
    /// Sign of first feature (momentum proxy).
    MomentumBaseline,
    /// Flat / no trade.
    HoldBaseline,
}

#[derive(Clone, Debug)]
pub struct BacktestConfig {
    pub strategy: StrategyKind,
    pub position_scale: f32,
    pub score_threshold: f32,
    pub notes: String,
}

impl Default for BacktestConfig {
    fn default() -> Self {
        Self {
            strategy: StrategyKind::LuxiEdge,
            position_scale: 1.0,
            score_threshold: 0.0,
            notes: String::new(),
        }
    }
}

/// Per-bar audit + P&L attribution.
#[derive(Clone, Debug)]
pub struct PnLBarRecord {
    pub bar_index: usize,
    pub timestamp: String,
    pub symbol: String,
    pub position: f32,
    pub forward_return: f32,
    pub bar_pnl: f32,
    pub cumulative_pnl: f32,
    pub score_receipt: [u8; 32],
    pub audit: AuditReport,
}

/// Full backtest output.
#[derive(Clone, Debug)]
pub struct ReceiptVerifiedBacktest {
    pub config: BacktestConfig,
    pub bars: Vec<PnLBarRecord>,
    pub total_pnl: f32,
    pub config_hash: [u8; 32],
    pub run_receipt: [u8; 32],
}

/// Side-by-side LuxiEdge vs baseline.
#[derive(Clone, Debug)]
pub struct BacktestComparison {
    pub luxi: ReceiptVerifiedBacktest,
    pub baseline: ReceiptVerifiedBacktest,
    pub edge_bps: f32,
}

impl ReceiptVerifiedBacktest {
    pub fn run(bars: &[QuantSignalBar], cfg: BacktestConfig) -> Self {
        let layer_cfg = Config::new(64, 4, 256, 128);
        let mut decoder = WNSM_GAE_Decoder::new(layer_cfg.clone(), 2);
        decoder.install_npow_wnsm(NPOW_PAYLOAD_DIM);
        let scaling_samples = run_scaling_samples(64);
        let npow_anchor = scaling_samples.last().expect("npow anchor");
        let npow = NpowPayload::from_samples(&scaling_samples, npow_anchor);
        let npow_line = npow.summary_line();
        let config_hash = sha256_of_f32_slice(&[layer_cfg.hidden_dim as f32]);

        let mut records = Vec::with_capacity(bars.len());
        let mut cumulative = 0.0f32;
        let mut pnl_series = Vec::with_capacity(bars.len());

        for (i, bar) in bars.iter().enumerate() {
            let (score_vec, position) = match cfg.strategy {
                StrategyKind::LuxiEdge => luxiedge_score(&mut decoder, bar, cfg.score_threshold),
                StrategyKind::MomentumBaseline => {
                    let mom = bar.features.first().copied().unwrap_or(0.0);
                    let pos = if mom > cfg.score_threshold {
                        cfg.position_scale
                    } else if mom < -cfg.score_threshold {
                        -cfg.position_scale
                    } else {
                        0.0
                    };
                    (vec![mom], pos)
                }
                StrategyKind::HoldBaseline => (vec![0.0], 0.0),
            };

            let score_receipt = sha256_of_f32_slice(&score_vec);
            let bar_pnl = position * bar.forward_return;
            cumulative += bar_pnl;
            pnl_series.push(cumulative);

            let notes = format!(
                "{} | bar={} | pos={:.4} | receipt={} | {}",
                cfg.notes,
                i,
                position,
                format_receipt(&score_receipt),
                if i == 0 { npow_line.as_str() } else { "" }
            );
            let audit = AuditReport::new(config_hash, score_receipt, notes);

            records.push(PnLBarRecord {
                bar_index: i,
                timestamp: bar.timestamp.clone(),
                symbol: bar.symbol.clone(),
                position,
                forward_return: bar.forward_return,
                bar_pnl,
                cumulative_pnl: cumulative,
                score_receipt,
                audit,
            });
        }

        let run_receipt = sha256_of_f32_slice(&pnl_series);

        Self {
            config: cfg,
            bars: records,
            total_pnl: cumulative,
            config_hash,
            run_receipt,
        }
    }

    pub fn print_summary(&self, label: &str) {
        println!("── {label} ──");
        println!("  strategy     : {:?}", self.config.strategy);
        println!("  bars         : {}", self.bars.len());
        println!("  total P&L    : {:.6}", self.total_pnl);
        println!("  run receipt  : {}", format_receipt(&self.run_receipt));
        if let Some(first) = self.bars.first() {
            println!(
                "  first bar    : {} {} receipt={}",
                first.timestamp,
                first.symbol,
                format_receipt(&first.score_receipt)
            );
        }
    }
}

impl BacktestComparison {
    pub fn run(bars: &[QuantSignalBar]) -> Self {
        let luxi = ReceiptVerifiedBacktest::run(
            bars,
            BacktestConfig {
                strategy: StrategyKind::LuxiEdge,
                position_scale: 1.0,
                score_threshold: 0.02,
                notes: "LuxiEdge=WNSM_GAE_Decoder".into(),
            },
        );
        let baseline = ReceiptVerifiedBacktest::run(
            bars,
            BacktestConfig {
                strategy: StrategyKind::MomentumBaseline,
                position_scale: 1.0,
                score_threshold: 0.0,
                notes: "baseline=momentum_f0".into(),
            },
        );
        let edge_bps = (luxi.total_pnl - baseline.total_pnl) * 10_000.0;
        Self {
            luxi,
            baseline,
            edge_bps,
        }
    }

    pub fn print_report(&self) {
        println!("═══════════════════════════════════════════════════════════");
        println!(" Receipt-verified backtest — LuxiEdge vs baseline");
        println!("═══════════════════════════════════════════════════════════");
        self.luxi.print_summary("LuxiEdge (receipt per bar)");
        self.baseline.print_summary("Momentum baseline");
        println!("── Edge ──");
        println!("  P&L edge (bps notional): {:.2}", self.edge_bps);
        println!(
            "  LuxiEdge wins: {}",
            self.luxi.total_pnl > self.baseline.total_pnl
        );
        println!("═══════════════════════════════════════════════════════════");
    }
}

/// Map signal features → decoder input → scalar alpha score → position.
fn luxiedge_score(
    decoder: &mut WNSM_GAE_Decoder,
    bar: &QuantSignalBar,
    threshold: f32,
) -> (Vec<f32>, f32) {
    let h = decoder.layers[0].config.hidden_dim;
    let seq = 1usize;
    let mut x = vec![0.0f32; h];
    for (i, &f) in bar.features.iter().enumerate() {
        if i < h {
            x[i] = f;
        }
    }
    let out = decoder.forward(x, seq);
    let score = out.iter().take(8).sum::<f32>() / 8.0f32;
    let position = if score > threshold {
        1.0
    } else if score < -threshold {
        -1.0
    } else {
        0.0
    };
    (vec![score], position)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quant::signal::load_signals_csv;

    #[test]
    fn backtest_produces_receipts_and_comparison() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/testdata/quant_signals.csv");
        let bars = load_signals_csv(path).unwrap();
        let cmp = BacktestComparison::run(&bars);
        assert!(!cmp.luxi.bars.is_empty());
        assert_ne!(cmp.luxi.run_receipt, [0u8; 32]);
        for bar in &cmp.luxi.bars {
            assert_ne!(bar.score_receipt, [0u8; 32]);
            assert!(bar.audit.verify(&bar.score_receipt));
        }
    }
}