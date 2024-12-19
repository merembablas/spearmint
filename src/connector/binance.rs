use crate::model;
use crate::model::result;
use binance::account;
use binance::api::*;
use binance::general;

pub const PLATFORM: &str = "binance";

pub struct Connector {
    account: account::Account,
    general: general::General,
}

impl Connector {
    pub fn from_credential(api_key: String, api_secret: String) -> Self {
        Self {
            account: Binance::new(Some(api_key.clone()), Some(api_secret.clone())),
            general: Binance::new(Some(api_key), Some(api_secret)),
        }
    }
}

impl model::Exchange for Connector {
    fn get_balances(&self) -> Vec<result::Balance> {
        let account_information = self.account.get_account().unwrap();
        let mut balances: Vec<result::Balance> = Vec::new();

        for b in account_information.balances.iter() {
            if b.free.parse::<f32>().unwrap() == 0.0 {
                continue;
            }
            balances.push(result::Balance {
                asset: b.asset.clone(),
                free: b.free.parse::<f64>().unwrap(),
            });
        }

        balances
    }

    fn market_buy_using_quote_quantity(&self, pair: String, qty: f64) -> result::Transaction {
        let transaction = self
            .account
            .market_buy_using_quote_quantity(pair, qty)
            .unwrap();

        let price: f64 = match transaction.fills {
            None => 0.0,
            Some(fills) => {
                let sum: f64 = fills.iter().map(|x| x.price).sum();
                sum / fills.len() as f64
            }
        };

        result::Transaction {
            order_id: transaction.order_id,
            pair: transaction.symbol,
            price: price,
            qty: transaction.executed_qty,
            platform: String::from(PLATFORM),
            timestamp: transaction.transact_time,
        }
    }

    fn market_sell(&self, pair: String, qty: f64) -> result::Transaction {
        let transaction = self.account.market_sell(pair, qty).unwrap();

        let price: f64 = match transaction.fills {
            None => 0.0,
            Some(fills) => {
                let sum: f64 = fills.iter().map(|x| x.price).sum();
                sum / fills.len() as f64
            }
        };

        result::Transaction {
            order_id: transaction.order_id,
            pair: transaction.symbol,
            price: price,
            qty: transaction.executed_qty,
            platform: String::from(PLATFORM),
            timestamp: transaction.transact_time,
        }
    }

    fn get_balance(&self, asset: String) -> result::Balance {
        let balance = self.account.get_balance(asset).unwrap();

        result::Balance {
            asset: balance.asset,
            free: balance.free.parse::<f64>().unwrap(),
        }
    }

    fn adjust_quantity(&self, pair: String, qty: f64) -> f64 {
        let exchange_info = self.general.exchange_info().unwrap();

        let symbol_info = exchange_info
            .symbols
            .into_iter()
            .find(|s| s.symbol == pair)
            .expect("Pair not found");

        let lot_size_filter = symbol_info
            .filters
            .iter()
            .find_map(|filter| match filter {
                binance::model::Filters::LotSize {
                    min_qty, step_size, ..
                } => Some((min_qty, step_size)),
                _ => None,
            })
            .expect("Lot size filter not found");

        let step_size: f64 = lot_size_filter.1.parse().unwrap();
        let precision = step_size.log10().abs().round() as u32;

        let adjusted_qty =
            (qty * 10f64.powi(precision as i32)).floor() / 10f64.powi(precision as i32);

        adjusted_qty
    }
}
