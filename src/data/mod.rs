use dialoguer::{theme::ColorfulTheme, Confirm};
use rusqlite::{params, Connection, Result};

pub mod action;
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
        ["USDT", "1000", "binance"],
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

pub fn save(config: args::Config) -> args::Config {
    let conn = Connection::open(DB_PATH).unwrap();

    let mut stmt = conn
        .prepare("SELECT id FROM bots WHERE pair=:pair AND platform=:platform LIMIT 1")
        .unwrap();
    let pair = &config.general.pair;
    let platform = &config.general.platform;
    let bots: Vec<Result<BotId>> = stmt
        .query_map([pair, platform], |row| Ok(BotId { id: row.get(0)? }))
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
                earning_callback=?6,
                margin=?7
            WHERE id=?8",
            params![
                config.title,
                config.general.strategy,
                config.parameters.cycle,
                config.parameters.first_buy_in,
                config.parameters.take_profit_ratio,
                config.parameters.earning_callback,
                serde_json::to_string(&config.margin.margin_configuration).unwrap(),
                bot.unwrap().id
            ],
        )
        .unwrap();
    } else {
        conn.execute(
            "INSERT INTO bots (
                title,
                pair,
                platform,
                strategy,
                cycle,
                first_buy_in,
                take_profit_ratio,
                earning_callback,
                margin,
                status
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 'PAUSED')",
            params![
                config.title,
                config.general.pair,
                config.general.platform,
                config.general.strategy,
                config.parameters.cycle,
                config.parameters.first_buy_in,
                config.parameters.take_profit_ratio,
                config.parameters.earning_callback,
                serde_json::to_string(&config.margin.margin_configuration).unwrap()
            ],
        )
        .unwrap();
    }

    config
}

pub fn get_bot(name: &str) -> Result<result::Bot> {
    let conn = Connection::open(DB_PATH).unwrap();
    let mut stmt = conn
        .prepare("SELECT * FROM bots WHERE title=:name LIMIT 1")
        .unwrap();
    let mut bots: Vec<Result<result::Bot>> = stmt
        .query_map([name], |row| {
            let margin_configuration: String = row.get(9)?;

            Ok(result::Bot {
                title: row.get(1)?,
                pair: row.get(2)?,
                platform: row.get(3)?,
                strategy: row.get(4)?,
                parameters: result::Parameters {
                    cycle: row.get(5)?,
                    first_buy_in: row.get(6)?,
                    take_profit_ratio: row.get(7)?,
                    earning_callback: row.get(8)?,
                },
                margin: result::Margin {
                    margin_configuration: serde_json::from_str(&margin_configuration).unwrap(),
                },
                status: row.get(10)?,
            })
        })
        .unwrap()
        .collect();

    bots.remove(0)
}

pub fn list() -> Result<Vec<result::Bot>> {
    let conn = Connection::open(DB_PATH).unwrap();
    let mut stmt = conn.prepare("SELECT * FROM bots")?;
    let bots = stmt.query_map([], |row| {
        let margin_configuration: String = row.get(9)?;

        Ok(result::Bot {
            title: row.get(1)?,
            pair: row.get(2)?,
            platform: row.get(3)?,
            strategy: row.get(4)?,
            parameters: result::Parameters {
                cycle: row.get(5)?,
                first_buy_in: row.get(6)?,
                take_profit_ratio: row.get(7)?,
                earning_callback: row.get(8)?,
            },
            margin: result::Margin {
                margin_configuration: serde_json::from_str(&margin_configuration).unwrap(),
            },
            status: row.get(10)?,
        })
    })?;

    let mut result: Vec<result::Bot> = Vec::new();
    for bot in bots {
        result.push(bot.unwrap());
    }

    Ok(result)
}

pub fn list_active() -> Result<Vec<result::Bot>> {
    let conn = Connection::open(DB_PATH).unwrap();
    let mut stmt = conn.prepare("SELECT * FROM bots WHERE status='ACTIVE'")?;
    let bots = stmt.query_map([], |row| {
        let margin_configuration: String = row.get(9)?;

        Ok(result::Bot {
            title: row.get(1)?,
            pair: row.get(2)?,
            platform: row.get(3)?,
            strategy: row.get(4)?,
            parameters: result::Parameters {
                cycle: row.get(5)?,
                first_buy_in: row.get(6)?,
                take_profit_ratio: row.get(7)?,
                earning_callback: row.get(8)?,
            },
            margin: result::Margin {
                margin_configuration: serde_json::from_str(&margin_configuration).unwrap(),
            },
            status: row.get(10)?,
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

pub fn save_api_key(api: args::ApiKey) -> args::ApiKey {
    let conn = Connection::open(DB_PATH).unwrap();

    let mut stmt = conn
        .prepare("SELECT * FROM bindings WHERE platform=:platform LIMIT 1")
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
            "UPDATE bindings SET
                api_key=?1,
                secret_key=?2
            WHERE id=?3",
            params![api.api_key, api.secret_key, curr_api.unwrap().id],
        )
        .unwrap();
    } else {
        conn.execute(
            "INSERT INTO bindings (
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
