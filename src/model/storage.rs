use super::result;
use rusqlite::{params, Connection, Result};

pub const DB_PATH: &str = "spearmint.db";
pub const DB_DATA_PATH: &str = "spearmint_data.db";

pub fn get_latest_trade(platform: &str, pair: &str) -> Result<result::Trade> {
    let conn = Connection::open(DB_PATH).unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT * FROM trades WHERE platform=:platform AND pair=:pair ORDER BY id DESC LIMIT 1",
        )
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

pub fn get_latest_pnl_trade(platform: &str, pair: &str) -> Result<result::PnL> {
    let conn = Connection::open(DB_PATH).unwrap();
    let mut stmt = conn
    .prepare("SELECT cycle FROM trades WHERE platform=:platform AND pair=:pair AND status=:status ORDER BY timestamp DESC LIMIT 1")
    .unwrap();
    let mut trades: Vec<Result<u64>> = stmt
        .query_map([platform, pair, "CLOSE"], |row| Ok(row.get(0)?))
        .unwrap()
        .collect();

    if trades.len() > 0 {
        let cycle = trades.remove(0).unwrap();
        let mut stmt = conn
    .prepare("SELECT * FROM trades WHERE platform=:platform AND pair=:pair AND cycle=:cycle ORDER BY timestamp DESC")
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

        let mut pnl: f64 = 0.0;
        for trade in trades {
            let tr = trade.unwrap();
            if tr.status == "CLOSE" {
                pnl = pnl + (tr.price * tr.qty);
            } else if tr.status == "OPEN" {
                pnl = pnl - (tr.price * tr.qty);
            }
        }

        Ok(result::PnL {
            pair: String::from(pair),
            cycle: cycle,
            pnl: pnl,
        })
    } else {
        Ok(Default::default())
    }
}

pub fn create_trade(trade: result::Trade) {
    let conn = Connection::open(DB_PATH).unwrap();
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

pub fn get_avg_price(platform: &str, pair: &str, cycle: u64) -> f64 {
    let conn = Connection::open(DB_PATH).unwrap();
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

    let mut total_amount: f64 = 0.0;
    let mut total_qty: f64 = 0.0;
    for trade in trades {
        let item = trade.unwrap();
        total_amount += item.price * item.qty;
        total_qty += item.qty;
    }

    total_amount / total_qty
}

pub fn get_latest_state(platform: &str, pair: &str) -> Result<result::BotState> {
    let conn = Connection::open(DB_PATH).unwrap();
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
                bottom_price: row.get(5)?,
                bottom_mfi: row.get(6)?,
                platform: row.get(7)?,
                timestamp: row.get(8)?,
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
    let conn = Connection::open(DB_PATH).unwrap();
    conn.execute(
        "INSERT INTO bot_states (
        pair,
        cycle,
        margin_position,
        top_price,
        bottom_price,
        bottom_mfi,
        platform,
        timestamp
    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            state.pair,
            state.cycle,
            state.margin_position,
            state.top_price,
            state.bottom_price,
            state.bottom_mfi,
            state.platform,
            state.timestamp
        ],
    )
    .unwrap();
}

pub fn update_top_price(id: u64, top_price: f64) {
    let conn = Connection::open(DB_PATH).unwrap();
    conn.execute(
        "UPDATE bot_states SET
            top_price=?1
        WHERE id=?2",
        params![top_price, id],
    )
    .unwrap();
}

pub fn update_bottom_price(id: u64, bottom_price: f64) {
    let conn = Connection::open(DB_PATH).unwrap();
    conn.execute(
        "UPDATE bot_states SET
            bottom_price=?1
        WHERE id=?2",
        params![bottom_price, id],
    )
    .unwrap();
}

pub fn update_bottom_mfi(id: u64, bottom_mfi: f64) {
    let conn = Connection::open(DB_PATH).unwrap();
    conn.execute(
        "UPDATE bot_states SET
            bottom_mfi=?1
        WHERE id=?2",
        params![bottom_mfi, id],
    )
    .unwrap();
}

pub fn update_margin_position(id: u64, position: u64) {
    let conn = Connection::open(DB_PATH).unwrap();
    conn.execute(
        "UPDATE bot_states SET
            margin_position=?1
        WHERE id=?2",
        params![position, id],
    )
    .unwrap();
}

pub fn get_wallet(quote: &str) -> f64 {
    let conn = Connection::open(DB_PATH).unwrap();
    let mut stmt = conn
        .prepare("SELECT amount FROM tokens WHERE token=:token LIMIT 1")
        .unwrap();
    let mut tokens: Vec<Result<f64>> = stmt
        .query_map([quote], |row| Ok(row.get(0)?))
        .unwrap()
        .collect();

    tokens.remove(0).unwrap()
}

pub fn update_wallet(quote: &str, amount: f64) {
    let conn = Connection::open(DB_PATH).unwrap();
    conn.execute(
        "UPDATE tokens SET
            amount=?1
        WHERE token=?2",
        params![amount, quote],
    )
    .unwrap();
}

pub fn create_ticker(path: &str, ticker: result::Ticker) {
    let conn = Connection::open(path).unwrap();
    conn.execute(
        "INSERT INTO tickers (
        pair,
        timestamp,
        open,
        high,
        low,
        close,
        volume,
        mfi
    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            ticker.pair,
            chrono::offset::Utc::now().timestamp() as u64,
            ticker.open,
            ticker.high,
            ticker.low,
            ticker.close,
            ticker.volume,
            ticker.mfi
        ],
    )
    .unwrap();
}

pub fn get_latest_mfi(pair: &str) -> [f64; 2] {
    let conn = Connection::open(DB_DATA_PATH).unwrap();
    let mut stmt = conn
        .prepare("SELECT mfi FROM tickers WHERE pair=:pair ORDER BY timestamp DESC LIMIT 2")
        .unwrap();
    let mut tickers: Vec<Result<f64>> = stmt
        .query_map([pair], |row| Ok(row.get(0)?))
        .unwrap()
        .collect();

    let mut ticks = [0.0, 0.0];
    if tickers.len() > 1 {
        ticks[0] = tickers.remove(0).unwrap();
        ticks[1] = tickers.remove(0).unwrap();
    }

    ticks
}

pub fn get_tickers(pair: &str, limit: u64) -> Vec<Result<result::Ticker>> {
    let conn = Connection::open(DB_DATA_PATH).unwrap();
    let mut stmt = conn
        .prepare("SELECT * FROM tickers WHERE pair=:pair ORDER BY timestamp DESC LIMIT :limit")
        .unwrap();
    let tickers: Vec<Result<result::Ticker>> = stmt
        .query_map([pair, &limit.to_string()], |row| {
            Ok(result::Ticker {
                pair: row.get(1)?,
                open: row.get(3)?,
                high: row.get(4)?,
                low: row.get(5)?,
                close: row.get(6)?,
                volume: row.get(7)?,
                mfi: row.get(8)?,
            })
        })
        .unwrap()
        .collect();

    tickers
}
