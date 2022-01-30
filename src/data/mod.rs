use dialoguer::{theme::ColorfulTheme, Confirm};
use rusqlite::{params, Connection, Result};

pub mod args;
pub mod result;

const DB_PATH: &str = "spearmint.db";

struct BotId {
    id: u64,
}

pub fn setup(path: &str) -> Result<()> {
    let conn = Connection::open(path)?;

    conn.execute(
        "CREATE TABLE if not exists bots (
            id                              INTEGER PRIMARY KEY AUTOINCREMENT,
            title                           TEXT NOT NULL,
            pair                            TEXT,
            exchange                        TEXT,
            strategy                        TEXT,
            cycle                           TEXT,
            first_buy_in                    REAL,
            take_profit_ratio               REAL,
            margin_call_limit               INTEGER,
            earning_callback                REAL,
            sub_position_callback           REAL,
            sub_position_earning_callback   REAL,
            margin                          TEXT,
            sub_position                    TEXT,
            status                          TEXT
        )
    ",
        [],
    )?;

    Ok(())
}

pub fn save(config: args::Config) -> args::Config {
    let conn = Connection::open(DB_PATH).unwrap();

    let mut stmt = conn
        .prepare("SELECT id FROM bots WHERE pair=:pair AND exchange=:exchange LIMIT 1")
        .unwrap();
    let pair = &config.general.pair;
    let exchange = &config.general.exchange;
    let bots: Vec<Result<BotId>> = stmt
        .query_map([pair, exchange], |row| Ok(BotId { id: row.get(0)? }))
        .unwrap()
        .collect();

    if bots.iter().count() != 0 {
        let bot = bots.into_iter().next().unwrap();
        conn.execute(
            "UPDATE bots SET
                title=?1,
                strategy=?2,
                cycle=?3,
                first_buy_in=?4,
                take_profit_ratio=?5,
                margin_call_limit=?6,
                earning_callback=?7,
                sub_position_callback=?8,
                sub_position_earning_callback=?9,
                margin=?10,
                sub_position=?11
            WHERE id=?12",
            params![
                config.title,
                config.general.strategy,
                config.parameters.cycle,
                config.parameters.first_buy_in,
                config.parameters.take_profit_ratio,
                config.parameters.margin_call_limit,
                config.parameters.earning_callback,
                config.parameters.sub_position_start,
                config.margin.sub_position_earning_callback,
                serde_json::to_string(&config.margin.margin_configuration).unwrap(),
                serde_json::to_string(&config.margin.sub_position_profit_ratio).unwrap(),
                bot.unwrap().id
            ],
        )
        .unwrap();
    } else {
        conn.execute(
            "INSERT INTO bots (
                title,
                pair,
                exchange,
                strategy,
                cycle,
                first_buy_in,
                take_profit_ratio,
                margin_call_limit,
                earning_callback,
                sub_position_callback,
                sub_position_earning_callback,
                margin,
                sub_position,
                status
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, 'PAUSED')",
            params![
                config.title,
                config.general.pair,
                config.general.exchange,
                config.general.strategy,
                config.parameters.cycle,
                config.parameters.first_buy_in,
                config.parameters.take_profit_ratio,
                config.parameters.margin_call_limit,
                config.parameters.earning_callback,
                config.parameters.sub_position_start,
                config.margin.sub_position_earning_callback,
                serde_json::to_string(&config.margin.margin_configuration).unwrap(),
                serde_json::to_string(&config.margin.sub_position_profit_ratio).unwrap()
            ],
        )
        .unwrap();
    }

    config
}

pub fn list() -> Result<Vec<result::Bot>> {
    let conn = Connection::open(DB_PATH).unwrap();
    let mut stmt = conn.prepare("SELECT * FROM bots")?;
    let bots = stmt.query_map([], |row| {
        Ok(result::Bot {
            title: row.get(1)?,
            pair: row.get(2)?,
            exchange: row.get(3)?,
            strategy: row.get(4)?,
            cycle: row.get(5)?,
            status: row.get(14)?,
        })
    })?;

    let mut result: Vec<result::Bot> = Vec::new();
    for bot in bots {
        result.push(bot.unwrap());
    }

    Ok(result)
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
