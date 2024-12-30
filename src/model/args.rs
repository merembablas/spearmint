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
    pub entry: OpenCriteria,
    pub take_profit: CloseCriteria,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Margin {
    pub margin_configuration: Vec<OpenCriteria>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenCriteria {
    pub mfi_below: f64,
    pub mfi_callback: f64,
    pub price_change_below: f64,
    pub price_callback: f64,
    pub amount_ratio: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CloseCriteria {
    pub price_change_above: f64,
    pub price_callback: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiCredential {
    pub api_key: String,
    pub secret_key: String,
    pub platform: String,
}
