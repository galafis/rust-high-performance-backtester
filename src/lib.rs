//! High-Performance Backtesting Framework
pub mod engine;
pub mod strategy;
pub mod data;
pub mod execution;
pub mod metrics;
pub mod optimization;
pub mod visualization;
pub mod types;

pub use types::{Strategy, BacktestResult, Trade};
