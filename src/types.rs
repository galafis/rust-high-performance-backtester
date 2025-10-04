use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub symbol: String,
    pub side: String,
    pub quantity: Decimal,
    pub price: Decimal,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestResult {
    pub total_return: Decimal,
    pub sharpe_ratio: Decimal,
    pub max_drawdown: Decimal,
    pub win_rate: Decimal,
}

pub trait Strategy: Send + Sync {
    fn name(&self) -> &str;
    fn on_bar(&mut self, price: Decimal) -> Option<String>;
}
