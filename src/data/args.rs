use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Kind {
    pub kind: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub title: String,
    pub general: General,
    pub parameters: Parameters,
    pub margin: Margin,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct General {
    pub pair: String,
    pub exchange: String,
    pub strategy: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Parameters {
    pub cycle: String,
    pub first_buy_in: f32,
    pub take_profit_ratio: f32,
    pub margin_call_limit: u32,
    pub earning_callback: f32,
    pub sub_position_start: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Margin {
    pub margin_configuration: Vec<Vec<f32>>,
    pub sub_position_profit_ratio: Vec<f32>,
    pub sub_position_earning_callback: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiKey {
    #[serde(default)]
    pub id: u64,
    pub api_key: String,
    pub secret_key: String,
    pub platform: String,
}
