use super::model::{result, storage};
use super::model::{BotCommand, Exchange, Initial, Session, Strategy};
use std::marker::PhantomData;
use std::sync::Arc;

#[derive(Debug)]
pub struct BotInfo {
    pub platform: String,
    pub pair: String,
    pub base: String,
    pub quote: String,
}

#[derive(Debug)]
pub struct Bot<T: Exchange, S: Strategy> {
    pub info: Option<BotInfo>,
    strategy: Option<S>,
    connector: Option<T>,
}

impl<T: Exchange, S: Strategy> Bot<T, S> {
    pub fn update(&self, price: f64) {
        let connector = self.connector.as_ref().unwrap();
        let info = self.info.as_ref().unwrap();

        let trade = storage::get_latest_trade(&info.platform, &info.pair).unwrap();
        let state = storage::get_latest_state(&info.platform, &info.pair).unwrap();
        let avg_price = storage::get_avg_price(&info.platform, &info.pair, state.cycle);
        let mfi = storage::get_latest_mfi(&info.pair);
        let mfi_dir = if mfi[0] > mfi[1] {
            "UP".to_string()
        } else {
            "DOWN".to_string()
        };

        if state.id == 0 || trade.status == "CLOSE" {
            storage::create_trade(result::Trade {
                pair: info.pair.clone(),
                cycle: trade.cycle + 1,
                price: 0.0,
                qty: 0.0,
                platform: info.platform.clone(),
                status: String::from("WAIT"),
                timestamp: chrono::offset::Utc::now().timestamp() as u64,
            });

            storage::create_bot_state(result::BotState {
                id: 0,
                pair: info.pair.clone(),
                cycle: trade.cycle + 1,
                margin_position: 0,
                top_price: price,
                bottom_price: price,
                bottom_mfi: 80.0,
                platform: info.platform.clone(),
                timestamp: chrono::offset::Utc::now().timestamp() as u64,
            });

            return;
        }

        self.update_price_level(
            state.id,
            price,
            avg_price,
            state.top_price,
            state.bottom_price,
        );

        self.update_mfi_level(state.id, mfi[0], state.bottom_mfi);

        let session = Session::<Initial> {
            avg_price,
            top_price: state.top_price,
            bottom_price: state.bottom_price,
            status: trade.status,
            margin_position: state.margin_position,
            mfi: mfi[0],
            mfi_dir,
            bottom_mfi: state.bottom_mfi,
            phantom: PhantomData,
        };

        let command = self.strategy.as_ref().unwrap().run(price, session);

        match command {
            BotCommand::Pause() => println!("Waiting good signal"),
            BotCommand::Entry(amount) => {
                let transaction =
                    connector.market_buy_using_quote_quantity(info.pair.clone(), amount);

                storage::create_trade(result::Trade {
                    pair: info.pair.clone(),
                    cycle: trade.cycle,
                    price: transaction.price,
                    qty: transaction.qty,
                    platform: info.platform.clone(),
                    status: String::from("OPEN"),
                    timestamp: chrono::offset::Utc::now().timestamp() as u64,
                });

                let capital = connector.get_balance(info.quote.clone());
                storage::update_wallet(&info.quote, capital.free);

                println!("Entry signal {}", amount)
            }
            BotCommand::Buy(amount) => {
                let transaction =
                    connector.market_buy_using_quote_quantity(info.pair.clone(), amount);

                storage::create_trade(result::Trade {
                    pair: info.pair.clone(),
                    cycle: state.cycle,
                    price: transaction.price,
                    qty: transaction.qty,
                    platform: info.platform.clone(),
                    status: String::from("OPEN"),
                    timestamp: chrono::offset::Utc::now().timestamp() as u64,
                });

                storage::update_margin_position(state.id, state.margin_position + 1);
                storage::update_bottom_mfi(state.id, mfi[0]);

                let capital = connector.get_balance(info.quote.clone());
                storage::update_wallet(&info.quote, capital.free);

                println!("Buy signal {}", amount)
            }
            BotCommand::Sell() => {
                let qty = connector.get_balance(info.base.clone());
                let adj_qty = connector.adjust_quantity(info.pair.clone(), qty.free);
                let transaction = connector.market_sell(info.pair.clone(), adj_qty);

                storage::create_trade(result::Trade {
                    pair: info.pair.clone(),
                    cycle: state.cycle,
                    price: transaction.price,
                    qty: transaction.qty,
                    platform: info.platform.clone(),
                    status: String::from("CLOSE"),
                    timestamp: chrono::offset::Utc::now().timestamp() as u64,
                });

                let capital = connector.get_balance(info.quote.clone());
                storage::update_wallet(&info.quote, capital.free);

                println!("Sell signal {}: {}", &info.base, adj_qty);
            }
        };
    }

    fn update_price_level(
        &self,
        id: u64,
        price: f64,
        avg_price: f64,
        top_price: f64,
        bottom_price: f64,
    ) {
        if price > top_price {
            storage::update_top_price(id, price);
        } else if price < avg_price && top_price != avg_price {
            storage::update_top_price(id, avg_price);
        }

        if price < bottom_price {
            storage::update_bottom_price(id, price);
        } else if price > avg_price && bottom_price != avg_price {
            storage::update_bottom_price(id, avg_price);
        }
    }

    fn update_mfi_level(&self, id: u64, mfi: f64, bottom_mfi: f64) {
        if bottom_mfi > mfi || mfi > 50.0 {
            storage::update_bottom_mfi(id, mfi);
        }
    }
}

pub struct BotBuilder<T: Exchange, S: Strategy> {
    bot: Bot<T, S>,
}

impl<T: Exchange, S: Strategy> BotBuilder<T, S> {
    pub fn new() -> Self {
        Self {
            bot: Bot {
                info: None,
                strategy: None,
                connector: None,
            },
        }
    }

    pub fn build(self) -> Arc<Bot<impl Exchange, impl Strategy>> {
        Arc::new(self.bot)
    }

    pub fn with_info(self, info: BotInfo) -> Self {
        Self {
            bot: Bot {
                info: Some(info),
                ..self.bot
            },
        }
    }

    pub fn with_strategy(self, strategy: S) -> Self {
        Self {
            bot: Bot {
                strategy: Some(strategy),
                ..self.bot
            },
        }
    }

    pub fn with_connector(self, connector: T) -> Self {
        Self {
            bot: Bot {
                connector: Some(connector),
                ..self.bot
            },
        }
    }
}
