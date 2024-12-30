use crate::model;
use crate::model::result;
use binance::account;
use binance::api::*;
use binance::general;
use binance::market;
use binance::model::TradeHistory;
use chrono::{Duration, Utc};

pub const PLATFORM: &str = "binance";

pub struct Connector {
    account: account::Account,
    general: general::General,
    market: market::Market,
}

impl Connector {
    pub fn from_credential(api_key: String, api_secret: String) -> Self {
        Self {
            account: Binance::new(Some(api_key.clone()), Some(api_secret.clone())),
            general: Binance::new(Some(api_key.clone()), Some(api_secret.clone())),
            market: Binance::new(Some(api_key), Some(api_secret)),
        }
    }

    pub fn get_today_pnl(&self) {
        let now = Utc::now();
        let twenty_four_hours_ago = now - Duration::hours(24);

        let start_ms = twenty_four_hours_ago.timestamp_millis() as u64;
        let today_trades = self.get_trades("PENGUUSDT");

        let bnb_usdt_price = self.get_price("BNBUSDT").unwrap_or(0.0);
        println!("Current BNB/USDT price: {:.2}", bnb_usdt_price);

        // Calculate PnL
        if let Some(trades) = today_trades {
            let today_pnl = self.calculate_pnl(&trades, bnb_usdt_price, start_ms);
            println!("Today's PnL: {:.2} USDT", today_pnl);
        } else {
            println!("No trades found for today.");
        }
    }

    pub fn get_trades(&self, pair: &str) -> Option<Vec<TradeHistory>> {
        match self.account.trade_history(pair) {
            Ok(trades) => Some(trades),
            Err(err) => {
                println!("Error fetching trade history: {:?}", err);
                None
            }
        }
    }

    pub fn get_price(&self, pair: &str) -> Option<f64> {
        match self.market.get_price(pair) {
            Ok(price) => Some(price.price),
            Err(_) => None,
        }
    }

    fn calculate_pnl(&self, trades: &[TradeHistory], comm_price: f64, start: u64) -> f64 {
        let mut total_buy = 0.0;
        let mut pnl = 0.0;

        for trade in trades.iter() {
            if start > trade.time || (!trade.is_buyer && total_buy == 0.0) {
                continue;
            }
            println!("{:?}", trade);
            let commission = trade.commission.parse::<f64>().unwrap_or(0.0);

            if trade.is_buyer {
                total_buy += (trade.qty * trade.price) + (commission * comm_price);
            } else {
                pnl += (trade.qty * trade.price) - (commission * comm_price) - total_buy;
                total_buy = 0.0;
            }
        }

        pnl
    }

    pub fn fetch_server_time(&self) {
        match self.general.get_server_time() {
            Ok(server_time) => println!("Binance Server Time: {}", server_time.server_time),
            Err(err) => println!("Error fetching server time: {:?}", err),
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
