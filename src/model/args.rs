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
    pub base: String,
    pub quote: String,
    pub platform: String,
    pub strategy: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Parameters {
    pub cycle: String,
    pub first_buy_in: f64,
    pub take_profit_ratio: f64,
    pub earning_callback: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Margin {
    pub margin_configuration: Vec<Vec<f64>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiCredential {
    pub api_key: String,
    pub secret_key: String,
    pub platform: String,
}
