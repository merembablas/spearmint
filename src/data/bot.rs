use super::{args, result};
use rusqlite::{params, Connection, Result};

pub fn save(config: args::Config) -> args::Config {
    let conn = Connection::open(super::DB_PATH).unwrap();

    let mut stmt = conn
        .prepare("SELECT id FROM bots WHERE pair=:pair AND platform=:platform LIMIT 1")
        .unwrap();
    let pair = &config.general.pair;
    let platform = &config.general.platform;
    let bots: Vec<Result<u64>> = stmt
        .query_map([pair, platform], |row| Ok(row.get(0)?))
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
                bot.unwrap()
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

pub fn get(name: &str) -> Result<result::Bot> {
    let conn = Connection::open(super::DB_PATH).unwrap();
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

pub fn all() -> Result<Vec<result::Bot>> {
    let conn = Connection::open(super::DB_PATH).unwrap();
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

pub fn active() -> Result<Vec<result::Bot>> {
    let conn = Connection::open(super::DB_PATH).unwrap();
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
