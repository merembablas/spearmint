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
    pub first_buy_in: f32,
    pub take_profit_ratio: f32,
    pub earning_callback: f32,
}

#[derive(Debug, Clone)]
pub struct Margin {
    pub margin_configuration: Vec<Vec<f32>>,
}

#[derive(Debug, Default)]
pub struct Trade {
    pub pair: String,
    pub cycle: u64,
    pub price: f32,
    pub qty: f32,
    pub platform: String,
    pub status: String,
    pub timestamp: i64,
}

#[derive(Debug, Default)]
pub struct BotState {
    pub id: u64,
    pub pair: String,
    pub cycle: u64,
    pub margin_position: u64,
    pub top_price: f32,
    pub platform: String,
    pub timestamp: i64,
}
