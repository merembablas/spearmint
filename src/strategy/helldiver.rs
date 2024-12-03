use crate::connector::binance::Account;
use crate::data::{action, bind, result};

pub fn execute_strategy(bot: &result::Bot, price: &str) {
    let api_credential = bind::get(&bot.platform);
    let account = Account {
        api_key: api_credential.api,
        api_secret: api_credential.secret,
    };

    let trade = match action::get_latest_trade(&bot.platform, &bot.pair) {
        Ok(trade) => trade,
        Err(_error) => Default::default(),
    };

    let fprice = price.parse::<f64>().unwrap();

    if trade.status == "OPEN" {
        let state = match action::get_latest_state(&bot.platform, &bot.pair) {
            Ok(state) => state,
            Err(_error) => Default::default(),
        };

        if state.id == 0 {
            return;
        }

        let avg_price = action::get_avg_price(&bot.platform, &bot.pair, state.cycle);
        let avg_percent_change = action::calculate_percent_change(avg_price, fprice);
        let top_percent_change = action::calculate_percent_change(state.top_price, fprice);
        let bottom_percent_change = action::calculate_percent_change(state.bottom_price, fprice);

        if avg_percent_change > bot.parameters.take_profit_ratio {
            if top_percent_change < bot.parameters.earning_callback {
                let asset = account.get_balance(String::from(&bot.pair.replace("USDC", "")));
                let free = account.adjust_quantity(String::from(&bot.pair), asset.free);
                let transaction = account.market_sell(bot.pair.clone(), free);

                action::create_trade(result::Trade {
                    pair: bot.pair.clone(),
                    cycle: state.cycle,
                    price: transaction.price,
                    qty: transaction.qty,
                    platform: bot.platform.clone(),
                    status: String::from("CLOSE"),
                    timestamp: chrono::offset::Utc::now().timestamp() as u64,
                });

                let capital = account.get_balance(String::from("USDC"));
                action::update_wallet(capital.free);
            }

            if fprice > state.top_price {
                action::update_top_price(state.id, fprice);
            }
        }

        let margin_len = u64::try_from(bot.margin.margin_configuration.len()).unwrap();

        if state.margin_position == 0 {
            if avg_percent_change < bot.margin.margin_configuration[0][0] {
                if bottom_percent_change > bot.margin.margin_configuration[0][1] {
                    let transaction = account.market_buy_using_quote_quantity(
                        bot.pair.clone(),
                        bot.parameters.first_buy_in * bot.margin.margin_configuration[0][2],
                    );

                    action::create_trade(result::Trade {
                        pair: bot.pair.clone(),
                        cycle: state.cycle,
                        price: transaction.price,
                        qty: transaction.qty,
                        platform: bot.platform.clone(),
                        status: String::from("OPEN"),
                        timestamp: chrono::offset::Utc::now().timestamp() as u64,
                    });

                    action::update_margin_position(state.id, 1);

                    let capital = account.get_balance(String::from("USDC"));
                    action::update_wallet(capital.free);
                }

                if fprice < state.bottom_price {
                    action::update_bottom_price(state.id, fprice);
                }
            }
        } else if state.margin_position < margin_len {
            let index = usize::try_from(state.margin_position).unwrap();
            if avg_percent_change < bot.margin.margin_configuration[index][0] {
                if bottom_percent_change > bot.margin.margin_configuration[index][1] {
                    let transaction = account.market_buy_using_quote_quantity(
                        bot.pair.clone(),
                        bot.parameters.first_buy_in * bot.margin.margin_configuration[index][2],
                    );

                    action::create_trade(result::Trade {
                        pair: bot.pair.clone(),
                        cycle: state.cycle,
                        price: transaction.price,
                        qty: transaction.qty,
                        platform: bot.platform.clone(),
                        status: String::from("OPEN"),
                        timestamp: chrono::offset::Utc::now().timestamp() as u64,
                    });

                    action::update_margin_position(state.id, state.margin_position + 1);

                    let capital = account.get_balance(String::from("USDC"));
                    action::update_wallet(capital.free);
                }

                if fprice < state.bottom_price {
                    action::update_bottom_price(state.id, fprice);
                }
            }
        }
    } else {
        let transaction =
            account.market_buy_using_quote_quantity(bot.pair.clone(), bot.parameters.first_buy_in);

        action::create_trade(result::Trade {
            pair: bot.pair.clone(),
            cycle: trade.cycle + 1,
            price: transaction.price,
            qty: transaction.qty,
            platform: bot.platform.clone(),
            status: String::from("OPEN"),
            timestamp: chrono::offset::Utc::now().timestamp() as u64,
        });

        action::create_bot_state(result::BotState {
            id: 0,
            pair: bot.pair.clone(),
            cycle: trade.cycle + 1,
            margin_position: 0,
            top_price: fprice,
            bottom_price: fprice,
            platform: bot.platform.clone(),
            timestamp: chrono::offset::Utc::now().timestamp() as u64,
        });

        let capital = account.get_balance(String::from("USDC"));
        action::update_wallet(capital.free);
    }
}
