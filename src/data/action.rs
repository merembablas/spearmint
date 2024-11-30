use crate::data;
use crate::data::result;
use chrono;
use rusqlite::{params, Connection, Result};

pub fn execute_strategy(bot: &result::Bot, price: &str) {
    let trade = match get_latest_trade(&bot.platform, &bot.pair) {
        Ok(trade) => trade,
        Err(_error) => Default::default(),
    };

    let fprice = price.parse::<f32>().unwrap();

    if trade.status == "OPEN" {
        let state = match get_latest_state(&bot.platform, &bot.pair) {
            Ok(state) => state,
            Err(_error) => Default::default(),
        };

        if state.id == 0 {
            return;
        }

        let avg_price = get_avg_price(&bot.platform, &bot.pair, state.cycle);
        let avg_percent_change = calculate_percent_change(avg_price, fprice);
        let top_percent_change = calculate_percent_change(state.top_price, fprice);

        if avg_percent_change > bot.parameters.take_profit_ratio {
            if top_percent_change < bot.parameters.earning_callback {
                create_trade(result::Trade {
                    pair: bot.pair.clone(),
                    cycle: state.cycle,
                    price: fprice,
                    qty: trade.qty,
                    platform: bot.platform.clone(),
                    status: String::from("CLOSE"),
                    timestamp: chrono::offset::Utc::now().timestamp(),
                });

                let capital = get_wallet();
                update_wallet(capital + (trade.qty * fprice));
            }

            if fprice > state.top_price {
                update_top_price(state.id, fprice);
            }
        }

        let margin_len = u64::try_from(bot.margin.margin_configuration.len()).unwrap();

        if state.margin_position == 0 {
            if avg_percent_change < bot.margin.margin_configuration[0][0] {
                let qty =
                    bot.parameters.first_buy_in * bot.margin.margin_configuration[0][1] / fprice;
                create_trade(result::Trade {
                    pair: bot.pair.clone(),
                    cycle: state.cycle,
                    price: fprice,
                    qty: qty,
                    platform: bot.platform.clone(),
                    status: String::from("OPEN"),
                    timestamp: chrono::offset::Utc::now().timestamp(),
                });

                update_margin_position(state.id, 1);

                let capital = get_wallet();
                update_wallet(
                    capital - (bot.parameters.first_buy_in * bot.margin.margin_configuration[0][1]),
                );
            }
        } else if state.margin_position < margin_len {
            let index = usize::try_from(state.margin_position).unwrap();
            if avg_percent_change < bot.margin.margin_configuration[index][0] {
                let qty = bot.parameters.first_buy_in * bot.margin.margin_configuration[index][1]
                    / fprice;
                create_trade(result::Trade {
                    pair: bot.pair.clone(),
                    cycle: state.cycle,
                    price: fprice,
                    qty: qty,
                    platform: bot.platform.clone(),
                    status: String::from("OPEN"),
                    timestamp: chrono::offset::Utc::now().timestamp(),
                });

                update_margin_position(state.id, state.margin_position + 1);

                let capital = get_wallet();
                update_wallet(
                    capital
                        - (bot.parameters.first_buy_in * bot.margin.margin_configuration[index][1]),
                );
            }
        }
    } else {
        create_trade(result::Trade {
            pair: bot.pair.clone(),
            cycle: trade.cycle + 1,
            price: fprice,
            qty: bot.parameters.first_buy_in / fprice,
            platform: bot.platform.clone(),
            status: String::from("OPEN"),
            timestamp: chrono::offset::Utc::now().timestamp(),
        });

        create_bot_state(result::BotState {
            id: 0,
            pair: bot.pair.clone(),
            cycle: trade.cycle + 1,
            margin_position: 0,
            top_price: fprice,
            platform: bot.platform.clone(),
            timestamp: chrono::offset::Utc::now().timestamp(),
        });

        let capital = get_wallet();
        update_wallet(capital - bot.parameters.first_buy_in);
    }
}

pub fn calculate_percent_change(old_value: f32, new_value: f32) -> f32 {
    if old_value == 0.0 {
        return f32::INFINITY;
    }

    ((new_value - old_value) / old_value) * 100.0
}

