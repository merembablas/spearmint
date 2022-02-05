use dialoguer::{theme::ColorfulTheme, Confirm};
use rusqlite::{params, Connection, Result};

pub mod args;
pub mod result;
pub mod streams;

pub const DB_PATH: &str = "spearmint.db";

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

    conn.execute(
        "CREATE TABLE if not exists platform_api_bindings (
            id                              INTEGER PRIMARY KEY AUTOINCREMENT,
            api_key                         TEXT NOT NULL,
            secret_key                      TEXT,
            platform                        TEXT
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

pub fn list_active() -> Result<Vec<String>> {
    let conn = Connection::open(DB_PATH).unwrap();
    let mut stmt = conn.prepare("SELECT * FROM bots WHERE status='ACTIVE'")?;
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

    let mut result: Vec<String> = Vec::new();
    for bot in bots {
        result.push(bot.unwrap().pair);
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

pub fn save_api_key(api: args::ApiKey) -> args::ApiKey {
    let conn = Connection::open(DB_PATH).unwrap();

    let mut stmt = conn
        .prepare("SELECT * FROM platform_api_bindings WHERE platform=:platform LIMIT 1")
        .unwrap();
    let platform = &api.platform;
    let apis: Vec<Result<args::ApiKey>> = stmt
        .query_map([platform], |row| {
            Ok(args::ApiKey {
                id: row.get(0)?,
                api_key: row.get(1)?,
                secret_key: row.get(2)?,
                platform: row.get(3)?,
            })
        })
        .unwrap()
        .collect();

    if apis.iter().count() != 0 {
        let curr_api = apis.into_iter().next().unwrap();
        conn.execute(
            "UPDATE platform_api_bindings SET
                api_key=?1,
                secret_key=?2
            WHERE id=?3",
            params![api.api_key, api.secret_key, curr_api.unwrap().id],
        )
        .unwrap();
    } else {
        conn.execute(
            "INSERT INTO platform_api_bindings (
                api_key,
                secret_key,
                platform
            ) VALUES (?1, ?2, ?3)",
            params![api.api_key, api.secret_key, api.platform],
        )
        .unwrap();
    }

    api
}
