use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Bot {
    pub title: String,
    pub pair: String,
    pub base: String,
    pub quote: String,
    pub platform: String,
    pub strategy: String,
    pub parameters: Parameters,
    pub margin: Margin,
    pub status: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Parameters {
    pub cycle: String,
    pub first_buy_in: f64,
    pub entry: OpenCriteria,
    pub take_profit: CloseCriteria,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Margin {
    pub margin_configuration: Vec<OpenCriteria>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpenCriteria {
    pub mfi_below: f64,
    pub mfi_callback: f64,
    pub price_change_below: f64,
    pub price_callback: f64,
    pub amount_ratio: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CloseCriteria {
    pub price_change_above: f64,
    pub price_callback: f64,
}

#[derive(Debug, Default)]
pub struct Trade {
    pub pair: String,
    pub cycle: u64,
    pub price: f64,
    pub qty: f64,
    pub platform: String,
    pub status: String,
    pub timestamp: u64,
}

#[derive(Debug, Default)]
pub struct BotState {
    pub id: u64,
    pub pair: String,
    pub cycle: u64,
    pub margin_position: u64,
    pub top_price: f64,
    pub bottom_price: f64,
    pub bottom_mfi: f64,
    pub platform: String,
    pub timestamp: u64,
}

#[derive(Debug, Default)]
pub struct ApiCredential {
    pub api: String,
    pub secret: String,
    pub platform: String,
}

#[derive(Debug)]
pub struct Balance {
    pub asset: String,
    pub free: f64,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Transaction {
    pub order_id: u64,
    pub pair: String,
    pub price: f64,
    pub qty: f64,
    pub platform: String,
    pub timestamp: u64,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Order {
    pub pair: String,
    pub order_id: u64,
    pub price: f64,
    pub orig_qty: String,
    pub executed_qty: String,
}

#[derive(Debug, Default)]
pub struct PnL {
    pub pair: String,
    pub cycle: u64,
    pub pnl: f64,
}

#[derive(Debug, Default)]
pub struct Ticker {
    pub pair: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub mfi: f64,
}