pub fn get_latest_trade(platform: &str, pair: &str) -> Result<result::Trade> {
    let conn = Connection::open(data::DB_PATH).unwrap();
    let mut stmt = conn
        .prepare("SELECT * FROM trades WHERE platform=:platform AND pair=:pair ORDER BY timestamp DESC LIMIT 1")
        .unwrap();
    let mut trades: Vec<Result<result::Trade>> = stmt
        .query_map([platform, pair], |row| {
            Ok(result::Trade {
                pair: row.get(1)?,
                cycle: row.get(2)?,
                price: row.get(3)?,
                qty: row.get(4)?,
                platform: row.get(5)?,
                status: row.get(6)?,
                timestamp: row.get(7)?,
            })
        })
        .unwrap()
        .collect();

    if trades.len() > 0 {
        trades.remove(0)
    } else {
        Ok(Default::default())
    }
}

pub fn create_trade(trade: result::Trade) {
    let conn = Connection::open(data::DB_PATH).unwrap();
    conn.execute(
        "INSERT INTO trades (
            pair,
            cycle,
            price,
            qty,
            platform,
            status,
            timestamp
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            trade.pair,
            trade.cycle,
            trade.price,
            trade.qty,
            trade.platform,
            trade.status,
            trade.timestamp
        ],
    )
    .unwrap();
}

pub fn get_avg_price(platform: &str, pair: &str, cycle: u64) -> f32 {
    let conn = Connection::open(data::DB_PATH).unwrap();
    let mut stmt = conn
        .prepare("SELECT * FROM trades WHERE platform=:platform AND pair=:pair AND cycle=:cycle")
        .unwrap();
    let trades: Vec<Result<result::Trade>> = stmt
        .query_map([platform, pair, &cycle.to_string()], |row| {
            Ok(result::Trade {
                pair: row.get(1)?,
                cycle: row.get(2)?,
                price: row.get(3)?,
                qty: row.get(4)?,
                platform: row.get(5)?,
                status: row.get(6)?,
                timestamp: row.get(7)?,
            })
        })
        .unwrap()
        .collect();

    let mut total_amount: f32 = 0.0;
    let mut total_qty: f32 = 0.0;
    for trade in trades {
        let item = trade.unwrap();
        total_amount += item.price * item.qty;
        total_qty += item.qty;
    }

    total_amount / total_qty
}

pub fn get_latest_state(platform: &str, pair: &str) -> Result<result::BotState> {
    let conn = Connection::open(data::DB_PATH).unwrap();
    let mut stmt = conn
        .prepare("SELECT * FROM bot_states WHERE platform=:platform AND pair=:pair ORDER BY timestamp DESC LIMIT 1")
        .unwrap();
    let mut states: Vec<Result<result::BotState>> = stmt
        .query_map([platform, pair], |row| {
            Ok(result::BotState {
                id: row.get(0)?,
                pair: row.get(1)?,
                cycle: row.get(2)?,
                margin_position: row.get(3)?,
                top_price: row.get(4)?,
                platform: row.get(5)?,
                timestamp: row.get(6)?,
            })
        })
        .unwrap()
        .collect();

    if states.len() > 0 {
        states.remove(0)
    } else {
        Ok(Default::default())
    }
}

pub fn create_bot_state(state: result::BotState) {
    let conn = Connection::open(data::DB_PATH).unwrap();
    conn.execute(
        "INSERT INTO bot_states (
            pair,
            cycle,
            margin_position,
            top_price,
            platform,
            timestamp
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            state.pair,
            state.cycle,
            state.margin_position,
            state.top_price,
            state.platform,
            state.timestamp
        ],
    )
    .unwrap();
}

pub fn update_top_price(id: u64, top_price: f32) {
    let conn = Connection::open(data::DB_PATH).unwrap();
    conn.execute(
        "UPDATE bot_states SET
                top_price=?1
            WHERE id=?2",
        params![top_price, id],
    )
    .unwrap();
}

pub fn update_margin_position(id: u64, position: u64) {
    let conn = Connection::open(data::DB_PATH).unwrap();
    conn.execute(
        "UPDATE bot_states SET
                margin_position=?1
            WHERE id=?2",
        params![position, id],
    )
    .unwrap();
}

pub fn get_wallet() -> f32 {
    let conn = Connection::open(data::DB_PATH).unwrap();
    let mut stmt = conn
        .prepare("SELECT amount FROM tokens WHERE token=:token LIMIT 1")
        .unwrap();
    let mut tokens: Vec<Result<f32>> = stmt
        .query_map(["USDT"], |row| Ok(row.get(0)?))
        .unwrap()
        .collect();

    tokens.remove(0).unwrap()
}

pub fn update_wallet(amount: f32) {
    let conn = Connection::open(data::DB_PATH).unwrap();
    conn.execute(
        "UPDATE tokens SET
                amount=?1
            WHERE token=?2",
        params![amount, "USDT"],
    )
    .unwrap();
}