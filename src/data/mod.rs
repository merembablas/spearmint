use dialoguer::{theme::ColorfulTheme, Confirm};
use rusqlite::{params, Connection, Result};

pub mod action;
pub mod args;
pub mod bind;
pub mod bot;
pub mod result;
pub mod streams;

pub const DB_PATH: &str = "spearmint.db";

pub fn setup(path: &str) -> Result<()> {
    let conn = Connection::open(path)?;

    conn.execute(
        "CREATE TABLE if not exists bots (
            id                              INTEGER PRIMARY KEY AUTOINCREMENT,
            title                           TEXT NOT NULL,
            pair                            TEXT,
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
        ["USDC", "1000", "binance"],
    )?;

    conn.execute(
        "CREATE TABLE if not exists bot_states (
            id                              INTEGER PRIMARY KEY AUTOINCREMENT,
            pair                            TEXT NOT NULL,
            cycle                           INTEGER,
            margin_position                 INTEGER,
            top_price                       REAL,
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
