//! Quant desk utilities: signal ingest, receipt-verified backtest, P&L vs baseline.

pub mod backtest;
pub mod signal;

pub use backtest::{
    BacktestComparison, BacktestConfig, PnLBarRecord, ReceiptVerifiedBacktest, StrategyKind,
};
pub use signal::{load_signals_csv, QuantSignalBar};