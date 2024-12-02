use crate::data::result;
use binance::account;
use binance::api::*;
use binance::general;
use binance::websockets::*;
use std::sync::atomic::AtomicBool;
use std::time::{Duration, Instant};

pub const PLATFORM: &str = "binance";

pub struct Account {
    pub api_key: String,
    pub api_secret: String,
}

impl Account {
    pub fn get_balances(&self) -> Vec<result::Balance> {
        let account: account::Account =
            Binance::new(Some(self.api_key.clone()), Some(self.api_secret.clone()));
        let account_information = account.get_account().unwrap();
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

    pub fn market_buy_using_quote_quantity(&self, pair: String, qty: f64) -> result::Transaction {
        let account: account::Account =
            Binance::new(Some(self.api_key.clone()), Some(self.api_secret.clone()));
        let transaction = account.market_buy_using_quote_quantity(pair, qty).unwrap();

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

    pub fn market_sell(&self, pair: String, qty: f64) -> result::Transaction {
        let account: account::Account =
            Binance::new(Some(self.api_key.clone()), Some(self.api_secret.clone()));
        let transaction = account.market_sell(pair, qty).unwrap();

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

    pub fn get_balance(&self, asset: String) -> result::Balance {
        let account: account::Account =
            Binance::new(Some(self.api_key.clone()), Some(self.api_secret.clone()));
        let balance = account.get_balance(asset).unwrap();

        result::Balance {
            asset: balance.asset,
            free: balance.free.parse::<f64>().unwrap(),
        }
    }

    pub fn adjust_quantity(&self, pair: String, qty: f64) -> f64 {
        let general: general::General =
            Binance::new(Some(self.api_key.clone()), Some(self.api_secret.clone()));
        let exchange_info = general.exchange_info().unwrap();

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

    #[allow(dead_code)]
    pub fn order_status(&self, pair: String, order_id: u64) -> result::Order {
        let account: account::Account =
            Binance::new(Some(self.api_key.clone()), Some(self.api_secret.clone()));
        let order = account.order_status(pair, order_id).unwrap();

        result::Order {
            pair: order.symbol,
            order_id: order.order_id,
            price: order.price,
            orig_qty: order.orig_qty,
            executed_qty: order.executed_qty,
        }
    }
}

#[allow(dead_code)]
pub fn ticks(account: Account, bots: Vec<result::Bot>) {
    let mut endpoints: Vec<String> = Vec::new();
    for bot in bots.iter() {
        endpoints.push(format!("{}@ticker", bot.pair.to_lowercase()));
    }

    let keep_running = AtomicBool::new(true);
    let mut last_block_time = Instant::now();
    let block_interval = Duration::from_secs(30);

    let mut web_socket: WebSockets<'_> = WebSockets::new(|event: WebsocketEvent| {
        if let WebsocketEvent::DayTicker(ticker_event) = event {
            println!("{:?}", ticker_event);

            let bot = bots
                .iter()
                .find(|&x| x.pair == ticker_event.symbol)
                .unwrap();

            if last_block_time.elapsed() >= block_interval {
                println!("Execute strategy...");
                println!("bot {:?}", bot);

                let balances = account.get_balances();
                println!("balances {:?}", balances);

                last_block_time = Instant::now(); // Reset the timer
            }
        }

        Ok(())
    });

    web_socket.connect_multiple_streams(&endpoints).unwrap();

    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Error: {:?}", e);
    }

    web_socket.disconnect().unwrap();
}
