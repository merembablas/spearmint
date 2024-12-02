#[derive(Debug, Clone)]
pub struct Bot {
    pub title: String,
    pub pair: String,
    pub platform: String,
    pub strategy: String,
    pub parameters: Parameters,
    pub margin: Margin,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct Parameters {
    pub cycle: String,
    pub first_buy_in: f64,
    pub take_profit_ratio: f64,
    pub earning_callback: f64,
}

#[derive(Debug, Clone)]
pub struct Margin {
    pub margin_configuration: Vec<Vec<f64>>,
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
