use dialoguer::{theme::ColorfulTheme, Confirm};
use rusqlite::{params, Connection, Result};
use std::marker::PhantomData;

pub mod args;
pub mod bind;
pub mod bot;
pub mod result;
pub mod storage;

pub const DB_PATH: &str = "spearmint.db";

pub fn setup(path: &str) -> Result<()> {
    let conn = Connection::open(path)?;

    conn.execute(
        "CREATE TABLE if not exists bots (
            id                              INTEGER PRIMARY KEY AUTOINCREMENT,
            title                           TEXT NOT NULL,
            pair                            TEXT,
            base                            TEXT,
            quote                           TEXT,
            platform                        TEXT,
            strategy                        TEXT,
            cycle                           TEXT,
            first_buy_in                    REAL,
            take_profit_ratio               REAL,
            earning_callback                REAL,
            margin                          TEXT,
            status                          TEXT
        )
    ",
        [],
    )?;

    conn.execute(
        "CREATE TABLE if not exists bindings (
            id                              INTEGER PRIMARY KEY AUTOINCREMENT,
            api_key                         TEXT NOT NULL,
            secret_key                      TEXT,
            platform                        TEXT
        )
    ",
        [],
    )?;

    conn.execute(
        "CREATE TABLE if not exists trades (
            id                              INTEGER PRIMARY KEY AUTOINCREMENT,
            pair                            TEXT NOT NULL,
            cycle                           INTEGER,
            price                           REAL,
            qty                             REAL,
            platform                        TEXT,
            status                          TEXT,
            timestamp                       INTEGER NOT NULL
        );

        CREATE INDEX pair_idx ON trades (platform, pair);
        CREATE INDEX timestamp_idx ON trades (timestamp) DESC;
    ",
        [],
    )?;

    conn.execute(
        "CREATE TABLE if not exists tokens (
            id                              INTEGER PRIMARY KEY AUTOINCREMENT,
            token                           TEXT NOT NULL,
            amount                          REAL,
            platform                        TEXT
        )
    ",
        [],
    )?;

    conn.execute(
        "INSERT INTO tokens (token, amount, platform) VALUES (?1, ?2, ?3)
    ",
        ["USDC", "0", "binance"],
    )?;

    conn.execute(
        "INSERT INTO tokens (token, amount, platform) VALUES (?1, ?2, ?3)
    ",
        ["USDT", "0", "binance"],
    )?;

    conn.execute(
        "INSERT INTO tokens (token, amount, platform) VALUES (?1, ?2, ?3)
    ",
        ["BTC", "0", "binance"],
    )?;

    conn.execute(
        "CREATE TABLE if not exists bot_states (
            id                              INTEGER PRIMARY KEY AUTOINCREMENT,
            pair                            TEXT NOT NULL,
            cycle                           INTEGER,
            margin_position                 INTEGER,
            top_price                       REAL,
            bottom_price                    REAL,
            platform                        TEXT,
            timestamp                       INTEGER NOT NULL
        );

        CREATE INDEX pair_idx ON bot_states (platform, pair);
        CREATE INDEX timestamp_idx ON bot_states (timestamp) DESC;
    ",
        [],
    )?;

    Ok(())
}

pub fn delete(name: &str) {
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you really want to continue?")
        .default(true)
        .interact()
        .unwrap()
    {
        let conn = Connection::open(DB_PATH).unwrap();
        conn.execute("DELETE FROM bots WHERE title=?1", params![name])
            .unwrap();
        println!("{} deleted!", name);
    } else {
        println!("Ok, nevermind then");
    }
}

pub fn start(name: &str) {
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you really want to continue?")
        .default(true)
        .interact()
        .unwrap()
    {
        let conn = Connection::open(DB_PATH).unwrap();
        conn.execute(
            "UPDATE bots SET status='ACTIVE' WHERE title=?1",
            params![name],
        )
        .unwrap();
        println!("{} activated!", name);
    } else {
        println!("Ok, nevermind then");
    }
}

pub fn stop(name: &str) {
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you really want to continue?")
        .default(true)
        .interact()
        .unwrap()
    {
        let conn = Connection::open(DB_PATH).unwrap();
        conn.execute(
            "UPDATE bots SET status='PAUSED' WHERE title=?1",
            params![name],
        )
        .unwrap();
        println!("{} paused!", name);
    } else {
        println!("Ok, nevermind then");
    }
}

pub trait Exchange {
    fn get_balances(&self) -> Vec<result::Balance>;
    fn market_buy_using_quote_quantity(&self, pair: String, qty: f64) -> result::Transaction;
    fn market_sell(&self, pair: String, qty: f64) -> result::Transaction;
    fn get_balance(&self, asset: String) -> result::Balance;
    fn adjust_quantity(&self, pair: String, qty: f64) -> f64;
}

pub trait Strategy {
    fn run(&self, price: f64, session: Session) -> BotCommand;
    fn is_entry_signal(&self, top_percent_change: f64, bottom_percent_change: f64) -> bool;
    fn is_sell_signal(&self, top_percent_change: f64, avg_percent_change: f64) -> bool;
    fn is_avg_buy_signal(
        &self,
        avg_percent_change: f64,
        bottom_percent_change: f64,
        margin_position: usize,
    ) -> bool;
}

#[derive(Debug)]
pub enum BotCommand {
    Entry(f64),
    Buy(f64),
    Sell(),
    Pause(),
}

pub trait SessionState {}

pub struct Initial;

impl SessionState for Initial {}

pub struct Session<State: SessionState = Initial> {
    pub status: String,
    pub avg_price: f64,
    pub top_price: f64,
    pub bottom_price: f64,
    pub margin_position: u64,
    pub phantom: PhantomData<State>,
}
